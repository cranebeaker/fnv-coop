//! Direct3D 9 on wgpu (ADR-0007): the part of Direct3D 9 the game uses, as a backend of the
//! recorded call stream (`d3d9-stream`), and the converter of the game's shader bytecode.
//!
//! The game's own renderer is translated from the exe and calls Direct3D 9 through the device's
//! interface; the platform layer's model answers every call and records the ones that change what
//! is drawn (`measure/render-thread/DESIGN.md`, "The seam"). This crate carries those out on wgpu
//! (Vulkan, Direct3D 12, Metal): [`WgpuBackend`] is a [`d3d9_stream::Backend`]. It reads no game
//! memory and keeps no answer the game reads, apart from the sync commands' replies.
//!
//! - `sm`: shader bytecode read, listed and converted to WGSL (`sm::wgsl`).
//! - `format`: `D3DFORMAT` to wgpu formats.
//! - `state`: render and sampler states to wgpu states.
//! - `device`: resources, passes, pipelines, draws and `Present`.
//! - `shot`: `NV_SHOT` captures numbered like the forwarding layer's.

pub mod device;
pub mod format;
pub mod shot;
pub mod sm;
pub mod state;

use d3d9_stream::{Backend, Cmd, Data, Handle, Reply};
pub use device::Output;

/// The Direct3D 9 calls carried out on wgpu.
pub struct WgpuBackend {
    output: Output,
    device: Option<device::Device>,
    /// The handle of the game's device (one is made).
    game_device: Handle,
}

impl WgpuBackend {
    /// A backend whose frames go to the game's window, or nowhere but captures
    /// ([`Output::Offscreen`], the shadow of the forwarding to the real `d3d9.dll`).
    pub fn new(output: Output) -> Self {
        WgpuBackend {
            output,
            device: None,
            game_device: 0,
        }
    }

    /// The wgpu device, once the game made its own.
    pub fn device(&mut self) -> Option<&mut device::Device> {
        self.device.as_mut()
    }
}

fn handles(d: &Data<'_>) -> Vec<Handle> {
    d.to_words().into_owned()
}

fn rect_words(d: &Data<'_>) -> Vec<i32> {
    d.to_words().iter().map(|&w| w as i32).collect()
}

/// A rectangle a command carries (left, top, right, bottom).
fn rect(d: &Option<Data<'_>>) -> Option<[i32; 4]> {
    let w = rect_words(d.as_ref()?);
    (w.len() == 4).then(|| [w[0], w[1], w[2], w[3]])
}

impl Backend for WgpuBackend {
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        if let Cmd::CreateDevice {
            device,
            focus,
            window,
            present,
            back_buffers,
            depth_stencil,
            ..
        } = &cmd
        {
            let pp = device::PresentParameters::from_words(&present.to_words());
            return match device::Device::new(
                self.output,
                if *window != 0 { *window } else { *focus },
                &pp,
                &handles(back_buffers),
                *depth_stencil,
            ) {
                Ok(d) => {
                    self.device = Some(d);
                    self.game_device = *device;
                    // No D3DX side channel: the texture loader cannot use a wgpu device.
                    Reply::Device {
                        created: true,
                        native: None,
                    }
                }
                Err(e) => {
                    eprintln!("NV_D3D9 wgpu: CreateDevice: {e}");
                    Reply::Device {
                        created: false,
                        native: None,
                    }
                }
            };
        }
        let Some(d) = self.device.as_mut() else {
            return match cmd {
                Cmd::ReadSurface { .. } => Reply::Bytes(None),
                Cmd::QueryGetData { .. } => Reply::Query(None),
                Cmd::GetRenderTargetData { .. } => Reply::Rows(None),
                _ => Reply::None,
            };
        };
        match cmd {
            Cmd::CreateDevice { .. } => unreachable!("handled above"),
            Cmd::Reset {
                present,
                back_buffers,
                depth_stencil,
                ..
            } => {
                let pp = device::PresentParameters::from_words(&present.to_words());
                d.reset(&pp, &handles(&back_buffers), depth_stencil);
            }
            Cmd::Present { frame, .. } => d.present(frame),
            Cmd::BeginScene { .. } | Cmd::EndScene { .. } => {}
            Cmd::Clear {
                rects,
                flags,
                colour,
                z,
                stencil,
                ..
            } => {
                let rects = rects.as_ref().map(rect_words);
                d.clear(rects.as_deref(), flags, colour, f32::from_bits(z), stencil);
            }
            Cmd::SetRenderTarget { index, surface, .. } => d.set_render_target(index, surface),
            Cmd::SetDepthStencilSurface { surface, .. } => d.set_depth_stencil(surface),
            Cmd::SetViewport { viewport, .. } => d.set_viewport(viewport),
            Cmd::SetScissorRect { rect, .. } => d.set_scissor(rect),
            Cmd::SetRenderState { state, value, .. } => d.set_render_state(state, value),
            Cmd::SetSamplerState {
                sampler,
                kind,
                value,
                ..
            } => d.set_sampler_state(sampler, kind, value),
            Cmd::SetTexture {
                sampler, texture, ..
            } => d.set_texture(sampler, texture),
            Cmd::SetVertexDeclaration { declaration, .. } => d.set_declaration(declaration),
            Cmd::SetShader { pixel, shader, .. } => d.set_shader(pixel, shader),
            Cmd::SetConstants {
                kind,
                start,
                count,
                data,
                ..
            } => d.set_constants(kind, start, count, &data.to_words()),
            Cmd::SetStreamSource {
                stream,
                buffer,
                offset,
                stride,
                ..
            } => d.set_stream(stream, buffer, offset, stride),
            Cmd::SetIndices { buffer, .. } => d.set_indices(buffer),
            Cmd::DrawPrimitive {
                kind, start, count, ..
            } => d.draw(kind, false, 0, start, count, None),
            Cmd::DrawIndexedPrimitive {
                kind,
                base_vertex,
                start_index,
                count,
                ..
            } => d.draw(kind, true, base_vertex, start_index, count, None),
            Cmd::DrawPrimitiveUp {
                kind,
                count,
                stride,
                vertices,
                ..
            } => {
                let bytes = vertices.bytes();
                d.draw(kind, false, 0, 0, count, Some((&bytes[..], stride)));
            }
            Cmd::UpdateTexture {
                source,
                destination,
                ..
            } => d.update_texture(source, destination),
            Cmd::StretchRect {
                source,
                source_rect,
                destination,
                destination_rect,
                filter,
                ..
            } => d.stretch_rect(
                source,
                rect(&source_rect),
                destination,
                rect(&destination_rect),
                filter,
            ),
            Cmd::CreateTexture {
                texture,
                levels,
                width,
                height,
                usage,
                format,
                ..
            } => d.create_texture(
                texture,
                &handles(&levels),
                width,
                height,
                usage,
                format,
                false,
            ),
            Cmd::CreateCubeTexture {
                texture,
                surfaces,
                edge,
                usage,
                format,
                ..
            } => d.create_texture(
                texture,
                &handles(&surfaces),
                edge,
                edge,
                usage,
                format,
                true,
            ),
            Cmd::CreateSurface {
                surface,
                depth,
                width,
                height,
                format,
                multisample,
                ..
            } => d.create_surface(surface, depth, width, height, format, multisample),
            Cmd::CreateOffscreenSurface {
                surface,
                width,
                height,
                format,
                ..
            } => d.create_surface(surface, false, width, height, format, 0),
            Cmd::CreateBuffer {
                buffer,
                index,
                length,
                format,
                ..
            } => d.create_buffer(buffer, index, length, format),
            Cmd::CreateShader { shader, tokens, .. } => d.create_shader(shader, &tokens.to_words()),
            Cmd::CreateDeclaration {
                declaration,
                elements,
                ..
            } => d.create_declaration(declaration, &elements.bytes()),
            Cmd::CreateQuery { query, .. } => d.create_query(query),
            Cmd::Destroy { object } => d.destroy(object),
            Cmd::UploadSurface { surface, data, .. } => d.upload_surface(surface, &data.bytes()),
            Cmd::UploadBuffer {
                buffer,
                offset,
                data,
                ..
            } => d.upload_buffer(buffer, offset, &data),
            Cmd::ReadSurface { .. } => return Reply::Bytes(None),
            Cmd::GetRenderTargetData { .. } => return Reply::Rows(None),
            Cmd::QueryGetData { .. } => return Reply::Query(None),
            // Fixed-function state and the rest: the game's shaders do not read them; a draw
            // without shaders is skipped and printed by the device.
            Cmd::SetTransform { .. }
            | Cmd::SetClipPlane { .. }
            | Cmd::SetGammaRamp { .. }
            | Cmd::SetTextureStageState { .. }
            | Cmd::SetFvf { .. }
            | Cmd::SetMaterial { .. }
            | Cmd::SetLight { .. }
            | Cmd::LightEnable { .. }
            | Cmd::SetSoftwareVertexProcessing { .. }
            | Cmd::ColorFill { .. }
            | Cmd::SetLod { .. }
            | Cmd::QueryIssue { .. } => {}
        }
        Reply::None
    }
}

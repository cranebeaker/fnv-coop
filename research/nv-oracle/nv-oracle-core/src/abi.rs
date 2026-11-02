//! 32-bit x86 calling conventions: where each argument lives.
//!
//! The same placement rules are used by `nv-call` to build a call and by
//! `nv-probe` to read arguments at a function entry, so a convention
//! written once in a vector or manifest means the same thing in both.
//!
//! Rules (MSVC and GCC agree for these cases):
//! - `cdecl`, `stdcall`: every argument on the stack, first at the lowest
//!   address. `cdecl` is cleaned up by the caller, `stdcall` by the callee.
//! - `thiscall`: the first argument (`this`) in ECX, the rest on the stack,
//!   callee cleans up.
//! - `fastcall`: the first two arguments that are one dword and integer-like
//!   go in ECX and EDX, scanning left to right; everything else (floats,
//!   64-bit values, later arguments) is on the stack; callee cleans up.

use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CallConv {
    Cdecl,
    Stdcall,
    Thiscall,
    Fastcall,
}

impl CallConv {
    pub fn parse(text: &str) -> Result<CallConv, String> {
        match text.trim().to_ascii_lowercase().as_str() {
            "cdecl" => Ok(CallConv::Cdecl),
            "stdcall" => Ok(CallConv::Stdcall),
            "thiscall" => Ok(CallConv::Thiscall),
            "fastcall" => Ok(CallConv::Fastcall),
            other => Err(format!(
                "unknown calling convention {other:?} (cdecl, stdcall, thiscall, fastcall)"
            )),
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            CallConv::Cdecl => "cdecl",
            CallConv::Stdcall => "stdcall",
            CallConv::Thiscall => "thiscall",
            CallConv::Fastcall => "fastcall",
        }
    }

    /// Whether the callee removes its stack arguments with `ret N`.
    pub fn callee_cleans(self) -> bool {
        !matches!(self, CallConv::Cdecl)
    }
}

impl fmt::Display for CallConv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// A declared argument type (manifests) or value kind (vectors).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArgType {
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    Bool,
    Ptr,
    F32,
    F64,
    I64,
    U64,
}

impl ArgType {
    pub fn parse(text: &str) -> Result<ArgType, String> {
        Ok(match text.trim().to_ascii_lowercase().as_str() {
            "i8" => ArgType::I8,
            "u8" => ArgType::U8,
            "i16" => ArgType::I16,
            "u16" => ArgType::U16,
            "i32" | "int" => ArgType::I32,
            "u32" | "uint" => ArgType::U32,
            "bool" => ArgType::Bool,
            "ptr" => ArgType::Ptr,
            "f32" | "float" => ArgType::F32,
            "f64" | "double" => ArgType::F64,
            "i64" => ArgType::I64,
            "u64" => ArgType::U64,
            other => return Err(format!("unknown argument type {other:?}")),
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            ArgType::I8 => "i8",
            ArgType::U8 => "u8",
            ArgType::I16 => "i16",
            ArgType::U16 => "u16",
            ArgType::I32 => "i32",
            ArgType::U32 => "u32",
            ArgType::Bool => "bool",
            ArgType::Ptr => "ptr",
            ArgType::F32 => "f32",
            ArgType::F64 => "f64",
            ArgType::I64 => "i64",
            ArgType::U64 => "u64",
        }
    }

    /// Number of 32-bit stack slots the value occupies.
    pub fn dwords(self) -> usize {
        match self {
            ArgType::F64 | ArgType::I64 | ArgType::U64 => 2,
            _ => 1,
        }
    }

    /// One dword and not floating point: eligible for ECX/EDX.
    pub fn int_like(self) -> bool {
        !matches!(
            self,
            ArgType::F32 | ArgType::F64 | ArgType::I64 | ArgType::U64
        )
    }

    pub fn shape(self) -> ArgShape {
        ArgShape {
            dwords: self.dwords(),
            int_like: self.int_like(),
        }
    }
}

/// What the placement rules need to know about an argument.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArgShape {
    pub dwords: usize,
    pub int_like: bool,
}

/// What the function returns, and therefore which registers to read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetKind {
    Void,
    I32,
    U32,
    Ptr,
    I64,
    /// A float returned in ST0 (the x87 stack), as MSVC and GCC do.
    F32X87,
    /// A double (or long double) returned in ST0.
    F64X87,
    /// A value returned in XMM0 (SSE conventions).
    Xmm0,
}

impl RetKind {
    pub fn parse(text: &str) -> Result<RetKind, String> {
        Ok(match text.trim().to_ascii_lowercase().as_str() {
            "void" => RetKind::Void,
            "i32" => RetKind::I32,
            "u32" => RetKind::U32,
            "ptr" => RetKind::Ptr,
            "i64" => RetKind::I64,
            "f32_x87" => RetKind::F32X87,
            "f64_x87" => RetKind::F64X87,
            "xmm0" => RetKind::Xmm0,
            other => {
                return Err(format!(
                "unknown return kind {other:?} (void, i32, u32, ptr, i64, f32_x87, f64_x87, xmm0)"
            ))
            }
        })
    }

    pub fn name(self) -> &'static str {
        match self {
            RetKind::Void => "void",
            RetKind::I32 => "i32",
            RetKind::U32 => "u32",
            RetKind::Ptr => "ptr",
            RetKind::I64 => "i64",
            RetKind::F32X87 => "f32_x87",
            RetKind::F64X87 => "f64_x87",
            RetKind::Xmm0 => "xmm0",
        }
    }

    /// Whether the value is read from ST0.
    pub fn uses_st0(self) -> bool {
        matches!(self, RetKind::F32X87 | RetKind::F64X87)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Place {
    Ecx,
    Edx,
    /// Dword index from the first stack argument (0 is `[esp+4]` at entry).
    Stack(usize),
}

/// Where every argument goes, and how many stack dwords are used.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Layout {
    /// One entry per argument. A multi-dword argument's place is its first
    /// dword; the rest follow it.
    pub places: Vec<Place>,
    pub stack_dwords: usize,
}

impl Layout {
    /// Bytes of stack arguments (what a callee-cleaned `ret N` pops).
    pub fn stack_bytes(&self) -> usize {
        self.stack_dwords * 4
    }
}

/// Assign argument places for a convention.
pub fn layout(cc: CallConv, args: &[ArgShape]) -> Result<Layout, String> {
    let mut places = Vec::with_capacity(args.len());
    let mut stack = 0usize;
    let mut regs: Vec<Place> = match cc {
        CallConv::Cdecl | CallConv::Stdcall => vec![],
        CallConv::Thiscall => vec![Place::Ecx],
        CallConv::Fastcall => vec![Place::Ecx, Place::Edx],
    };
    regs.reverse(); // pop() hands out ECX first
    if cc == CallConv::Thiscall {
        match args.first() {
            Some(a) if a.int_like => {}
            Some(_) => {
                return Err(
                    "thiscall needs an integer or pointer `this` as the first argument".into(),
                )
            }
            None => {}
        }
    }
    for a in args {
        if a.int_like && a.dwords == 1 {
            if let Some(r) = regs.pop() {
                places.push(r);
                continue;
            }
        }
        places.push(Place::Stack(stack));
        stack += a.dwords;
    }
    Ok(Layout {
        places,
        stack_dwords: stack,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ints(n: usize) -> Vec<ArgShape> {
        vec![ArgType::I32.shape(); n]
    }

    #[test]
    fn parse_names() {
        assert_eq!(CallConv::parse(" ThisCall ").unwrap(), CallConv::Thiscall);
        assert!(CallConv::parse("pascal").is_err());
        assert_eq!(ArgType::parse("double").unwrap(), ArgType::F64);
        assert!(ArgType::parse("i128").is_err());
        assert_eq!(RetKind::parse("f32_x87").unwrap(), RetKind::F32X87);
        assert!(RetKind::parse("st0").is_err());
        for t in [
            "i8", "u8", "i16", "u16", "i32", "u32", "bool", "ptr", "f32", "f64", "i64", "u64",
        ] {
            assert_eq!(ArgType::parse(t).unwrap().name(), t);
        }
        for r in [
            "void", "i32", "u32", "ptr", "i64", "f32_x87", "f64_x87", "xmm0",
        ] {
            assert_eq!(RetKind::parse(r).unwrap().name(), r);
        }
        assert!(RetKind::F64X87.uses_st0() && !RetKind::Xmm0.uses_st0());
    }

    #[test]
    fn cdecl_and_stdcall_use_only_the_stack() {
        for cc in [CallConv::Cdecl, CallConv::Stdcall] {
            let l = layout(cc, &ints(3)).unwrap();
            assert_eq!(
                l.places,
                [Place::Stack(0), Place::Stack(1), Place::Stack(2)]
            );
            assert_eq!(l.stack_bytes(), 12);
        }
        assert!(CallConv::Stdcall.callee_cleans() && !CallConv::Cdecl.callee_cleans());
    }

    #[test]
    fn thiscall_puts_this_in_ecx() {
        let l = layout(
            CallConv::Thiscall,
            &[
                ArgType::Ptr.shape(),
                ArgType::I32.shape(),
                ArgType::F32.shape(),
            ],
        )
        .unwrap();
        assert_eq!(l.places, [Place::Ecx, Place::Stack(0), Place::Stack(1)]);
        assert_eq!(l.stack_dwords, 2);
        assert!(layout(CallConv::Thiscall, &[ArgType::F32.shape()]).is_err());
        assert_eq!(layout(CallConv::Thiscall, &[]).unwrap().stack_dwords, 0);
    }

    #[test]
    fn fastcall_takes_the_first_two_integer_dwords() {
        let l = layout(CallConv::Fastcall, &ints(4)).unwrap();
        assert_eq!(
            l.places,
            [Place::Ecx, Place::Edx, Place::Stack(0), Place::Stack(1)]
        );
        // Floats and 64-bit values are skipped, not counted.
        let l = layout(
            CallConv::Fastcall,
            &[
                ArgType::F32.shape(),
                ArgType::U64.shape(),
                ArgType::Ptr.shape(),
                ArgType::Bool.shape(),
                ArgType::I32.shape(),
            ],
        )
        .unwrap();
        assert_eq!(
            l.places,
            [
                Place::Stack(0),
                Place::Stack(1),
                Place::Ecx,
                Place::Edx,
                Place::Stack(3)
            ]
        );
        assert_eq!(l.stack_dwords, 4);
    }

    #[test]
    fn wide_values_take_two_stack_dwords() {
        let l = layout(
            CallConv::Cdecl,
            &[
                ArgType::F64.shape(),
                ArgType::I32.shape(),
                ArgType::I64.shape(),
            ],
        )
        .unwrap();
        assert_eq!(
            l.places,
            [Place::Stack(0), Place::Stack(2), Place::Stack(3)]
        );
        assert_eq!(l.stack_dwords, 5);
        // A multi-dword raw argument never goes in a register.
        let raw2 = ArgShape {
            dwords: 2,
            int_like: true,
        };
        let l = layout(CallConv::Fastcall, &[raw2, ArgType::I32.shape()]).unwrap();
        assert_eq!(l.places, [Place::Stack(0), Place::Ecx]);
    }
}

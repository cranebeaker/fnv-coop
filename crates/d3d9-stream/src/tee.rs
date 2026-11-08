//! Two backends fed the same commands (`NV_D3D9=real,wgpu`): the primary draws for the player and
//! answers; the shadow gets every command after it, so the two can be compared within one run
//! (shots of the same `frame`).

use crate::cmd::{Backend, Cmd, Data, Reply};

/// A backend that is two: the primary's replies are the answers; the shadow's are dropped. A
/// shadow that panics is printed once and left out from then on; the primary goes on.
///
/// The shadow gets the same window handles as the primary: it should draw off screen.
pub struct Tee {
    primary: Box<dyn Backend>,
    shadow: Option<Box<dyn Backend>>,
}

impl Tee {
    pub fn new(primary: Box<dyn Backend>, shadow: Box<dyn Backend>) -> Tee {
        Tee {
            primary,
            shadow: Some(shadow),
        }
    }

    /// Whether the shadow still gets the commands.
    pub fn shadow_running(&self) -> bool {
        self.shadow.is_some()
    }
}

impl Backend for Tee {
    fn execute(&mut self, cmd: Cmd<Data<'_>>) -> Reply {
        let reply = self.primary.execute(cmd);
        if let Some(shadow) = &mut self.shadow {
            let ran = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                shadow.execute(cmd);
            }));
            if let Err(panic) = ran {
                eprintln!(
                    "the shadow backend stopped at {}: {}; the primary goes on alone",
                    cmd.name(),
                    crate::runner::panic_message(&*panic)
                );
                // Dropped inside a guard too: a shadow left broken may panic in its drop.
                let failed = self.shadow.take();
                let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
                    drop(failed);
                }));
            }
        }
        reply
    }
}

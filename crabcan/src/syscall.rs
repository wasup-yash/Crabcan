use syscallz::{Action, Context};

use crate::error::Ourerror;
pub fn setsyscalls() -> Result<(), Ourerror> {
    log::debug!("Refusing / Filtering unwanted syscalls");
    // Unconditional syscall deny
    // Conditional syscall deny
    // Initialize seccomp profile with all syscalls allowed by default
    if let Ok(mut ctx) = Context::init_with_action(Action::Allow) {
        // Configure profile here
        if let Err(_) = ctx.load() {
            return Err(Ourerror::SyscallsError(0));
        }
        Ok(())
    } else {
        Err(Ourerror::SyscallsError(1))
    }
}

use crate::error::Ourerror;
use libc::TIOCSTI;
use nix::sched::CloneFlags;
use nix::sys::stat::Mode;
use syscallz::{Action, Cmp, Comparator, Context, Syscall};
pub fn setsyscalls() -> Result<(), Ourerror> {
    log::debug!("Refusing / Filtering unwanted syscalls");
    // Unconditional syscall deny
    let syscalls_refused = [
        Syscall::keyctl,
        Syscall::add_key,
        Syscall::request_key,
        Syscall::mbind,
        Syscall::migrate_pages,
        Syscall::move_pages,
        Syscall::set_mempolicy,
        Syscall::userfaultfd,
        Syscall::perf_event_open,
    ];
    let s_isuid: u64 = Mode::S_ISUID.bits().into();
    let s_isgid: u64 = Mode::S_ISGID.bits().into();
    let clone_new_user: u64 = CloneFlags::CLONE_NEWUSER.bits() as u64;

    // Conditional syscall deny
    let syscalls_refuse_comp = [
        (Syscall::chmod, 1, s_isuid),
        (Syscall::chmod, 1, s_isgid),
        (Syscall::fchmod, 1, s_isuid),
        (Syscall::fchmod, 1, s_isgid),
        (Syscall::fchmodat, 2, s_isuid),
        (Syscall::fchmodat, 2, s_isgid),
        (Syscall::unshare, 0, clone_new_user),
        (Syscall::clone, 0, clone_new_user),
        (Syscall::ioctl, 1, TIOCSTI),
    ];
    // Initialize seccomp profile with all syscalls allowed by default
    if let Ok(mut ctx) = Context::init_with_action(Action::Allow) {
        // Configure profile here
        for sc in syscalls_refused.iter() {
            refuse_syscall(&mut ctx, sc)?;
        }
        for (sc, ind, biteq) in syscalls_refuse_comp.iter() {
            refuse_syscall_comp(&mut ctx, *ind, sc, *biteq)?;
        }
        if let Err(_) = ctx.load() {
            return Err(Ourerror::SyscallsError(0));
        }
        Ok(())
    } else {
        Err(Ourerror::SyscallsError(1))
    }
}

//We created this func to refuse the syscall we do not want in child process to execute.
//this will deny any attempt to that syscall

const EPERM: u16 = 1;

fn refuse_syscall(ctx: &mut Context, sc: &Syscall) -> Result<(), Ourerror> {
    match ctx.set_action_for_syscall(Action::Errno(EPERM), *sc) {
        Ok(_) => Ok(()),
        Err(_) => Err(Ourerror::SyscallsError(2)),
    }
}
// taking value and returning permission to set specific syscall {trying to restrict syscall when perticular condition is met}
fn refuse_syscall_comp(
    ctx: &mut Context,
    index: u32,
    sc: &Syscall,
    biteq: u64,
) -> Result<(), Ourerror> {
    match ctx.set_rule_for_syscall(
        Action::Errno(EPERM),
        *sc,
        &[Comparator::new(index, Cmp::MaskedEq, biteq, Some(biteq))],
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(Ourerror::SocketError(3)),
    }
}

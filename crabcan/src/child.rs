use crate::config::Containeropts;
use crate::error::Ourerror;
use nix::sched::clone;
use nix::sched::CloneFlags;
use nix::sys::signal::Signal;
use nix::unistd::Pid;

fn child(config: Containeropts) -> isize {
    log::info!(
        " Starting with command {} and arg {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );
    0
}

const STACK_SIZE: usize = 1024 * 1024;
pub fn generate_child_process(config: Containeropts) -> Result<Pid, Ourerror> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    //Flag defination
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);
    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => Ok(pid),
        Err(_) => Err(Ourerror::ChildProcessError(0)),
    }
}

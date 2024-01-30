use rlimit::{setrlimit, Resource};
use cgroups_rs::cgroup_builder::CgroupBuilder;
use cgroups_rs::MaxValue;
use cgroups_rs::hierarchies::V2;
use cgroups_rs::CgroupPid;
use nix::unistd::Pid;
use crate:: error::Ourerror;
//                       KiB    MiB    Gib
const KMEM_LIMIT: i64 = 1024 * 1024 * 1024;
const MEM_LIMIT: i64 = KMEM_LIMIT;
const MAX_PID: MaxValue = MaxValue::Value(64);
const NOFILE_RLIMIT: u64 = 64;
pub fn restrict_resources(hostname: &String) -> Result<(), Ourerror>{
    log::debug!("Restricting resources for hostname {}", hostname);
    // Cgroups
    let cgs = CgroupBuilder::new(hostname)
        // Allocate less CPU time than other processes
        .cpu().shares(256).done()
        // Limiting the memory usage to 1 GiB
        // The user can limit it to less than this, never increase above 1Gib
        .memory().kernel_memory_limit(KMEM_LIMIT).memory_hard_limit(MEM_LIMIT).done()
        // This process can only create a maximum of 64 child processes
        .pid().maximum_number_of_processes(MAX_PID).done()
.blkio().weight(50).done()
.build(Box::new(V2::new()));
// We apply the cgroups rules to the child process we just created
let raw = pid.as_raw();
let pid : u64 = raw.try_into().unwrap();
if let Err(_) = cgs.add_task(CgroupPid::from(pid)) {
return Err(Ourerror::ResourcesError(0));
};
// Rlimit
// Can create only 64 file descriptors
if let Err(_) = setrlimit(Resource::NOFILE, NOFILE_RLIMIT, NOFILE_RLIMIT){
return Err(Ourerror::ResourcesError(0));
}
Ok(())
}
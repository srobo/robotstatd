use sysinfo::{CpuExt, Pid, PidExt, ProcessExt, System, SystemExt};
use crate::messages::MemoryInfo;

pub fn get_cpu_usage(sys: &mut System) -> Vec<f32> {
    sys.refresh_cpu();
    sys.cpus().iter().map(|cpu| { cpu.cpu_usage() }).collect()
}

pub fn get_mem_usage(sys: &mut System, usercode_pid: u32) -> MemoryInfo {
    sys.refresh_memory();

    MemoryInfo {
        used: sys.used_memory(),
        usercode: if usercode_pid == 0 {
            0
        } else {
            let pid = Pid::from_u32(usercode_pid);
            sys.refresh_process(pid);
            sys.process(pid).unwrap().memory()
        },
        total: sys.total_memory(),
    }
}

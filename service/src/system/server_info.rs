use db::system::models::server_info::{Cpu, CpuLoad, DiskUsage, Memory, Network, Process, Server, SysInfo};
use sysinfo::{Networks, System};

pub fn get_oper_sys_info() -> SysInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    let pid = sysinfo::get_current_pid().expect("failed to get PID");
    let server = Server {
        oper_sys_name: System::name().unwrap_or_else(|| "unknown".to_owned()),
        host_name: System::host_name().unwrap_or_else(|| "unknown".to_owned()),
        system_version: System::long_os_version().unwrap_or_else(|| "unknown".to_owned()),
        system_kerne: System::kernel_version().unwrap_or_else(|| "unknown".to_owned()),
    };
    let process = match sys.process(pid) {
        Some(p) => Process {
            name: p.name().to_str().unwrap_or_else(|| "unknown").to_owned(),
            used_memory: p.memory(),
            used_virtual_memory: p.virtual_memory(),
            cup_usage: p.cpu_usage(),
            start_time: p.start_time(),
            run_time: p.run_time(),
            disk_usage: DiskUsage {
                read_bytes: p.disk_usage().read_bytes,
                total_read_bytes: p.disk_usage().total_read_bytes,
                written_bytes: p.disk_usage().written_bytes,
                total_written_bytes: p.disk_usage().total_written_bytes,
            },
        },
        None => Process { ..Default::default() },
    };

    let mut network: Vec<Network> = Vec::new();

    for (interface_name, data) in Networks::new_with_refreshed_list().iter() {
        network.push(Network {
            name: interface_name.to_string(),
            received: data.received(),
            total_received: data.total_received(),
            transmitted: data.transmitted(),
            total_transmitted: data.total_transmitted(),
        });
    }
    let cpu = Cpu {
        name: sys.cpus().get(0).map(|c| c.name().to_string()).unwrap_or_else(|| "Unknown".to_owned()),
        arch: std::env::consts::ARCH.to_string(),
        cores: sys.physical_core_count().map(|c| c.to_string()).unwrap_or_else(|| "Unknown".to_owned()),
        total_use: sys.global_cpu_usage(),
        frequency: sys.cpus().get(0).map(|c| c.frequency()).unwrap_or_else(|| 0),
        processors: sys.cpus().len(),
    };
    let cpu_load = CpuLoad {
        one: System::load_average().one,
        five: System::load_average().five,
        fifteen: System::load_average().fifteen,
    };
    let memory = Memory {
        total_memory: sys.total_memory(),
        used_memory: sys.used_memory(),
        total_swap: sys.total_swap(),
        used_swap: sys.used_swap(),
    };

    SysInfo {
        server,
        cpu,
        memory,
        process,
        network,
        cpu_load,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_oper_sys_info() {
        let sys_info = get_oper_sys_info();
        println!("{:#?}", sys_info);
    }
}

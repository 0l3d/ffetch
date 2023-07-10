use std::fs::read_to_string;
use sysinfo::{System, SystemExt};
use whoami;

fn get_kernel_version() -> String {
    let mut kernel_result: Vec<String> = Vec::new();

    for line in read_to_string("/proc/version").unwrap().lines() {
        kernel_result.push(line.to_string());
    };

    let kernel_result_full: Vec<_> = kernel_result[0].split(" ").collect();

    return kernel_result_full[2].to_string();

}

fn get_username() -> String {
    return whoami::username()
}

fn get_cpu_name() -> String {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string("/proc/cpuinfo").unwrap().lines() {
        result.push(line.to_string());
    };
    let result_full: &String = &result[4].split("model name\t: ").collect();
    
    return result_full.to_string();
}


fn get_memory() -> String {
    let mut sys = System::new();
    sys.refresh_memory();
    let total_memory = sys.total_memory() / 1000000;
    let used_memory = sys.used_memory() / 1000000;

    return format!("{} / {}", used_memory, total_memory);
}

fn get_os_name() -> String {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string("/etc/os-release").unwrap().lines() {
        result.push(line.to_string());
    };
    let result_full: &String = &result[0].split("NAME=").collect();    
    return result_full.to_string();
}

fn get_hostname() -> String {
    return whoami::hostname();
}

fn get_desktop_env() -> String {
    return whoami::desktop_env().to_string();
}

fn get_cpu_arch() -> String {
    return whoami::arch().to_string();
}

fn main() {

    let ascii : String = format!("    ----------------------------------------------------
    |                F-FETCH
    | USERNAME : {}@{}
    | OS NAME : {}
    | KERNEL VERSION : {}
    | DESKTOP ENV : {}
    | MEMORY : {} MB
    | CPU : {} | {}
    |                                                 
    ---------------------------------------------------- ", get_username(), get_hostname() ,get_os_name(), get_kernel_version(), get_desktop_env() ,get_memory(), get_cpu_name(), get_cpu_arch());
    println!("{}", ascii);

}

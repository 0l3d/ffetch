pub mod ffetch {
    use std::fs::read_to_string;
    use sysinfo::{ System, SystemExt };
    use whoami;
    pub fn get_kernel_version() -> String {
        let mut kernel_result: Vec<String> = Vec::new();

        for line in read_to_string("/proc/version").expect("you are not using linux (/proc/version is empty)").lines() {
            kernel_result.push(line.to_string());
        }

        let kernel_result_full: Vec<_> = kernel_result[0].split(" ").collect();

        return kernel_result_full[2].to_string();
    }

    pub fn get_username() -> String {
        return whoami::username();
    }

    pub fn get_cpu_name() -> String {
        let mut cpuname_result: Vec<String> = Vec::new();

        for line in read_to_string("/proc/cpuinfo").expect("you are not using linux (/proc/cpuinfo is empty)").lines() {
            cpuname_result.push(line.to_string());
        }
        let result_full: &String = &cpuname_result[4].split("model name\t: ").collect();

        return result_full.to_string();
    }

    pub fn get_memory() -> String {
        let mut sys = System::new();
        sys.refresh_memory();
        let total_memory = sys.total_memory() / 1000000;
        let used_memory = sys.used_memory() / 1000000;

        return format!("{} / {}", used_memory, total_memory);
    }

    pub fn get_os_name() -> String {
        let mut osname_result: Vec<String> = Vec::new();

        for line in read_to_string("/etc/os-release").expect("you are not using linux (/proc/os-release is empty)").lines() {
            osname_result.push(line.to_string());
        }
        let result_full: &String = &osname_result[0].split("NAME=").collect();
        return result_full.to_string();
    }

    pub fn get_hostname() -> String {
        let mut hostname_result: Vec<String> = Vec::new();

        for line in read_to_string("/etc/hostname").expect("you are not using linux (/proc/hostname is empty)").lines() {
            hostname_result.push(line.to_string());
        }
        return hostname_result[0].to_string();
    }

    pub fn get_desktop_env() -> String {
        return whoami::desktop_env().to_string();
    }

    pub fn get_cpu_arch() -> String {
        return whoami::arch().to_string();
    }

    pub fn get_device_name() -> String {
        return whoami::devicename().to_string();
    }

    pub fn get_platform() -> String {
        return whoami::platform().to_string();
    }
}

pub mod ffetch {
    use std::{fs::read_to_string, process::Command};
    use sysinfo::{ System, SystemExt };
    use whoami;
    use rsbash::rash;


   
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

        for line in read_to_string("/etc/os-release").expect("you are not using linux (/etc/os-release is empty)").lines() {
            osname_result.push(line.to_string());
        }
        let result_full: &String = &osname_result[0].split("NAME=").collect();
        return result_full.split("\"").collect();
    }

    pub fn get_hostname() -> String {
        let mut hostname_result: Vec<String> = Vec::new();

        for line in read_to_string("/etc/hostname").expect("you are not using linux (/etc/hostname is empty)").lines() {
            hostname_result.push(line.to_string());
        }
        return hostname_result[0].to_string();
    }

    pub fn get_desktop_env() -> String {
        return whoami::desktop_env().to_string();
    }

    pub fn get_cpu_arch() -> String {
        let get_cpu_arch_command = Command::new("uname").arg("-m").output().expect("lscpu command error");
        return String::from_utf8(get_cpu_arch_command.stdout).expect("Error : ").split("\n").collect();
    }

    pub fn get_platform() -> String {
        return whoami::platform().to_string();
    }

    pub fn get_uptime() -> String {
        let uptime_command  = Command::new("uptime").arg("-p").output().expect("uptime command error");
        let uptime : String = (String::from_utf8(uptime_command.stdout).expect("Error uptime from utf8 string")).split("up ").collect();
        return uptime.split("\n").collect();
    }

    pub fn get_packages() -> String {
        let getos: &str = &get_os_name();
        let mut for_var = String::new();
        let mut return_var: Vec<String> = Vec::new();
        
        match getos {
            "Debian" => for_var = String::from_utf8((Command::new("apt").arg("list").arg("--installed").output().expect("uptime command error")).stdout).expect("Error osname from utf8 string"),
            "Fedora" => for_var = String::from_utf8((Command::new("dnf").arg("list").arg("installed").output().expect("uptime command error")).stdout).expect("Error osname from utf8 string"),
            "Arch Linux" => for_var = String::from_utf8((Command::new("pacman").arg("-Q").output().expect("uptime command error")).stdout).expect("Error osname from utf8 string"),
            _ => ()
        };

        for line in for_var.lines() {
            return_var.push(line.to_string());
        }

        return (return_var.len()).to_string();
    }

    pub fn get_gpu() -> String {
        let cpu_command: String = String::from_utf8(Command::new("lspci").output().expect("lspci command error").stdout).expect("fromutf8 error");
        let mut cpu_lines: Vec<String> = Vec::new();
        for line in cpu_command.lines() {
            cpu_lines.push(line.to_string());
        }

        let cpu_1_1 : String = cpu_lines[1].split("00:02.0 VGA compatible controller: ").collect();
        return cpu_1_1.split(" (rev 02)").collect();
    }

    pub fn get_shell() -> String {
        let shell_command = rash!("echo $SHELL").expect("error rash command for shell");
        return shell_command.1.split("\n").collect();
    }

}

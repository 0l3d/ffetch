pub mod ffetch {
    use display_info::DisplayInfo;
    use lazy_static::lazy_static;
    use rsbash::rash;
    use std::env;
    use std::{fs::read_to_string, process::Command};
    use sysinfo::{Disks, System};
    use whoami;

    lazy_static! {
        static ref DISPLAY_INFORMATION: Vec<DisplayInfo> = DisplayInfo::all().unwrap();
    }

    pub fn get_kernel_version() -> String {
        let mut kernel_result: Vec<String> = Vec::new();

        for line in read_to_string("/proc/version")
            .expect("you are not using linux (/proc/version is empty)")
            .lines()
        {
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

        for line in read_to_string("/proc/cpuinfo")
            .expect("you are not using linux (/proc/cpuinfo is empty)")
            .lines()
        {
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

        for line in read_to_string("/etc/os-release")
            .expect("you are not using linux (/etc/os-release is empty)")
            .lines()
        {
            osname_result.push(line.to_string());
        }
        let result_full: &String = &osname_result[0].split("NAME=").collect();
        return result_full.split("\"").collect();
    }

    pub fn get_hostname() -> String {
        let mut hostname_result: Vec<String> = Vec::new();

        for line in read_to_string("/etc/hostname")
            .expect("you are not using linux (/etc/hostname is empty)")
            .lines()
        {
            hostname_result.push(line.to_string());
        }
        return hostname_result[0].to_string();
    }

    pub fn get_desktop_env() -> String {
        let de = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .or_else(|_| env::var("GDMSESSION"))
            .unwrap_or_else(|_| "Unknown".to_string());

        return de;
    }

    pub fn get_cpu_arch() -> String {
        let get_cpu_arch_command = Command::new("uname")
            .arg("-m")
            .output()
            .expect("lscpu command error");
        return String::from_utf8(get_cpu_arch_command.stdout)
            .expect("Error : ")
            .split("\n")
            .collect();
    }

    pub fn get_platform() -> String {
        return whoami::platform().to_string();
    }

    pub fn get_uptime() -> String {
        let uptime_command = Command::new("uptime")
            .arg("-p")
            .output()
            .expect("uptime command error");
        let uptime: String = (String::from_utf8(uptime_command.stdout)
            .expect("Error uptime from utf8 string"))
        .split("up ")
        .collect();
        return uptime.split("\n").collect();
    }

    pub fn get_packages() -> String {
        let getos: &str = &get_os_name();
        let mut for_var = String::new();
        let mut return_var: Vec<String> = Vec::new();

        match getos {
            "Debian" => {
                for_var = String::from_utf8(
                    (Command::new("apt")
                        .arg("list")
                        .arg("--installed")
                        .output()
                        .expect("packages command error"))
                    .stdout,
                )
                .expect("Error osname from utf8 string")
            }
            "Fedora" => {
                for_var = String::from_utf8(
                    (Command::new("dnf")
                        .arg("list")
                        .arg("installed")
                        .output()
                        .expect("packages command error"))
                    .stdout,
                )
                .expect("Error osname from utf8 string")
            }
            "Arch Linux" => {
                for_var = String::from_utf8(
                    (Command::new("pacman")
                        .arg("-Q")
                        .output()
                        .expect("packages command error"))
                    .stdout,
                )
                .expect("Error osname from utf8 string")
            }
            _ => (),
        };

        for line in for_var.lines() {
            return_var.push(line.to_string());
        }

        return (return_var.len()).to_string();
    }

    pub fn get_gpu() -> String {
        let output = Command::new("lspci")
            .output()
            .expect("Failed to execute lspci");

        let lspci_output = String::from_utf8_lossy(&output.stdout);

        for line in lspci_output.lines() {
            if let Some(idx) = line.find("VGA compatible controller:") {
                let full = &line[idx + "VGA compatible controller: ".len()..];
                if full.contains("NVIDIA") {
                    if let Some(start) = full.find("GeForce") {
                        let end = full.find("] (rev").unwrap_or(full.len());
                        return format!("NVIDIA {}", full[start..end].trim());
                    } else {
                        return format!("NVIDIA {}", full.trim());
                    }
                }

                if full.contains("AMD") {
                    let end = full.find("(rev").unwrap_or(full.len());
                    return full[..end].trim().to_string();
                }
            }
        }

        "GPU not found".to_string()
    }

    pub fn get_m_gpu() -> String {
        let output = Command::new("lspci")
            .output()
            .expect("Failed to execute lspci");

        let lspci_output = String::from_utf8_lossy(&output.stdout);

        for line in lspci_output.lines() {
            if line.contains("3D controller:") {
                if let Some(idx) = line.find("3D controller:") {
                    return line[idx + "3D controller:".len()..].trim().to_string();
                }
            }
        }

        "Mobile GPU not found".to_string()
    }

    pub fn get_shell() -> String {
        let shell_command = rash!("echo $SHELL").expect("error rash command for shell");
        return shell_command.1.split("\n").collect();
    }
    pub fn get_monitor(monitor_index: usize) -> String {
        let mut trues = "";

        if DISPLAY_INFORMATION[monitor_index].is_primary {
            trues = "*";
        }

        let all_of_things = format!(
            "{} {}x{} {} Hz {}",
            DISPLAY_INFORMATION[monitor_index].friendly_name,
            DISPLAY_INFORMATION[monitor_index].width,
            DISPLAY_INFORMATION[monitor_index].height,
            DISPLAY_INFORMATION[monitor_index].frequency,
            trues
        );
        return all_of_things;
    }

    pub fn get_disks(disk_point: &str) -> String {
        let disks = Disks::new_with_refreshed_list();
        for disk in disks.list() {
            let available = disk.available_space() / 1000000000;
            let total = disk.total_space() / 1000000000;
            let used = total - available;
            if disk.mount_point().to_str() == Some(disk_point) {
                let return_point = format!("{} GB / {} GB", used, total);
                return return_point;
            }
        }
        return String::new();
    }
}

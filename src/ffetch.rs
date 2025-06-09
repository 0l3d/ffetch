pub mod ffetch {
    use display_info::DisplayInfo;
    use lazy_static::lazy_static;
    use regex::Regex;
    use rsbash::rash;
    use std::env;
    use std::{fs::read_to_string, process::Command};
    use sysinfo::{Disks, System};
    use which::which;
    use whoami;

    lazy_static! {
        static ref DISPLAY_INFORMATION: Vec<DisplayInfo> = DisplayInfo::all().unwrap();
        static ref REGEX: Regex = Regex::new(r"window id # (0x[0-9a-f]+)").unwrap();
        static ref NAME_REGEX: Regex = Regex::new(r#""([^"]+)""#).unwrap();
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

        kernel_result_full[2].to_string()
    }

    pub fn get_locale() -> String {
        env::var("LC_ALL")
            .or_else(|_| env::var("LANG"))
            .unwrap_or_else(|_| "C".to_string())
    }

    pub fn get_username() -> String {
        whoami::username()
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

        result_full.to_string()
    }

    pub fn get_memory() -> String {
        let mut sys = System::new();
        sys.refresh_memory();
        let total_memory = sys.total_memory() / 1000000;
        let used_memory = sys.used_memory() / 1000000;

        format!("{} / {}", used_memory, total_memory)
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
        result_full.split("\"").collect()
    }

    pub fn get_hostname() -> String {
        let mut hostname_result: Vec<String> = Vec::new();

        for line in read_to_string("/etc/hostname")
            .expect("you are not using linux (/etc/hostname is empty)")
            .lines()
        {
            hostname_result.push(line.to_string());
        }
        hostname_result[0].to_string()
    }

    pub fn get_desktop_env() -> String {
        env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .or_else(|_| env::var("GDMSESSION"))
            .unwrap_or_else(|_| "Unknown".to_string())
    }

    pub fn get_cpu_arch() -> String {
        let get_cpu_arch_command = Command::new("uname")
            .arg("-m")
            .output()
            .expect("uname command error");
        String::from_utf8(get_cpu_arch_command.stdout)
            .expect("Error : ")
            .split("\n")
            .collect()
    }

    pub fn get_platform() -> String {
        whoami::platform().to_string()
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
        uptime.split("\n").collect()
    }

    pub fn get_packages() -> String {
        let mut res = Vec::new();

        let managers = [
            ("emerge", vec!["qlist", "-I"]),
            ("flatpak", vec!["flatpak", "list"]),
            ("apt-get", vec!["apt-get", "list", "--installed"]),
            ("dnf", vec!["dnf", "list", "installed"]),
            ("yum", vec!["yum", "list", "installed"]),
            ("pacman", vec!["pacman", "-Q"]),
            ("zypper", vec!["zypper", "se", "--installed-only"]),
            ("nix", vec!["nix-env", "-q"]),
            ("xbps", vec!["xbps-query", "-l"]),
        ];

        for (name, cmd) in managers.iter() {
            if which(cmd[0]).is_ok() {
                let output = Command::new(&cmd[0])
                    .args(&cmd[1..])
                    .output()
                    .expect("Command failed");

                let stdout = String::from_utf8_lossy(&output.stdout);
                let count = stdout.lines().count();

                res.push(format!("{} ({})", count, name));
            }
        }

        res.join(" ")
    }

    pub fn get_gpu() -> String {
        let output = Command::new("lspci")
            .output()
            .expect("Failed to execute lspci, not found.");
        let lspci_output = String::from_utf8_lossy(&output.stdout);

        for line in lspci_output.lines() {
            if let Some(idx) = line.find("VGA compatible controller:") {
                let full = &line[idx + "VGA compatible controller: ".len()..];

                if full.contains("NVIDIA") {
                    if let Some(start) = full.find("GeForce") {
                        let end = full
                            .find("] (rev")
                            .or_else(|| full.find("(rev"))
                            .unwrap_or(full.len());
                        return format!("NVIDIA {}", full[start..end].trim());
                    } else if let Some(start) = full.find("Quadro") {
                        let end = full
                            .find("] (rev")
                            .or_else(|| full.find("(rev"))
                            .unwrap_or(full.len());
                        return format!("NVIDIA {}", full[start..end].trim());
                    } else {
                        let end = full.find("(rev").unwrap_or(full.len());
                        return format!("NVIDIA {}", full[..end].trim());
                    }
                }

                if full.contains("AMD") || full.contains("ATI") {
                    let end = full.find("(rev").unwrap_or(full.len());
                    let gpu_name = full[..end].trim();

                    if gpu_name.starts_with("Advanced Micro Devices") {
                        return gpu_name
                            .replace("Advanced Micro Devices, Inc. [", "AMD ")
                            .replace("]", "")
                            .trim()
                            .to_string();
                    }
                    return gpu_name.to_string();
                }

                if full.contains("Intel") {
                    let end = full.find("(rev").unwrap_or(full.len());
                    let gpu_name = full[..end].trim();

                    if gpu_name.starts_with("Intel Corporation") {
                        return gpu_name
                            .replace("Intel Corporation ", "Intel ")
                            .trim()
                            .to_string();
                    }
                    return gpu_name.to_string();
                }
            }
        }
        "GPU not found".to_string()
    }

    pub fn get_m_gpu() -> String {
        let output = Command::new("lspci")
            .output()
            .expect("Failed to execute lspci, not found.");
        let lspci_output = String::from_utf8_lossy(&output.stdout);

        for line in lspci_output.lines() {
            if line.contains("3D controller:") {
                if let Some(idx) = line.find("3D controller:") {
                    let full = &line[idx + "3D controller:".len()..].trim();

                    if full.contains("NVIDIA") {
                        if let Some(start) = full.find("GeForce") {
                            let end = full
                                .find("] (rev")
                                .or_else(|| full.find("(rev"))
                                .unwrap_or(full.len());
                            return format!("NVIDIA {}", full[start..end].trim());
                        } else {
                            let end = full.find("(rev").unwrap_or(full.len());
                            return format!("NVIDIA {}", full[..end].trim());
                        }
                    }

                    if full.contains("AMD") || full.contains("ATI") {
                        let end = full.find("(rev").unwrap_or(full.len());
                        let gpu_name = full[..end].trim();

                        if gpu_name.starts_with("Advanced Micro Devices") {
                            return gpu_name
                                .replace("Advanced Micro Devices, Inc. [", "AMD ")
                                .replace("]", "")
                                .trim()
                                .to_string();
                        }
                        return gpu_name.to_string();
                    }

                    if full.contains("Intel") {
                        let end = full.find("(rev").unwrap_or(full.len());
                        let gpu_name = full[..end].trim();

                        if gpu_name.starts_with("Intel Corporation") {
                            return gpu_name
                                .replace("Intel Corporation ", "Intel ")
                                .trim()
                                .to_string();
                        }
                        return gpu_name.to_string();
                    }

                    let end = full.find("(rev").unwrap_or(full.len());
                    return full[..end].trim().to_string();
                }
            }
        }
        "Mobile GPU not found".to_string()
    }

    pub fn get_shell() -> String {
        let shell_command = rash!("echo $SHELL").expect("error rash command for shell");
        shell_command.1.split("\n").collect()
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
        all_of_things
    }

    pub fn get_terminal() -> String {
        let output = Command::new("xprop")
            .args(["-root", "_NET_ACTIVE_WINDOW"])
            .output()
            .expect("xprop run error.");

        let stdout = String::from_utf8_lossy(&output.stdout);

        let window_id = match REGEX.captures(&stdout) {
            Some(cap) => cap[1].to_string(),
            None => {
                eprintln!("⚠️ Window not found.");
                return "".to_string();
            }
        };

        let output = Command::new("xprop")
            .args(["-id", &window_id])
            .output()
            .expect("xprop id run error.");

        let info = String::from_utf8_lossy(&output.stdout);
        for line in info.lines() {
            if line.starts_with("WM_CLASS") {
                let mut matches = NAME_REGEX.captures_iter(line);

                if let Some(app_class) = matches.next() {
                    let terminal = app_class[1].to_string();
                    return terminal;
                }
            }
        }
        return "".to_string();
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
        String::new()
    }
}

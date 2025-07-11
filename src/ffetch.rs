use display_info::DisplayInfo;
use lazy_static::lazy_static;
use regex::Regex;
use std::{env, fs::read_to_string, process::Command};
use which::which;

lazy_static! {
    static ref DISPLAY_INFORMATION: Vec<DisplayInfo> = DisplayInfo::all().unwrap();
    static ref REGEX: Regex = Regex::new(r"window id # (0x[0-9a-f]+)").unwrap();
    static ref NAME_REGEX: Regex = Regex::new(r#""([^"]+)""#).unwrap();
}


/// Gets the Linux kernel version from `/proc/version`.
/// 
/// This function reads the kernel version information and extracts
/// the version number (third field after splitting by spaces).
/// 
/// # Returns
/// 
/// Returns a `String` containing the kernel version (e.g., "6.1.0-18-amd64").
/// 
/// # Panics
/// 
/// Panics if `/proc/version` cannot be read (non-Linux systems).
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_kernel_version;
/// 
/// let kernel = get_kernel_version();
/// println!("Kernel version: {}", kernel);
/// // Output: Kernel version: 6.1.0-18-amd64
/// ```
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

/// Gets the system locale from environment variables.
/// 
/// Checks `LC_ALL` first, then `LANG`, and defaults to "C" if neither is set.
/// 
/// # Returns
/// 
/// Returns a `String` containing the locale (e.g., "en_US.UTF-8").
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_locale;
/// 
/// let locale = get_locale();
/// println!("System locale: {}", locale);
/// // Output: System locale: en_US.UTF-8
/// ```
pub fn get_locale() -> String {
    env::var("LC_ALL")
        .or_else(|_| env::var("LANG"))
        .unwrap_or_else(|_| "C".to_string())
}

/// Gets the current username from environment variables.
/// 
/// Checks `USER` first, then `USERNAME`, and defaults to "User not found" if neither is set.
/// 
/// # Returns
/// 
/// Returns a `String` containing the username.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_username;
/// 
/// let username = get_username();
/// println!("Current user: {}", username);
/// // Output: Current user: john
/// ```
pub fn get_username() -> String {
    env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| "User not found".to_string())
}


/// Gets the GTK theme from the `GTK_THEME` environment variable.
/// 
/// # Returns
/// 
/// Returns a `String` containing the GTK theme name or an error message if not set.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::gtk_theme;
/// 
/// let theme = gtk_theme();
/// println!("GTK Theme: {}", theme);
/// // Output: GTK Theme: Adwaita:dark
/// ```
pub fn gtk_theme() -> String {
    env::var("GTK_THEME").unwrap_or_else(|_| "GTK_THEME variable is not setting up".to_string())
}


/// Gets the Qt platform theme from the `QT_QPA_PLATFORMTHEME` environment variable.
/// 
/// # Returns
/// 
/// Returns a `String` containing the Qt theme name or an error message if not set.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::qt_theme;
/// 
/// let theme = qt_theme();
/// println!("Qt Theme: {}", theme);
/// // Output: Qt Theme: gtk2
/// ```
pub fn qt_theme() -> String {
    env::var("QT_QPA_PLATFORMTHEME")
        .unwrap_or_else(|_| "QT_QPA_PLATFORMTHEME variable is not setting up".to_string())
}


/// Gets the CPU name from `/proc/cpuinfo`.
/// 
/// Reads the CPU model name from the fifth line of `/proc/cpuinfo`.
/// 
/// # Returns
/// 
/// Returns a `String` containing the CPU model name.
/// 
/// # Panics
/// 
/// Panics if `/proc/cpuinfo` cannot be read (non-Linux systems).
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_cpu_name;
/// 
/// let cpu = get_cpu_name();
/// println!("CPU: {}", cpu);
/// // Output: CPU: Intel(R) Core(TM) i7-8750H CPU @ 2.20GHz
/// ```
pub fn get_cpu_name() -> String {
    for line in read_to_string("/proc/cpuinfo")
        .expect("you are not using linux (/proc/cpuinfo is empty)")
        .lines()
    {
        if line.starts_with("model name") {
            if let Some(cpu) = line.split(":").nth(1) {
                return cpu.trim().to_string();
            }
        }
    }
    "Unknown CPU".to_string()
}


/// Gets memory usage information from `/proc/meminfo`.
/// 
/// Calculates used memory by subtracting available memory from total memory.
/// Values are converted from kB to MB.
/// 
/// # Returns
/// 
/// Returns a `String` in the format "used / total" (e.g., "8192 / 16384").
/// 
/// # Panics
/// 
/// Panics if `/proc/meminfo` cannot be read.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_memory;
/// 
/// let memory = get_memory();
/// println!("Memory: {} MB", memory);
/// // Output: Memory: 8192 / 16384 MB
/// ```
pub fn get_memory() -> String {
    let mut total_memory: u64 = 0;
    let mut free_memory: u64 = 0;
    for line in read_to_string("/proc/meminfo")
        .expect("meminfo read error")
        .lines()
    {
        let spline: Vec<&str> = line.split(":").collect();
        match spline[0] {
            "MemTotal" => {
                let total_memory_split: Vec<_> = spline[1].split("kB").collect();
                let total_memory_trim = total_memory_split[0].trim();
                let _total_memory_asint: u64 = total_memory_trim.parse().unwrap();
                total_memory = _total_memory_asint / 1024;
            }
            "MemAvailable" => {
                let free_memory_split: Vec<_> = spline[1].split("kB").collect();
                let free_memory_trim = free_memory_split[0].trim();
                let _free_memory_asint: u64 = free_memory_trim.parse().unwrap();
                free_memory = _free_memory_asint / 1024;
            }
            _ => {}
        }
    }
    let memory_usage = total_memory - free_memory;
    format!("{} / {}", memory_usage, total_memory)
}


/// Gets the operating system name from `/etc/os-release`.
/// 
/// Extracts the NAME field from the os-release file and removes quotes.
/// 
/// # Returns
/// 
/// Returns a `String` containing the OS name (e.g., "Ubuntu").
/// 
/// # Panics
/// 
/// Panics if `/etc/os-release` cannot be read.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_os_name;
/// 
/// let os = get_os_name();
/// println!("OS: {}", os);
/// // Output: OS: Ubuntu
/// ```
pub fn get_os_name() -> String {
    for line in read_to_string("/etc/os-release")
        .expect("you are not using linux (/etc/os-release is empty)")
        .lines()
    {
        if line.starts_with("NAME=") {
            return line
                .trim_start_matches("NAME=")
                .trim_matches('"')
                .to_string();
        }
    }
    "Unknown OS".to_string()
}

/// Gets the system hostname from `/etc/hostname`.
/// 
/// # Returns
/// 
/// Returns a `String` containing the hostname.
/// 
/// # Panics
/// 
/// Panics if `/etc/hostname` cannot be read.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_hostname;
/// 
/// let hostname = get_hostname();
/// println!("Hostname: {}", hostname);
/// // Output: Hostname: my-computer
/// ```
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

/// Gets the desktop environment name from environment variables.
/// 
/// Checks multiple environment variables in order of preference:
/// `XDG_CURRENT_DESKTOP`, `DESKTOP_SESSION`, `GDMSESSION`.
/// 
/// # Returns
/// 
/// Returns a `String` containing the desktop environment name or "Unknown".
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_desktop_env;
/// 
/// let de = get_desktop_env();
/// println!("Desktop Environment: {}", de);
/// // Output: Desktop Environment: GNOME
/// ```
pub fn get_desktop_env() -> String {
    env::var("XDG_CURRENT_DESKTOP")
        .or_else(|_| env::var("DESKTOP_SESSION"))
        .or_else(|_| env::var("GDMSESSION"))
        .unwrap_or_else(|_| "Unknown".to_string())
}

/// Gets the compositor backend from the `XDG_BACKEND` environment variable.
/// 
/// # Returns
/// 
/// Returns a `String` containing the backend name or "Unknown Backend".
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_compositor;
/// 
/// let compositor = get_compositor();
/// println!("Compositor: {}", compositor);
/// // Output: Compositor: wayland
/// ```
pub fn get_compositor() -> String {
    env::var("XDG_BACKEND").unwrap_or_else(|_| "Unknown Backend".to_string())
}


/// Gets the CPU architecture using the `uname -m` command.
/// 
/// # Returns
/// 
/// Returns a `String` containing the architecture (e.g., "x86_64").
/// 
/// # Panics
/// 
/// Panics if the `uname` command fails.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_cpu_arch;
/// 
/// let arch = get_cpu_arch();
/// println!("Architecture: {}", arch);
/// // Output: Architecture: x86_64
/// ```
pub fn get_cpu_arch() -> String {
    let output = Command::new("uname")
        .arg("-m")
        .output()
        .expect("uname command error");

    String::from_utf8(output.stdout)
        .expect("UTF-8 error")
        .trim()
        .to_string()
}

/// Gets the platform name.
/// 
/// Currently hardcoded to return "Linux" as this library only supports Linux.
/// 
/// # Returns
/// 
/// Returns a `String` containing "Linux".
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_platform;
/// 
/// let platform = get_platform();
/// println!("Platform: {}", platform);
/// // Output: Platform: Linux
/// ```
pub fn get_platform() -> String {
    "Linux".to_string()
}


/// Gets the system uptime using the `uptime -p` command.
/// 
/// # Returns
/// 
/// Returns a `String` containing the uptime in human-readable format.
/// 
/// # Panics
/// 
/// Panics if the `uptime` command fails.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_uptime;
/// 
/// let uptime = get_uptime();
/// println!("Uptime: {}", uptime);
/// // Output: Uptime: 2 hours, 30 minutes
/// ```
pub fn get_uptime() -> String {
    let output = Command::new("uptime")
        .arg("-p")
        .output()
        .expect("uptime command error");

    let stdout = String::from_utf8(output.stdout).expect("uptime UTF-8 parse failed");

    stdout.trim_start_matches("up ").trim().to_string()
}

/// Gets the number of installed packages from various package managers.
/// 
/// Detects and counts packages from multiple package managers including:
/// Portage, Flatpak, APT, DNF, YUM, Pacman, Zypper, Nix, and XBPS.
/// 
/// # Returns
/// 
/// Returns a `String` containing package counts for each detected manager.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_packages;
/// 
/// let packages = get_packages();
/// println!("Packages: {}", packages);
/// // Output: Packages: 1234 (apt-get) 56 (flatpak)
/// ```
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
            let output = Command::new(cmd[0])
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


/// Gets the primary GPU information using `lspci`.
/// 
/// Parses the output of `lspci` to find VGA compatible controllers and
/// formats the GPU name for better readability.
/// 
/// # Returns
/// 
/// Returns a `String` containing the GPU name or an error message.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_gpu;
/// 
/// let gpu = get_gpu();
/// println!("GPU: {}", gpu);
/// // Output: GPU: NVIDIA GeForce RTX 3080
/// ```
pub fn get_gpu() -> String {
    if !is_lspci_available() {
        return "lspci not available".to_string();
    }

    let output = match Command::new("lspci").output() {
        Ok(output) => output,
        Err(_) => return "Failed to execute lspci".to_string(),
    };

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

            let end = full.find("(rev").unwrap_or(full.len());
            return full[..end].trim().to_string();
        }
    }
    "GPU not found".to_string()
}

/// Gets the secondary/mobile GPU information using `lspci`.
/// 
/// Searches for 3D controllers or secondary VGA controllers,
/// typically used for discrete/mobile GPUs in laptops.
/// 
/// # Returns
/// 
/// Returns a `String` containing the mobile GPU name or "Mobile GPU not found".
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_m_gpu;
/// 
/// let mobile_gpu = get_m_gpu();
/// println!("Mobile GPU: {}", mobile_gpu);
/// // Output: Mobile GPU: NVIDIA GeForce GTX 1650 Mobile
/// ```
pub fn get_m_gpu() -> String {
    if !is_lspci_available() {
        return "lspci not available".to_string();
    }

    let output = match Command::new("lspci").output() {
        Ok(output) => output,
        Err(_) => return "Failed to execute lspci".to_string(),
    };

    let lspci_output = String::from_utf8_lossy(&output.stdout);
    let mut vga_lines = Vec::new();

    for line in lspci_output.lines() {
        if line.contains("VGA compatible controller:") {
            vga_lines.push(line.to_string());
        }

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

    if vga_lines.len() > 1 {
        let second_vga = &vga_lines[1];
        if let Some(idx) = second_vga.find("VGA compatible controller:") {
            let full = &second_vga[idx + "VGA compatible controller: ".len()..];

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

            let end = full.find("(rev").unwrap_or(full.len());
            return full[..end].trim().to_string();
        }
    }

    "Mobile GPU not found".to_string()
}



/// Checks if the `lspci` command is available on the system.
/// 
/// # Returns
/// 
/// Returns `true` if `lspci` is available, `false` otherwise.
fn is_lspci_available() -> bool {
    Command::new("which")
        .arg("lspci")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}


/// Gets the shell from the `SHELL` environment variable.
/// 
/// # Returns
/// 
/// Returns a `String` containing the shell path or "Unknown Shell".
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_shell;
/// 
/// let shell = get_shell();
/// println!("Shell: {}", shell);
/// // Output: Shell: /bin/bash
/// ```
pub fn get_shell() -> String {
    env::var("SHELL").unwrap_or_else(|_| "Unknown Shell".to_string())
}


/// Gets monitor information for a specific monitor index.
/// 
/// Retrieves display information including resolution, refresh rate,
/// and primary monitor status.
/// 
/// # Arguments
/// 
/// * `monitor_index` - The zero-based index of the monitor to query
/// 
/// # Returns
/// 
/// Returns a `String` containing monitor details with format:
/// "Name WIDTHxHEIGHT FREQUENCY Hz *" (asterisk indicates primary monitor)
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_monitor;
/// 
/// let monitor = get_monitor(0);
/// println!("Primary Monitor: {}", monitor);
/// // Output: Primary Monitor: HDMI-A-1 1920x1080 60 Hz *
/// ```
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

/// Gets the current terminal emulator name using `xprop`.
/// 
/// Uses X11 properties to identify the currently active window
/// and extract the terminal emulator name from WM_CLASS.
/// 
/// # Returns
/// 
/// Returns a `String` containing the terminal name or falls back to `$TERM`.
/// 
/// # Panics
/// 
/// Panics if `xprop` command fails to execute.
/// 
/// # Examples
/// 
/// ```rust
/// use ffetch::get_terminal;
/// 
/// let terminal = get_terminal();
/// println!("Terminal: {}", terminal);
/// // Output: Terminal: gnome-terminal
/// ```
/// 
/// # Note
/// 
/// This function requires X11 and the `xprop` utility to be available.
/// It may not work in Wayland environments.

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
    env::var("TERM").unwrap_or_else(|_| "Unknown".to_string())
}

/// Gets disk usage information for a specific mount point using `df -h`.
///
/// Executes the `df -h` command and parses the output to find disk usage
/// statistics for the specified mount point.
///
/// # Arguments
///
/// * `mountpoint` - The mount point to query (e.g., "/", "/home")
///
/// # Returns
///
/// Returns a `String` with format "USED / TOTAL (PERCENTAGE)" or empty string if not found.
///
/// # Panics
///
/// Panics if the `df` command fails to execute.
///
/// # Examples
///
/// ```rust
/// use ffetch::get_disks;
///
/// let root_disk = get_disks("/");
/// println!("Root disk usage: {}", root_disk);
/// // Output: Root disk usage: 45G / 100G (45%)
///
/// let home_disk = get_disks("/home");
/// println!("Home disk usage: {}", home_disk);
/// // Output: Home disk usage: 120G / 500G (24%)
/// ```
pub fn get_disks(mountpoint: &str) -> String {
    // 0 disk | 1 size | 2 used | 3 avail | 4 use percent | 5 mountpoint
    let dfh = Command::new("df")
        .arg("-h")
        .output()
        .expect("df -h command error");
    let info = String::from_utf8_lossy(&dfh.stdout);
    for line in info.lines() {
        let info_split: Vec<_> = line.split_whitespace().collect();
        if info_split.len() >= 6 && info_split[5] == mountpoint {
            let info_split_format =
                format!("{}  / {} ({})", info_split[2], info_split[1], info_split[4]);
            return info_split_format;
        }
    }
    "".to_string()
}

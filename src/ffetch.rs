use std::{
    env,
    fs::{metadata, read, read_dir, read_link, read_to_string, File},
    io::{BufRead, BufReader, Error, ErrorKind},
    path::Path,
    process::Command,
};

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
pub fn get_kernel_version() -> Result<String, Error> {
    let contents = read_to_string("/proc/version").map_err(|e| {
        Error::new(
            e.kind(),
            "Failed to read /proc/version. Are you running Linux?",
        )
    })?;

    let first_line = contents
        .lines()
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "/proc/version appears to be empty."))?;

    let parts: Vec<&str> = first_line.split_whitespace().collect();

    if parts.len() < 3 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Unexpected format in /proc/version.",
        ));
    }

    Ok(parts[2].to_string())
}

/// Gets the motherboard vendor from `/sys/class/dmi/id/board_vendor`.
///
/// This function reads the vendor information of the mainboard from the
/// sysfs DMI interface provided by the Linux kernel.
///
/// # Returns
///
/// Returns a `String` containing the motherboard vendor name  
/// (e.g., `"Micro-Star International Co., Ltd."`, `"ASUSTeK COMPUTER INC."`).
///
/// # Panics
///
/// Panics if `/sys/class/dmi/id/board_vendor` cannot be read
/// (e.g., if the file does not exist or the system does not provide DMI info).
///
/// # Examples
///
/// ```rust
/// use ffetch::get_board_vendor;
///
/// let vendor = get_board_vendor();
/// println!("Motherboard vendor: {}", vendor);
/// // Output: Motherboard vendor: Micro-Star International Co., Ltd.
/// ```
pub fn get_board_vendor() -> Result<String, Error> {
    let content = read_to_string("/sys/class/dmi/id/board_vendor")
        .map_err(|e| Error::new(e.kind(), "Failed to read /sys/class/dmi/id/board_vendor. File may not exist or permission denied."))?;

    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "The board_vendor file is empty.",
        ));
    }

    Ok(trimmed.to_string())
}

/// Gets the motherboard name (model) from `/sys/class/dmi/id/board_name`.
///
/// This function reads the mainboard model information from the
/// sysfs DMI interface provided by the Linux kernel.
///
/// # Returns
///
/// Returns a `String` containing the motherboard model name  
/// (e.g., `"MS-7D95"`, `"ROG STRIX B550-F GAMING"`).
///
/// # Panics
///
/// Panics if `/sys/class/dmi/id/board_name` cannot be read
/// (e.g., if the file does not exist or the system does not provide DMI info).
///
/// # Examples
///
/// ```rust
/// use ffetch::get_board_name;
///
/// let name = get_board_name();
/// println!("Motherboard name: {}", name);
/// // Output: Motherboard name: MS-7D95
/// ```
pub fn get_board_name() -> Result<String, Error> {
    let content = read_to_string("/sys/class/dmi/id/board_name").map_err(|e| {
        Error::new(
            e.kind(),
            "Failed to read /sys/class/dmi/id/board_name. File may not exist or permission denied.",
        )
    })?;

    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "The board_name file is empty.",
        ));
    }

    Ok(trimmed.to_string())
}

/// Gets the motherboard version from `/sys/class/dmi/id/board_version`.
///
/// This function reads the mainboard version information from the
/// sysfs DMI interface provided by the Linux kernel.
///
/// # Returns
///
/// Returns a `String` containing the motherboard version  
/// (e.g., `"1.0"`, `"Rev X.0"`, `"Not Specified"`).
///
/// # Panics
///
/// Panics if `/sys/class/dmi/id/board_version` cannot be read
/// (e.g., if the file does not exist or the system does not provide DMI info).
///
/// # Examples
///
/// ```rust
/// use ffetch::get_board_ver;
///
/// let version = get_board_ver();
/// println!("Motherboard version: {}", version);
/// // Output: Motherboard version: 1.0
/// ```
pub fn get_board_ver() -> Result<String, Error> {
    let content = read_to_string("/sys/class/dmi/id/board_version")
        .map_err(|e| Error::new(e.kind(), "Failed to read /sys/class/dmi/id/board_version. File may not exist or permission denied."))?;

    let trimmed = content.trim();
    if trimmed.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "The board_version file is empty.",
        ));
    }

    Ok(trimmed.to_string())
}

/// Gets the system init system
/// # Returns
///
/// Returns a `String` containing the init system name (e.g., "runit-init").
///
/// # Panics
///
/// A specific panic setting was not added, it could be because it cannot find file if it is panic.
///
/// # Examples
///
/// ```rust
/// use ffetch::get_init_system();
/// let init_system = get_init_system();
/// println!("Init: {}", init_system);
/// // Output: Init: runit-init
/// ```
pub fn get_init_system() -> Result<String, Error> {
    let path = read_link("/sbin/init")
        .map_err(|e| Error::new(e.kind(), "Failed to resolve /sbin/init symlink."))?;

    let init = path
        .to_str()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "Symlink target is not valid UTF-8."))?;

    if metadata("/sbin/openrc").is_ok() {
        return Ok("openrc".to_string());
    }

    let init_system = match init {
        "runit-init" => "runit",
        "/lib/systemd/systemd" => "systemd",
        _ => init,
    };

    Ok(init_system.to_string())
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
pub fn get_cpu_name() -> Result<String, Error> {
    let contents = read_to_string("/proc/cpuinfo").map_err(|e| {
        Error::new(
            e.kind(),
            "Failed to read /proc/cpuinfo. Are you running Linux?",
        )
    })?;

    for line in contents.lines() {
        if line.starts_with("model name") {
            if let Some(cpu) = line.split(':').nth(1) {
                let name = cpu.trim();
                if !name.is_empty() {
                    return Ok(name.to_string());
                }
            }
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "Could not find 'model name' in /proc/cpuinfo.",
    ))
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
pub fn get_memory() -> Result<String, Error> {
    let contents = read_to_string("/proc/meminfo")
        .map_err(|e| Error::new(e.kind(), "Failed to read /proc/meminfo."))?;

    let mut total_memory: Option<u64> = None;
    let mut free_memory: Option<u64> = None;

    for line in contents.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value_part = parts[1].trim();

        if key == "MemTotal" {
            if let Some(value_kb) = value_part.strip_suffix("kB") {
                if let Ok(val) = value_kb.trim().parse::<u64>() {
                    total_memory = Some(val / 1024); // Convert to MB
                }
            }
        }

        if key == "MemAvailable" {
            if let Some(value_kb) = value_part.strip_suffix("kB") {
                if let Ok(val) = value_kb.trim().parse::<u64>() {
                    free_memory = Some(val / 1024); // Convert to MB
                }
            }
        }

        if total_memory.is_some() && free_memory.is_some() {
            break;
        }
    }

    let total = total_memory.ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidData,
            "Failed to find MemTotal in /proc/meminfo.",
        )
    })?;
    let free = free_memory.ok_or_else(|| {
        Error::new(
            ErrorKind::InvalidData,
            "Failed to find MemAvailable in /proc/meminfo.",
        )
    })?;

    let used = total - free;
    Ok(format!("{used} / {total} MB"))
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
pub fn get_os_name() -> Result<String, Error> {
    let contents = read_to_string("/etc/os-release").map_err(|e| {
        Error::new(
            e.kind(),
            "Failed to read /etc/os-release. Are you running Linux?",
        )
    })?;

    for line in contents.lines() {
        if line.starts_with("NAME=") {
            let name = line.trim_start_matches("NAME=").trim_matches('"').trim();

            if name.is_empty() {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "OS name is empty in /etc/os-release.",
                ));
            }

            return Ok(name.to_string());
        }
    }

    Err(Error::new(
        ErrorKind::NotFound,
        "OS name (NAME=) not found in /etc/os-release.",
    ))
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
pub fn get_hostname() -> Result<String, Error> {
    let contents = read_to_string("/etc/hostname").map_err(|e| {
        Error::new(
            e.kind(),
            "Failed to read /etc/hostname. Are you running Linux?",
        )
    })?;

    let first_line = contents
        .lines()
        .next()
        .ok_or_else(|| Error::new(ErrorKind::InvalidData, "/etc/hostname is empty."))?;

    Ok(first_line.to_string())
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
pub fn get_cpu_arch() -> Result<String, Error> {
    let output = Command::new("uname")
        .arg("-m")
        .output()
        .map_err(|e| Error::new(e.kind(), "Failed to execute 'uname -m' command"))?;

    if !output.status.success() {
        return Err(Error::other("'uname -m' command exited with error"));
    }

    let arch = String::from_utf8(output.stdout)
        .map_err(|_| {
            Error::new(
                ErrorKind::InvalidData,
                "Failed to parse output of 'uname -m' as UTF-8",
            )
        })?
        .trim()
        .to_string();

    Ok(arch)
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
pub fn get_uptime() -> Result<String, Error> {
    let output = Command::new("uptime")
        .arg("-p")
        .output()
        .map_err(|e| Error::new(e.kind(), "Failed to execute 'uptime -p' command"))?;

    if !output.status.success() {
        return Err(Error::other("'uptime -p' command exited with error"));
    }

    let stdout = String::from_utf8(output.stdout).map_err(|_| {
        Error::new(
            ErrorKind::InvalidData,
            "Failed to parse output of 'uptime -p' as UTF-8",
        )
    })?;

    Ok(stdout.trim_start_matches("up ").trim().to_string())
}

/// Gets the number of installed packages from various package managers.
///
/// Detects and counts packages from multiple package managers including:
/// Portage, Flatpak, APT, DNF, YUM, Pacman, Zypper, Nix, and XBPS.
///
/// # Notes
/// This function runs external commands (`rpm -qa`) only for RPM-based
/// package managers (DNF, YUM, Zypper). Other managers use direct
/// file/directory reading.
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
pub fn get_packages() -> Result<String, Error> {
    fn count_emerge() -> Result<usize, Error> {
        let mut count = 0;
        let entries = read_dir("/var/db/pkg")
            .map_err(|e| Error::new(e.kind(), "Failed to read /var/db/pkg directory"))?;

        for entry in entries {
            let entry =
                entry.map_err(|e| Error::new(e.kind(), "Failed to read entry in /var/db/pkg"))?;
            if entry.path().is_dir() {
                let pkgs = read_dir(entry.path()).map_err(|e| {
                    Error::new(
                        e.kind(),
                        "Failed to read package directory under /var/db/pkg",
                    )
                })?;
                for pkg in pkgs {
                    let pkg =
                        pkg.map_err(|e| Error::new(e.kind(), "Failed to read package entry"))?;
                    if pkg.path().is_dir() {
                        count += 1;
                    }
                }
            }
        }
        Ok(count)
    }

    fn count_flatpak() -> Result<usize, Error> {
        let apps = read_dir("/var/lib/flatpak/app")
            .map_err(|e| Error::new(e.kind(), "Failed to read /var/lib/flatpak/app"))?
            .count();

        let runtimes = read_dir("/var/lib/flatpak/runtime")
            .map_err(|e| Error::new(e.kind(), "Failed to read /var/lib/flatpak/runtime"))?
            .count();

        Ok(apps + runtimes)
    }

    fn count_dpkg() -> Result<usize, Error> {
        let file = File::open("/var/lib/dpkg/status")
            .map_err(|e| Error::new(e.kind(), "Failed to open /var/lib/dpkg/status"))?;
        let reader = BufReader::new(file);

        let count = reader
            .lines()
            .filter_map(|l| l.ok())
            .filter(|l| l.starts_with("Package: "))
            .count();

        Ok(count)
    }

    fn count_pacman() -> Result<usize, Error> {
        let count = read_dir("/var/lib/pacman/local")
            .map_err(|e| Error::new(e.kind(), "Failed to read /var/lib/pacman/local"))?
            .count();

        Ok(count)
    }

    fn count_nix() -> Result<usize, Error> {
        let count = read_dir("/nix/store")
            .map_err(|e| Error::new(e.kind(), "Failed to read /nix/store"))?
            .count();

        Ok(count)
    }

    fn count_xbps() -> Result<usize, Error> {
        let path = "/var/db/xbps";
        let entries =
            read_dir(path).map_err(|e| Error::new(e.kind(), format!("Failed to read {}", path)))?;

        let count = entries
            .filter_map(|e| e.ok())
            .filter(|entry| {
                entry
                    .path()
                    .extension()
                    .map(|ext| ext == "plist")
                    .unwrap_or(false)
            })
            .count();

        Ok(count)
    }

    fn count_rpm() -> Result<usize, Error> {
        let output = Command::new("rpm")
            .arg("-qa")
            .output()
            .map_err(|e| Error::new(e.kind(), "Failed to execute 'rpm -qa' command"))?;

        if !output.status.success() {
            return Err(Error::other("'rpm -qa' command failed"));
        }

        if output.stdout.is_empty() {
            return Err(Error::other("'rpm -qa' returned empty output"));
        }

        let count = String::from_utf8(output.stdout)
            .map_err(|_| {
                Error::new(
                    ErrorKind::InvalidData,
                    "Failed to parse rpm output as UTF-8",
                )
            })?
            .lines()
            .count();

        Ok(count)
    }

    let backends: &[(&str, fn() -> Result<usize, Error>)] = &[
        ("emerge", count_emerge),
        ("flatpak", count_flatpak),
        ("apt-get", count_dpkg),
        ("pacman", count_pacman),
        ("nix", count_nix),
        ("xbps", count_xbps),
    ];

    let mut results = Vec::new();

    for (name, func) in backends {
        if let Ok(count) = func() {
            if count > 0 {
                results.push(format!("{count} ({name})"));
            }
        }
    }

    let rpm_managers = ["dnf", "yum", "zypper"];
    match count_rpm() {
        Ok(count) if count > 0 => {
            for name in &rpm_managers {
                results.push(format!("{count} ({name})"));
            }
        }
        _ => {}
    }

    Ok(results.join(" "))
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
    fn mm_to_inches(mm: u8) -> f32 {
        (mm as f32) / 25.4
    }

    fn parse_edid(edid_bytes: &[u8]) -> Option<(String, String, f32, f32, f32)> {
        if edid_bytes.len() < 128 {
            return None;
        }

        let width_mm = edid_bytes[21];
        let height_mm = edid_bytes[22];

        let mut model_name = "Unknown".to_string();
        for i in (54..=125).step_by(18) {
            if edid_bytes[i] == 0x00 && edid_bytes[i + 3] == 0xFC {
                let name_bytes = &edid_bytes[i + 5..i + 18];
                model_name = String::from_utf8_lossy(name_bytes)
                    .trim_end_matches(char::from(10))
                    .trim()
                    .to_string();
                break;
            }
        }

        let dtd = &edid_bytes[54..72];
        let pixel_clock = u16::from_le_bytes([dtd[0], dtd[1]]) as u32 * 10_000;

        let h_active = ((dtd[2] as u16) + (((dtd[4] & 0xF0) as u16) << 4)) as u32;
        let h_blank = ((dtd[3] as u16) + (((dtd[4] & 0x0F) as u16) << 8)) as u32;
        let v_active = ((dtd[5] as u16) + (((dtd[7] & 0xF0) as u16) << 4)) as u32;
        let v_blank = ((dtd[6] as u16) + (((dtd[7] & 0x0F) as u16) << 8)) as u32;

        let h_total = h_active + h_blank;
        let v_total = v_active + v_blank;

        if h_total == 0 || v_total == 0 {
            return None;
        }

        let refresh_rate = (pixel_clock as f64) / (h_total as f64 * v_total as f64);
        let refresh_rate_rounded = (refresh_rate * 100.0).round() / 100.0;

        Some((
            model_name,
            format!("{h_active}x{v_active}"),
            refresh_rate_rounded as f32,
            mm_to_inches(width_mm),
            mm_to_inches(height_mm),
        ))
    }

    fn parse_xrandr_monitors() -> Vec<String> {
        let output = Command::new("xrandr")
            .arg("--query")
            .output()
            .expect("Failed to execute xrandr");

        if !output.status.success() {
            return vec!["Failed to run xrandr".to_string()];
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut lines = stdout.lines();

        let mut results = Vec::new();

        while let Some(line) = lines.next() {
            if line.contains(" connected") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let connector_full = parts[0];
                let resolution_part = parts
                    .iter()
                    .find(|s| s.contains('+') && s.contains('x'))
                    .unwrap_or(&"Unknown");
                let connector_short = connector_full.split('-').next().unwrap_or("Unknown");
                let location = if connector_full.starts_with("eDP") {
                    "Internal"
                } else {
                    "External"
                };

                let resolution = resolution_part.split('+').next().unwrap_or("Unknown");

                let mut width_mm = 0.0;
                let mut height_mm = 0.0;

                for i in 0..parts.len() {
                    if parts[i].ends_with("mm") {
                        if width_mm == 0.0 {
                            width_mm = parts[i]
                                .trim_end_matches("mm")
                                .parse::<f32>()
                                .unwrap_or(0.0);
                        } else if height_mm == 0.0 {
                            height_mm = parts[i]
                                .trim_end_matches("mm")
                                .parse::<f32>()
                                .unwrap_or(0.0);
                            break;
                        }
                    }
                }

                let mut refresh_hz = "??".to_string();

                while let Some(mode_line) = lines.next() {
                    if mode_line.trim().is_empty() || mode_line.contains("connected") {
                        break;
                    }

                    let mode_parts: Vec<&str> = mode_line.trim().split_whitespace().collect();
                    if mode_parts.len() >= 2 && mode_parts[0] == resolution {
                        refresh_hz = mode_parts[1]
                            .trim_end_matches(|c: char| c == '*' || c == '+')
                            .to_string();
                        break;
                    }
                }

                let width_in = width_mm / 25.4;
                let height_in = height_mm / 25.4;
                let diagonal_in = if width_in > 0.0 && height_in > 0.0 {
                    ((width_in.powi(2) + height_in.powi(2)).sqrt() * 10.0).round() / 10.0
                } else {
                    0.0
                };

                let line = if diagonal_in > 0.0 {
                    format!(
                    "({connector_short}): {resolution} @ {refresh_hz} Hz in {diagonal_in:.1}\" [{location}]"
                )
                } else {
                    format!(
                    "({connector_short}): {resolution} @ {refresh_hz} Hz [No size] [{location}]"
                )
                };

                results.push(line);
            }
        }

        results
    }

    let drm_dir = Path::new("/sys/class/drm");
    let mut all_of_things = Vec::new();

    let entries = match read_dir(drm_dir) {
        Ok(e) => e,
        Err(_) => {
            all_of_things.push("Failed to read /sys/class/drm directory".to_string());
            return all_of_things[monitor_index].clone();
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => {
                all_of_things.push("Failed to read drm directory entry".to_string());
                continue;
            }
        };
        let path = entry.path();
        if !path.join("status").exists() {
            continue;
        }

        let status = match read_to_string(path.join("status")) {
            Ok(s) => s.trim().to_string(),
            Err(_) => {
                all_of_things.push(format!("Failed to read status for {}", path.display()));
                continue;
            }
        };

        if status != "connected" {
            continue;
        }

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown");
        let connector = name.split_once('-').map(|(_, c)| c).unwrap_or(name);

        let edid_path = path.join("edid");
        if !edid_path.exists() {
            all_of_things.push(format!("({connector}): Unknown (no EDID)"));
            continue;
        }

        let edid_bytes = match read(edid_path) {
            Ok(b) => b,
            Err(_) => {
                all_of_things.push(format!("Failed to read EDID for {connector}"));
                continue;
            }
        };

        if let Some((model, resolution, hz, width_in, height_in)) = parse_edid(&edid_bytes) {
            let size_diag = ((width_in.powi(2) + height_in.powi(2)).sqrt() * 10.0).round() / 10.0;

            let hz_str = if (hz - hz.round()).abs() < 0.01 {
                format!("{hz:.0}")
            } else {
                format!("{hz:.2}")
            };

            let location = if connector.starts_with("eDP") {
                "Internal"
            } else {
                "External"
            };

            let line =
                format!("({model}): {resolution} @ {hz_str} Hz in {size_diag:.1}\" [{location}]");

            all_of_things.push(line);
        } else {
            all_of_things.push(parse_xrandr_monitors()[monitor_index].to_string());
        }
    }

    all_of_things[monitor_index].clone()
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

    let window_id = stdout
        .split("window id # ")
        .nth(1)
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| {
            eprintln!("⚠️ Window not found.");
            "".to_string()
        });

    let output = Command::new("xprop")
        .args(["-id", &window_id])
        .output()
        .expect("xprop id run error.");

    let info = String::from_utf8_lossy(&output.stdout);
    for line in info.lines() {
        if line.starts_with("WM_CLASS") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start + 1..].find('"') {
                    let terminal = &line[start + 1..start + 1 + end];
                    return terminal.to_string();
                }
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

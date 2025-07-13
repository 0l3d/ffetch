# 🚀 **F-Fetch:** *Fast, Minimal & Rust-Powered System Fetcher* 🦀✨

F-Fetch is a **very fast**, **minimal** terminal system fetcher written **100% in Rust**.  
It is **highly customizable** — allowing you to add your own syntax and integrate your own features.  
At its core, it works simply and the codebase is easy to understand. 💡

---


| ⚙️ **Features**       | 📜 **Description**                                              |
|-----------------------|----------------------------------------------------------------|
| 🦀 **Rust**           | Written 100% in Rust                                            |
| 🎨 **Flexibility**    | Highly flexible — customize the appearance as you wish          |
| ⚡ **Efficiency**     | Despite flexibility, uses very little space and system resources |
| 🔧 **Customization** | Offers a wide range of customization options                    |

---

![voidimage](https://github.com/user-attachments/assets/336c3423-4d1b-40c6-92e7-8eb1b0ad0bd2)

---

# F-Fetch Components List

- `getUsername`  
  Returns the current **Username**.

- `getKernel`  
  Returns the **Kernel Version**.

- `getCpu`  
  Returns **CPU Information**.

- `getMemory`  
  Returns **Memory Usage**.

- `getHostname`  
  Returns the system **Hostname**.

- `getOsName`  
  Returns the **Operating System Name**.

- `getDesktop`  
  Returns the **Desktop Environment** or Window Manager.

- `getArch`  
  Returns the system **Architecture** (e.g., x86_64).

- `getPlatform`  
  Returns the system **Platform** information.

- `getUptime`  
  Returns the system **Uptime**.

- `getTerm`  
  Returns the current **Terminal Emulator**.

- `getGpu`  
  Returns the primary **GPU Information**.

- `getMGpu`  
  Returns the secondary **GPU Information** (if any).

- `getShell`  
  Returns the current **Shell**.

- `getDisk(mountpoint)`  
  Returns the **Disk Usage** information for the specified mount point.

- `getMonitor(monitorindex)`  
  Returns the **Monitor** information for the specified monitor index.
  
- `getPackages` (emerge, flatpak, apt, dnf, yum, pacman, zypper, nix-env, xbps-query)  
  Returns the **number of installed packages**.

- `getLocale`  
  Returns your system **locale** (e.g., `en_US`).
  
- `getTerm`  
  Returns your **terminal emulator**. 

- `getBackend`  
  Returns your **windowing system**.
  
- `getInit`  
  Returns your system **init** (e.g, `runit-init`).

- `getQT` or `getGTK`  
  Returns your theme from environment variables.

# F-Fetch Style ANSI Color Codes Reference

## 🎨 Foreground Colors (Text Colors)

| Placeholder | ANSI Code | Color | Description |
|-------------|-----------|-------|-------------|
| `fg.black` | `\x1b[30m` | ⚫ | Black text |
| `fg.red` | `\x1b[31m` | 🔴 | Red text |
| `fg.green` | `\x1b[32m` | 🟢 | Green text |
| `fg.yellow` | `\x1b[33m` | 🟡 | Yellow text |
| `fg.blue` | `\x1b[34m` | 🔵 | Blue text |
| `fg.magenta` | `\x1b[35m` | 🟣 | Magenta text |
| `fg.cyan` | `\x1b[36m` | 🔵 | Cyan text |
| `fg.white` | `\x1b[37m` | ⚪ | White text |

## ✨ Bright Foreground Colors

| Placeholder | ANSI Code | Color | Description |
|-------------|-----------|-------|-------------|
| `fg.bright_black` | `\x1b[90m` | ⚫ | Bright black (gray) text |
| `fg.bright_red` | `\x1b[91m` | 🔴 | Bright red text |
| `fg.bright_green` | `\x1b[92m` | 🟢 | Bright green text |
| `fg.bright_yellow` | `\x1b[93m` | 🟡 | Bright yellow text |
| `fg.bright_blue` | `\x1b[94m` | 🔵 | Bright blue text |
| `fg.bright_magenta` | `\x1b[95m` | 🟣 | Bright magenta text |
| `fg.bright_cyan` | `\x1b[96m` | 🔵 | Bright cyan text |
| `fg.bright_white` | `\x1b[97m` | ⚪ | Bright white text |

## 🎭 Background Colors

| Placeholder | ANSI Code | Color | Description |
|-------------|-----------|-------|-------------|
| `bg.black` | `\x1b[40m` | ⚫ | Black background |
| `bg.red` | `\x1b[41m` | 🔴 | Red background |
| `bg.green` | `\x1b[42m` | 🟢 | Green background |
| `bg.yellow` | `\x1b[43m` | 🟡 | Yellow background |
| `bg.blue` | `\x1b[44m` | 🔵 | Blue background |
| `bg.magenta` | `\x1b[45m` | 🟣 | Magenta background |
| `bg.cyan` | `\x1b[46m` | 🔵 | Cyan background |
| `bg.white` | `\x1b[47m` | ⚪ | White background |

## 🌟 Bright Background Colors

| Placeholder | ANSI Code | Color | Description |
|-------------|-----------|-------|-------------|
| `bg.bright_black` | `\x1b[100m` | ⚫ | Bright black (gray) background |
| `bg.bright_red` | `\x1b[101m` | 🔴 | Bright red background |
| `bg.bright_green` | `\x1b[102m` | 🟢 | Bright green background |
| `bg.bright_yellow` | `\x1b[103m` | 🟡 | Bright yellow background |
| `bg.bright_blue` | `\x1b[104m` | 🔵 | Bright blue background |
| `bg.bright_magenta` | `\x1b[105m` | 🟣 | Bright magenta background |
| `bg.bright_cyan` | `\x1b[106m` | 🔵 | Bright cyan background |
| `bg.bright_white` | `\x1b[107m` | ⚪ | Bright white background |

## 📝 Text Styles

| Placeholder | ANSI Code | Style | Description |
|-------------|-----------|-------|-------------|
| `t.bold` | `\x1b[1m` | **Bold** | Make text bold |
| `t.dim` | `\x1b[2m` | Dim | Make text dimmed |
| `t.italic` | `\x1b[3m` | *Italic* | Make text italic |
| `t.underline` | `\x1b[4m` | <u>Underline</u> | Underline text |
| `t.inverse` | `\x1b[7m` | Inverse | Swap foreground/background |
| `t.hidden` | `\x1b[8m` | Hidden | Hide text |
| `t.strike` | `\x1b[9m` | ~~Strike~~ | Strike through text |
| `t.bold_off` | `\x1b[21m` | Normal | Turn off bold |
| `t.underline_off` | `\x1b[24m` | Normal | Turn off underline |
| `t.inverse_off` | `\x1b[27m` | Normal | Turn off inverse |

## 🔄 Reset

| Placeholder | ANSI Code | Function | Description |
|-------------|-----------|----------|-------------|
| `all.reset` | `\x1b[0m` | Reset | Reset all formatting |


---
# 🛠️ **F-Fetch Configuration Examples**  
### (Advanced, Middle, Minimal)

```bash
# 🌟 Advanced config
echo "╭───────────── " t.underline t.bold fg.yellow getUsername
echo "│  " t.underline " System Information :"
echo "│    " fg.blue " OS: " fg.cyan getOsName " " t.italic fg.yellow t.bold getArch
echo "│    " fg.blue " Kernel: " fg.black getPlatform " " fg.green getKernel
echo "│    " fg.blue " Hostname: " fg.cyan getHostname
echo "│    " fg.blue " Packages: " fg.cyan getPackages
echo "│    " fg.blue " Locale: " fg.cyan getLocale
echo "│    "
echo "│  " t.underline "󰋊 Hardware Information :"
echo "│    "
echo "│    " fg.yellow " CPU: " fg.white t.bold getCpu
echo "│    " fg.yellow "󰍹 GPU: " fg.white t.bold getGpu
echo "│    " fg.yellow " Memory: " fg.white t.bold getMemory fg.yellow " MB"
echo "│    " fg.yellow " Root: " fg.white t.bold getDisk(/)
echo "│    "
echo "│  " t.underline " Desktop Information :"
echo "│    "
echo "│    " fg.magenta "󰪫 DE/WM: " fg.white t.bold getDesktop
echo "│    " fg.magenta " Uptime: " fg.white t.bold getUptime
echo "│    " fg.magenta "󰧨 Primary: " fg.white t.bold getMonitor(0)
echo "│    " fg.magenta " Shell: " fg.white t.bold getShell
echo "│    " fg.magenta " Terminal: " fg.white t.bold getTerm
echo "╰───────────────────────────────╯"

ascii = "/home/getUsername/.config/ffetch/ascii.txt"
ascii_color = "fg.cyan"
```

```bash
# ⚡ Middle config
echo t.bold fg.yellow getUsername fg.black "@" fg.yellow getHostname
echo fg.blue "Distro: " fg.yellow t.bold getOsName
echo fg.blue "Platform: " fg.yellow t.bold getPlatform
echo fg.blue "Kernel: " fg.yellow t.bold getKernel
echo fg.blue "Memory: " fg.yellow t.bold getMemory " MB"
echo fg.blue "CPU: " fg.yellow t.bold getCpu
echo fg.blue "GPU: " fg.yellow t.bold getGpu
echo fg.blue "Packages: " fg.yellow t.bold getPackages
echo fg.blue "Disk: " fg.yellow t.bold getDisk(/)
echo fg.blue "Desktop: " fg.yellow t.bold getDesktop
echo fg.blue "Primary: " fg.yellow t.bold getMonitor(0)
echo fg.blue "Uptime: " fg.yellow t.bold getUptime
echo fg.blue "Terminal: " fg.yellow t.bold getTerm
echo fg.blue "Shell: " fg.yellow t.bold getShell

ascii = "/home/getUsername/.config/ffetch/ascii.txt"
ascii_color = "fg.cyan"
```

```bash
# 🔥 Minimal config
echo getUsername "@" getHostname
echo "Distro: " getOsName
echo "Platform: " getPlatform
echo "Kernel: " getKernel
echo "Memory: " getMemory " MB"
echo "CPU: " getCpu
echo "GPU: "  getGpu
echo "Packages: " getPackages
echo "Disk: " getDisk(/)
echo "Desktop: " getDesktop
echo "Primary: " getMonitor(0)
echo "Terminal: " getTerm
echo "Uptime: "  getUptime
echo "Shell: " getShell

ascii = "/home/getUsername/.config/ffetch/ascii.txt"
ascii_color = "fg.cyan"

```

---
# ⚙️ **Installation & Configuration**

## 📦 **Install via Cargo**
```bash
# Install from crates.io
cargo install ffetch
```

## 🚀 **Installation Script**
```bash
# Run Installation Script
bash <(curl -s https://raw.githubusercontent.com/0l3d/ffetch/master/install.sh)
```

## 🏃 **Run F-Fetch**
```bash
ffetch
```

---

# 📚 **Use as Library**

You can also use F-Fetch as a library in your Rust projects:

## 📥 **Add to your project**
```bash
cargo add ffetch
```

Or add to your `Cargo.toml`:
```toml
[dependencies]
ffetch = "0.3.1"
```

## 🛠️ **Example Usage**
```rust
use ffetch::*;

fn main() {
    // Get system information
    let username = get_username();
    let os_name = get_os_name();
    let cpu_info = get_cpu();
    let memory = get_memory();
    
    println!("User: {}", username);
    println!("OS: {}", os_name);
    println!("CPU: {}", cpu_info);
    println!("Memory: {} MB", memory);
}
```
---

# ☢️ All Possible Problems and Solutions

You're clearly not on Linux:
``` text
you are not using linux
```

Failed to run uname — really? Make sure uname exists:
``` text
uname command error
```

Failed to run uptime — seriously? Check if uptime is available:
``` text
uptime command error
```

Couldn't detect your GPU with lspci. Are you using WSL or something else?
``` text
Failed to execute lspci, not found.
```

Check your ~/.config/ffetch/ffetch.conf file and make sure ascii="" is set correctly:
``` text
Failed to read file
```

Check the getMonitor() function in your configuration. If you're only using one monitor, either remove getMonitor(1) or add your third monitor.
``` text
index out of bound
```

If the disk is not showing up, check getDisk() in the config and make sure it's using a valid mountpoint, like getDisk(/).

---


## ☕ Support the project!  
If you like F-Fetch and want to support the development, consider buying me a coffee:

👉 [https://buymeacoffee.com/oled](https://buymeacoffee.com/oled)



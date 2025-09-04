# 🚀 **F-Fetch:** _Fast, Minimal & Rust-Powered System Fetcher_

F-Fetch is a **very fast**, **minimal** terminal system fetcher written **100% in Rust**.  
It is **highly customizable** - allowing you to add your own syntax and integrate your own features.  
At its core, it works simply and the codebase is easy to understand.

---

| ⚙️ **Features**      | 📜 **Description**                                               |
| -------------------- | ---------------------------------------------------------------- |
| 🦀 **Rust**          | Written 100% in Rust                                             |
| 🎨 **Flexibility**   | Highly flexible - customize the appearance as you wish           |
| ⚡ **Efficiency**    | Despite flexibility, uses very little space and system resources |
| 🔧 **Customization** | Offers a wide range of customization options                     |

---

<img width="1100" height="608" alt="image" src="https://github.com/user-attachments/assets/23fcc419-4c0b-4e3e-aeda-b557b45cb690" />

---

# ⚙️ **Installation & Configuration**

## For Arch Linux

### 📦 Install via AUR

```bash
paru -S ffetch
paru -S ffetch-git
```

### Install via `makepkg`

```bash
git clone --depth=1 https://github.com/0l3d/ffetch.git
cd ffetch
makepkg -si
```

## 📦 **Install via Cargo**

```bash
cargo install ffetch
```

## 🚀 **Installation Script**

```bash
bash <(curl -s https://raw.githubusercontent.com/0l3d/ffetch/master/install.sh)
```

## 🏃 **Run F-Fetch**

```bash
ffetch
```

---

# 📚 **Use as Library**

You can also use F-Fetch as a library in your Rust projects.

Check: [crates.io/ffetch](https://crates.io/crates/ffetch)  
Docs: [docs.rs/ffetch](https://docs.rs/ffetch/latest/ffetch/)

---

## Dependencies

If u're using `getTerm`, u need to make sure xprop is installed.
If `getMonitor` fails, it uses `xrandr`.

For GPU and disk information, I use standard Linux tools like lspci (from pciutils) and df (from coreutils).

# F-Fetch Configuration & Examples

## F-Fetch Components List

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

- `getBoardVendor`, `getBoardName` and `getBoardVersion`  
  Returns your system motherboard informations. (e.g, MSI)

## F-Fetch Style ANSI Color Codes Reference

### 🎨 Foreground Colors (Text Colors)

| Placeholder  | ANSI Code  | Color | Description  |
| ------------ | ---------- | ----- | ------------ |
| `fg.black`   | `\x1b[30m` | ⚫    | Black text   |
| `fg.red`     | `\x1b[31m` | 🔴    | Red text     |
| `fg.green`   | `\x1b[32m` | 🟢    | Green text   |
| `fg.yellow`  | `\x1b[33m` | 🟡    | Yellow text  |
| `fg.blue`    | `\x1b[34m` | 🔵    | Blue text    |
| `fg.magenta` | `\x1b[35m` | 🟣    | Magenta text |
| `fg.cyan`    | `\x1b[36m` | 🔵    | Cyan text    |
| `fg.white`   | `\x1b[37m` | ⚪    | White text   |

### ✨ Bright Foreground Colors

| Placeholder         | ANSI Code  | Color | Description              |
| ------------------- | ---------- | ----- | ------------------------ |
| `fg.bright_black`   | `\x1b[90m` | ⚫    | Bright black (gray) text |
| `fg.bright_red`     | `\x1b[91m` | 🔴    | Bright red text          |
| `fg.bright_green`   | `\x1b[92m` | 🟢    | Bright green text        |
| `fg.bright_yellow`  | `\x1b[93m` | 🟡    | Bright yellow text       |
| `fg.bright_blue`    | `\x1b[94m` | 🔵    | Bright blue text         |
| `fg.bright_magenta` | `\x1b[95m` | 🟣    | Bright magenta text      |
| `fg.bright_cyan`    | `\x1b[96m` | 🔵    | Bright cyan text         |
| `fg.bright_white`   | `\x1b[97m` | ⚪    | Bright white text        |

### 🎭 Background Colors

| Placeholder  | ANSI Code  | Color | Description        |
| ------------ | ---------- | ----- | ------------------ |
| `bg.black`   | `\x1b[40m` | ⚫    | Black background   |
| `bg.red`     | `\x1b[41m` | 🔴    | Red background     |
| `bg.green`   | `\x1b[42m` | 🟢    | Green background   |
| `bg.yellow`  | `\x1b[43m` | 🟡    | Yellow background  |
| `bg.blue`    | `\x1b[44m` | 🔵    | Blue background    |
| `bg.magenta` | `\x1b[45m` | 🟣    | Magenta background |
| `bg.cyan`    | `\x1b[46m` | 🔵    | Cyan background    |
| `bg.white`   | `\x1b[47m` | ⚪    | White background   |

### 🌟 Bright Background Colors

| Placeholder         | ANSI Code   | Color | Description                    |
| ------------------- | ----------- | ----- | ------------------------------ |
| `bg.bright_black`   | `\x1b[100m` | ⚫    | Bright black (gray) background |
| `bg.bright_red`     | `\x1b[101m` | 🔴    | Bright red background          |
| `bg.bright_green`   | `\x1b[102m` | 🟢    | Bright green background        |
| `bg.bright_yellow`  | `\x1b[103m` | 🟡    | Bright yellow background       |
| `bg.bright_blue`    | `\x1b[104m` | 🔵    | Bright blue background         |
| `bg.bright_magenta` | `\x1b[105m` | 🟣    | Bright magenta background      |
| `bg.bright_cyan`    | `\x1b[106m` | 🔵    | Bright cyan background         |
| `bg.bright_white`   | `\x1b[107m` | ⚪    | Bright white background        |

### 📝 Text Styles

| Placeholder       | ANSI Code  | Style            | Description                |
| ----------------- | ---------- | ---------------- | -------------------------- |
| `t.bold`          | `\x1b[1m`  | **Bold**         | Make text bold             |
| `t.dim`           | `\x1b[2m`  | Dim              | Make text dimmed           |
| `t.italic`        | `\x1b[3m`  | _Italic_         | Make text italic           |
| `t.underline`     | `\x1b[4m`  | <u>Underline</u> | Underline text             |
| `t.inverse`       | `\x1b[7m`  | Inverse          | Swap foreground/background |
| `t.hidden`        | `\x1b[8m`  | Hidden           | Hide text                  |
| `t.strike`        | `\x1b[9m`  | ~~Strike~~       | Strike through text        |
| `t.bold_off`      | `\x1b[21m` | Normal           | Turn off bold              |
| `t.underline_off` | `\x1b[24m` | Normal           | Turn off underline         |
| `t.inverse_off`   | `\x1b[27m` | Normal           | Turn off inverse           |

### 🔄 Reset

| Placeholder | ANSI Code | Function | Description          |
| ----------- | --------- | -------- | -------------------- |
| `all.reset` | `\x1b[0m` | Reset    | Reset all formatting |

---

## 🛠️ **F-Fetch Configuration Examples**

### (Advanced, Middle, Minimal)

```bash
# 🌟 Advanced config
echo "╭───────────── " t.underline t.bold fg.yellow getUsername all.reset " ─────────────────────────╮"
echo "│  " t.underline fg.bright_red " System Information :"
echo "│    " fg.white t.bold " OS: " all.reset fg.green getOsName " " t.italic fg.yellow t.bold getArch
echo "│    " fg.white t.bold "󰇅 Host: " all.reset fg.green getBoardName " " (getBoardVersion)
echo "│    " fg.white t.bold " Kernel: " all.reset fg.green getKernel
echo "│    " fg.white t.bold " Hostname: " all.reset fg.green getHostname
echo "│    " fg.white t.bold " Packages: " all.reset fg.green getPackages
echo "│    " fg.white t.bold " Locale: " all.reset fg.green getLocale
echo "│    " fg.white t.bold " Init: " all.reset fg.green getInit
echo "│    "
echo "│  " t.underline fg.bright_green "󰋊 Hardware Information :"
echo "│    "
echo "│    " fg.yellow t.bold " CPU: " all.reset fg.white getCpu
echo "│    " fg.yellow t.bold "󰍹 GPU: " all.reset fg.white getGpu
echo "│    " fg.yellow t.bold " Memory: " all.reset fg.white getMemory fg.yellow " MB"
echo "│    " fg.yellow t.bold " Root: " all.reset fg.white getDisk(/)
echo "│    "
echo "│  " t.underline fg.bright_blue " Desktop Information :"
echo "│    "
echo "│    " fg.magenta t.bold "󰪫 DE/WM: " all.reset fg.white getDesktop " " (getBackend)
echo "│    " fg.magenta t.bold " Uptime: " all.reset fg.white getUptime
echo "│    " fg.magenta t.bold "󰧨 Primary: " all.reset fg.white getMonitor(0)
echo "│    " fg.magenta t.bold " Shell: " all.reset fg.white getShell
echo "╰──────────────────────────────────────────────╯"

ascii = "/home/getUsername/.config/ffetch/ascii.txt"
ascii_color = "fg.green"

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

## ☕ Support the project

If you like F-Fetch and want to support the development, consider buying me a coffee:

👉 [https://buymeacoffee.com/oled](https://buymeacoffee.com/oled)

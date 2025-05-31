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

![image](https://github.com/user-attachments/assets/acc883e8-abe3-4cf8-838d-68425747939d)

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
  
- `getPackages` (Not yet available)  
  Would return the number of installed packages.


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
echo "│    "
echo "│  " t.underline "󰋊 Hardware Information :"
echo "│    "
echo "│    " fg.yellow " CPU: " fg.white t.bold getCpu
echo "│    " fg.yellow "󰍹 GPU: " fg.white t.bold getGpu
echo "│    " fg.yellow " Memory: " fg.white t.bold getMemory fg.yellow " MB"
echo "│    " fg.yellow " Root: " fg.white t.bold getDisk(/)
echo "│    " fg.yellow " Sata: " fg.white t.bold getDisk(/mnt/sata)
echo "│    "
echo "│  " t.underline " Desktop Information :"
echo "│    "
echo "│    " fg.magenta "󰪫 DE/WM: " fg.white t.bold getDesktop
echo "│    " fg.magenta " Uptime: " fg.white t.bold getUptime
echo "│    " fg.magenta "󰧨 Primary: " fg.white t.bold getMonitor(0)
echo "│    " fg.magenta "󱡶 Secondary: " fg.white t.bold getMonitor(1)
echo "│    " fg.magenta " Shell: " fg.white t.bold getShell
echo "╰───────────────────────────────╯"

ascii = "/home/username/.config/ffetch/ascii.txt"
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
echo fg.blue "Disk: " fg.yellow t.bold getDisk(/)
echo fg.blue "Desktop: " fg.yellow t.bold getDesktop
echo fg.blue "Primary: " fg.yellow t.bold getMonitor(0)
echo fg.blue "Uptime: " fg.yellow t.bold getUptime
echo fg.blue "Shell: " fg.yellow t.bold getShell

ascii = "ascii_text.txt"
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
echo "Disk: " getDisk(/)
echo "Desktop: " getDesktop
echo "Primary: " getMonitor(0)
echo "Uptime: "  getUptime
echo "Shell: " getShell

ascii = "ascii_text.txt"
ascii_color = "fg.cyan"
```
---
# ⚙️ **Installation & Configuration**

```bash
# Clone the repo and build:
git clone https://github.com/username/ffetch.git
cd ffetch
cargo build --release
sudo cp target/release/ffetch /usr/local/bin/

# Setup configuration:
mkdir -p ~/.config/ffetch
mv ffetch-yourconfig.conf ~/.config/ffetch/ffetch.conf
mv ascii.txt ~/.config/ffetch/ascii.txt

# Run F-Fetch:
ffetch
```

## ☕ Support the project!  
If you like F-Fetch and want to support the development, consider buying me a coffee:

👉 [https://buymeacoffee.com/oled](https://buymeacoffee.com/oled)

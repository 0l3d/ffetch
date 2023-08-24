# F-Fetch
F-Fetch targets low systems. Written in Rust. It's very simple, designed so you can pick it up and replace it.

## First Look 

~/.config/ffetch/config.yaml
```yml
ascii_path : "/home/user/.config/ffetch/ascii_arts/debian.txt"
#all components : user.host,platform,os.name,memory,cpu,uptime,user.name,host.name,kernel.version,de,packages
components : "user.host,platform,os.name,memory,cpu,uptime,packages"
```
Supported distros : Debian, Fedora and Arch Linux


Example
```sh
name@hostname
Platform :          Linux
OS Name :           Arch Linux
Kernel Version :    kernel
Memory :            1000 / 4096 MB
CPU :               Intel | x86_64
Uptime :            1 hours, 25 minutes
```

* If you are using wm, it will say Unknown in the desktop section.

It is far from colors and visuals! Focused on function only!

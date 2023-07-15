# F-Fetch
F-Fetch targets low systems. Written in Rust. It's very simple, designed so you can pick it up and replace it.

## First Look 

config.yaml
```yml
ascii_path : /home/sh/.config/ffetch/ascii.txt
components : "user.host,platform,os.name,memory,cpu,uptime"
```

output
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

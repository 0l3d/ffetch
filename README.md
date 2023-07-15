# F-Fetch
F-Fetch targets low systems. Written in Rust. It's very simple, designed so you can pick it up and replace it.

## First Look 

config.rs
```rs
mod core;

use crate::config::core::ffetch;

pub fn get_ascii() -> String {
    return format!("        

                {}@{}
 ▄       ▄      Platform :          {}
▄ ▀▄   ▄▀ ▄     OS Name :           {}
█▄█▀███▀█▄█     Kernel Version :    {}
▀█████████▀     Memory :            {} MB
 ▄▀     ▀▄      CPU :               {} | {}
                Uptime :            {}
                    ",
        ffetch::get_username(),
        ffetch::get_hostname(),
        ffetch::get_platform(),
        ffetch::get_os_name(),
        ffetch::get_kernel_version(),
        ffetch::get_memory(),
        ffetch::get_cpu_name(),
        ffetch::get_cpu_arch(),
        ffetch::get_uptime()
    );
}
```

output
```sh
                name@hostname
 ▄       ▄      Platform :          Linux
▄ ▀▄   ▄▀ ▄     OS Name :           "Arch Linux"
█▄█▀███▀█▄█     Kernel Version :    kernel
▀█████████▀     Memory :            1000 / 4096 MB
 ▄▀     ▀▄      CPU :               Intel | x86_64
                Uptime :            1 hours, 25 minutes
```

* If you are using wm, it will say Unknown in the desktop section.

It is far from colors and visuals! Focused on function only!

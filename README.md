# F-Fetch
F-Fetch targets low systems. Written in Rust. It's very simple, designed so you can pick it up and replace it.

## First Look 

config.rs
```rs
mod core;

use crate::config::core::ffetch;

pub fn get_ascii() -> String {
    return format!("        
        ----------------------------------------------------
        |                F-FETCH
        | USERNAME : {}@{}
        | PLATFORM : {}
        | OS NAME : {}
        | KERNEL VERSION : {}
        | DESKTOP ENV : {}
        | MEMORY : {} MB
        | CPU : {} | {}
        |                                        
        ---------------------------------------------------- ",
        ffetch::get_username(),
        ffetch::get_hostname(),
        ffetch::get_platform(),
        ffetch::get_os_name(),
        ffetch::get_kernel_version(),
        ffetch::get_desktop_env(),
        ffetch::get_memory(),
        ffetch::get_cpu_name(),
        ffetch::get_cpu_arch()
    );
}
```

output
```sh
        ----------------------------------------------------
        |                F-FETCH
        | USERNAME : sh@sh
        | PLATFORM : Linux
        | OS NAME : "Distro Name"
        | KERNEL VERSION : 6.4.2-arch1-1
        | DESKTOP ENV : Unknown: bspwm *
        | MEMORY : 7078 / 12441 MB
        | CPU : Intel bla bla | x86_64
        |                                        
        ---------------------------------------------------- 
```

* If you are using wm, it will say Unknown in the desktop section.

It is far from colors and visuals! Focused on function only!

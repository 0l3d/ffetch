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
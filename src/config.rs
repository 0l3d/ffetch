mod core;
use std::fs::read_to_string;

use crate::config::core::ffetch;

use serde_yaml;
use std::collections::BTreeMap;

pub fn get_ascii() -> String {

    let yaml_code = read_to_string(format!("/home/{}/.config/ffetch/config.yml", ffetch::get_username())).expect(
        "Could not open file."
    );
    let map: BTreeMap<String, String> = serde_yaml::from_str(&yaml_code).expect("Deserialize error : ");

    //let ascii_art: String = read_to_string(&map["ascii_path"]).expect("Read ascii error : ").split("\t").collect();

    let config_map : Vec<&str> = map["components"].split(',').collect();

    let mut config = String::new();

    for mut components in 0..config_map.len() {      
        match config_map[components] {
            "user.host" => config += &format!("{}@{}\n", ffetch::get_username(), ffetch::get_username()),
            "platform" => config += &format!("Platform :          {}\n", ffetch::get_platform()),
            "os.name" => config += &format!("OS Name :           {}\n", ffetch::get_os_name()),
            "memory" => config += &format!("Memory :            {} MB\n", ffetch::get_memory()),
            "cpu" => config += &format!("CPU :               {} | {}\n", ffetch::get_cpu_name(), ffetch::get_cpu_arch()),
            "uptime" => config += &format!("Uptime :            {}\n", ffetch::get_uptime()),
            "user.name" => config += &format!("User Name :         {}\n", ffetch::get_username()),
            "host.name" => config += &format!("Host Name :         {}\n", ffetch::get_hostname()),
            "de" => config += &format!("DE :                {}\n", ffetch::get_desktop_env()),
            "kernel.version" => config += &format!("Kernel Version :    {}\n", ffetch::get_kernel_version()),
            _ => ()
        };
        components += 1;
    };
    return config.to_string();
    /*
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
*/
}

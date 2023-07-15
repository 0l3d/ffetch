mod core;
use std::fs::read_to_string;

use crate::config::core::ffetch;

use serde_yaml;
use std::collections::BTreeMap;

pub fn get_ascii() -> String {

    let config_keywords = [
        "user.host",
        "platform",
        "os.name",
        "memory",
        "cpu",
        "uptime",
        "user.name",
        "host.name",
        "de",
        "kernel.version",
    ];


    let yaml_code = read_to_string(format!("/home/{}/.config/ffetch/config.yml", ffetch::get_username())).expect(
        "Could not open file."
    );
    let map: BTreeMap<String, String> = serde_yaml::from_str(&yaml_code).expect("Deserialize error : ");

    //let ascii_art: String = read_to_string(&map["ascii_path"]).expect("Read ascii error : ").split("\t").collect();

    let config_map : Vec<&str> = map["components"].split(",").collect();

    let mut config : String = "".to_string();

    for mut components in 0..config_map.len() {
        if config_map[components] == config_keywords[0] {
            config += &format!("{}@{}\n", ffetch::get_username(), ffetch::get_username());
        } else if config_map[components] == config_keywords[1] {
            config += &format!("Platform :      {}\n", ffetch::get_platform());
        }else if config_map[components] == config_keywords[2] {
            config += &format!("OS Name :       {}\n", ffetch::get_os_name());
        }else if config_map[components] == config_keywords[3] {
            config += &format!("Memory :        {}\n", ffetch::get_memory());
        } else if config_map[components] == config_keywords[4] {
            config += &format!("CPU :           {} | {}\n", ffetch::get_cpu_name(), ffetch::get_cpu_arch());
        }else if config_map[components] == config_keywords[5] {
            config += &format!("Uptime :        {}\n", ffetch::get_uptime());
        }else if config_map[components] == config_keywords[6] {
            config += &format!("Username :      {}\n", ffetch::get_username());
        }else if config_map[components] == config_keywords[7] {
            config += &format!("Hostname :      {}\n", ffetch::get_hostname());
        }else if config_map[components] == config_keywords[8] {
            config += &format!("DE :            {}\n", ffetch::get_desktop_env());
        }else if config_map[components] == config_keywords[9] {
            config += &format!("Kernel Version :{}\n", ffetch::get_kernel_version());
        }
        components += 1;
    };
    return format!("{}", config.to_string());
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

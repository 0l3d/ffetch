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
    
    let mut ascii_vec = Vec::new();

    for line in read_to_string(&map["ascii_path"]).expect("Read ascii error : ").lines() {
        ascii_vec.push(line.to_string());
    }

    let config_map : Vec<&str> = map["components"].split(',').collect();

    let mut config = String::new();
    
    let mut for_end : usize = 0;
    let mut data = String::new();
    let mut data_var = String::new();
   
    for mut trims in 0..ascii_vec[0].len() {
        data_var += &" ".to_string();
        trims += 1;
    }

    for mut components in 0..config_map.len() { 
        
        if for_end == ascii_vec.len() {
            data = data_var.to_string();
        }  else {
            data = ascii_vec[for_end].to_string();
            for_end += 1;
        }



        match config_map[components] {
            "user.host" => config += &format!("{}{}@{}\n",data,ffetch::get_username(), ffetch::get_hostname()),
            "platform" => config += &format!("{}Platform :          {}\n", data, ffetch::get_platform()),
            "os.name" => config += &format!("{}OS Name :           {}\n", data,ffetch::get_os_name()),
            "memory" => config += &format!("{}Memory :            {} MB\n", data,ffetch::get_memory()),
            "cpu" => config += &format!("{}CPU :               {} | {}\n", data,ffetch::get_cpu_name(), ffetch::get_cpu_arch()),
            "uptime" => config += &format!("{}Uptime :            {}\n", data,ffetch::get_uptime()),
            "user.name" => config += &format!("{}User Name :         {}\n", data,ffetch::get_username()),
            "host.name" => config += &format!("{}Host Name :         {}\n", data,ffetch::get_hostname()),
            "de" => config += &format!("{}DE :                {}\n", data,ffetch::get_desktop_env()),
            "kernel.version" => config += &format!("{}Kernel Version :    {}\n", data,ffetch::get_kernel_version()),
            "packages" => config += &format!("{}Packages :          {}\n", data,ffetch::get_packages()),
            "gpu" => config += &format!("{}GPU :               {}\n", data,ffetch::get_gpu()),
            "shell" => config += &format!("{}Shell :             {}\n", data,ffetch::get_shell()),
            _ => ()
        };
        components += 1;
    };

    if for_end >= config_map.len() {
        for ascii_end in for_end..ascii_vec.len() {
            config += &format!("{}\n", ascii_vec[ascii_end]);
        }
    }

    return config.to_string();
}

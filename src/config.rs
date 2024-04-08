mod core;
use std::fs::read_to_string;

use crate::config::core::ffetch;

use serde_yaml;
use std::collections::BTreeMap;

use termion::color;

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
    let ascii_color: Vec<&str> = map["ascii_color"].split(';').collect();
    let color_map : Vec<&str> = map["colors"].split(',').collect();

    let mut config = String::new();
    
    let mut for_end : usize = 0;
    let mut data = String::new();
    let mut data_var = String::new();

    let mut color_string : String = "".to_string();

    let mut ascii_color_string : String = "".to_string();
   
    for mut trims in 0..ascii_vec[0].len() {
        data_var += &" ".to_string();
        trims += 1;
    }

    match ascii_color[0] {
        "c.red" => ascii_color_string = format!("{}", color::Fg(color::Red)),
        "c.green" => ascii_color_string = format!("{}", color::Fg(color::Green)),
        "c.yellow" => ascii_color_string = format!("{}", color::Fg(color::Yellow)),
        "c.blue" => ascii_color_string = format!("{}", color::Fg(color::Blue)),
        "c.magenta" => ascii_color_string = format!("{}", color::Fg(color::Magenta)),
        "c.cyan" => ascii_color_string = format!("{}", color::Fg(color::Cyan)),
        "c.white" => ascii_color_string = format!("{}", color::Fg(color::White)),
        "c.black" => ascii_color_string = format!("{}", color::Fg(color::Black)),
        "nocolor" => ascii_color_string = "".to_string(),
        _ => ()
    }

    for mut components in 0..config_map.len() { 
        
        if for_end == ascii_vec.len() {
            data = data_var.to_string();
        }  else {
            data = ascii_vec[for_end].to_string();
            for_end += 1;
        }

        match color_map[components] {
            "c.red" => color_string = format!("{}", color::Fg(color::Red)),
            "c.green" => color_string = format!("{}", color::Fg(color::Green)),
            "c.yellow" => color_string = format!("{}", color::Fg(color::Yellow)),
            "c.blue" => color_string = format!("{}", color::Fg(color::Blue)),
            "c.magenta" => color_string = format!("{}", color::Fg(color::Magenta)),
            "c.cyan" => color_string = format!("{}", color::Fg(color::Cyan)),
            "c.white" => color_string = format!("{}", color::Fg(color::White)),
            "c.black" => ascii_color_string = format!("{}", color::Fg(color::Black)),
            "nocolor" => color_string = "".to_string(),
            _ => ()
        }


        match config_map[components] {
            "user.host" => config += &format!("{}{}{}{}{}@{}{}\n",ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_username(), ffetch::get_hostname(), color::Fg(color::Reset)),
            "platform" => config += &format!("{}{}{}{}Platform :          {}{}\n",ascii_color_string,data, color::Fg(color::Reset),color_string,ffetch::get_platform(),color::Fg(color::Reset)),
            "os.name" => config += &format!("{}{}{}{}OS Name :           {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_os_name(),color::Fg(color::Reset)),
            "memory" => config += &format!("{}{}{}{}Memory :            {} MB{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_memory(),color::Fg(color::Reset)),
            "cpu" => config += &format!("{}{}{}{}CPU :               {} | {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_cpu_name(), ffetch::get_cpu_arch(),color::Fg(color::Reset)),
            "uptime" => config += &format!("{}{}{}{}Uptime :            {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_uptime(),color::Fg(color::Reset)),
            "user.name" => config += &format!("{}{}{}{}User Name :         {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_username(),color::Fg(color::Reset)),
            "host.name" => config += &format!("{}{}{}{}Host Name :         {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_hostname(),color::Fg(color::Reset)),
            "de" => config += &format!("{}{}{}{}DE :                {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_desktop_env(),color::Fg(color::Reset)),
            "kernel.version" => config += &format!("{}{}{}{}Kernel Version :    {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_kernel_version(),color::Fg(color::Reset)),
            "packages" => config += &format!("{}{}{}{}Packages :          {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_packages(),color::Fg(color::Reset)),
            "gpu" => config += &format!("{}{}{}{}GPU :               {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_gpu(),color::Fg(color::Reset)),
            "mgpu" => config += &format!("{}{}{}{}GPU :               {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_m_gpu(),color::Fg(color::Reset)),
            "shell" => config += &format!("{}{}{}{}Shell :             {}{}\n", ascii_color_string,data,color::Fg(color::Reset),color_string,ffetch::get_shell(),color::Fg(color::Reset)),
            _ => ()
        };
        components += 1;
    };

    if for_end >= config_map.len() {
        for ascii_end in for_end..ascii_vec.len() {
            config += &format!("{}{}{}\n", ascii_color_string,ascii_vec[ascii_end], color::Fg(color::Reset),);
        }
    }

    return config.to_string();
}

pub mod ffetch;
use std::fs;

use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use regex::Regex;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();
    for line in fs::read_to_string(filename)
        .expect("Failed to read file.")
        .lines()
    {
        result.push(line.to_string())
    }
    result
}

lazy_static! {
    static ref DISK_REGEX: Regex = Regex::new(r"getDisk\((.*?)\)").expect("Regex error:");
    static ref MON_REGEX: Regex = Regex::new(r"getMonitor\((.*?)\)").expect("Regex error:");
    static ref USERNAME: String = ffetch::ffetch::get_username();
    static ref KERNEL: String = ffetch::ffetch::get_kernel_version();
    static ref CPU: String = ffetch::ffetch::get_cpu_name();
    static ref MEMORY: String = ffetch::ffetch::get_memory();
    static ref HOSTNAME: String = ffetch::ffetch::get_hostname();
    static ref OSNAME: String = ffetch::ffetch::get_os_name();
    static ref DESKTOP: String = ffetch::ffetch::get_desktop_env();
    static ref LOCALE: String = ffetch::ffetch::get_locale();
    static ref ARCH: String = ffetch::ffetch::get_cpu_arch();
    static ref PLATFORM: String = ffetch::ffetch::get_platform();
    static ref UPTIME: String = ffetch::ffetch::get_uptime();
    static ref SHELL: String = ffetch::ffetch::get_shell();
    static ref GPU: String = ffetch::ffetch::get_gpu();
    static ref mGPU: String = ffetch::ffetch::get_m_gpu();
    static ref PACKAGES: String = ffetch::ffetch::get_packages();
    static ref PATH: String = format!(
        "/home/{}/.config/ffetch/ffetch.conf",
        ffetch::ffetch::get_username()
    );
}

static CONTENTS: Lazy<Vec<String>> = Lazy::new(|| {
    fs::read_to_string(&*PATH)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.to_string())
        .collect()
});

fn comp_with_disk_argument(input: &str) -> String {
    if let Some(caps) = DISK_REGEX.captures(input) {
        return caps.get(1).unwrap().as_str().to_string();
    }
    String::new()
}

fn comp_with_mon_argument(input: &str) -> usize {
    if let Some(caps) = MON_REGEX.captures(input) {
        let matched_str = caps.get(1).unwrap().as_str();
        return matched_str.parse::<usize>().unwrap_or(0);
    }
    0
}

fn find_token(string: &str, findstr: &str) -> bool {
    let tokens: &Vec<&str> = &string.split(" ").collect();
    for i in 0..tokens.len() {
        if tokens[i] == findstr {
            return true;
        }
    }
    false
}

fn replace_syntax(conf: &str) -> String {
    let disk = comp_with_disk_argument(conf);
    let diskf = format!("getDisk({})", disk);

    let monitor = comp_with_mon_argument(conf);
    let monitorf = format!("getMonitor({})", monitor);
    let index: usize = monitor;

    let replaced_conf: String = conf
        // strings
        .replace('"', "")
        .replace("getUsername", &USERNAME)
        .replace("getKernel", &KERNEL)
        .replace("getCpu", &CPU)
        .replace("getMemory", &MEMORY)
        .replace("getHostname", &HOSTNAME)
        .replace("getOsName", &OSNAME)
        .replace("getDesktop", &DESKTOP)
        .replace("getArch", &ARCH)
        .replace("getPlatform", &PLATFORM)
        .replace("getUptime", &UPTIME)
        .replace("getPackages", &PACKAGES)
        .replace("getGpu", &GPU)
        .replace("getMGpu", &mGPU)
        .replace("getShell", &SHELL)
        .replace("getLocale", &LOCALE)
        .replace(&diskf, &ffetch::ffetch::get_disks(&disk))
        .replace(&monitorf, &ffetch::ffetch::get_monitor(index))
        // Fg Colors
        .replace("fg.black", "\x1b[30m")
        .replace("fg.red", "\x1b[31m")
        .replace("fg.green", "\x1b[32m")
        .replace("fg.yellow", "\x1b[33m")
        .replace("fg.blue", "\x1b[34m")
        .replace("fg.magenta", "\x1b[35m")
        .replace("fg.cyan", "\x1b[36m")
        .replace("fg.white", "\x1b[37m")
        .replace("fg.bright_black", "\x1b[90m")
        .replace("fg.bright_red", "\x1b[91m")
        .replace("fg.bright_green", "\x1b[92m")
        .replace("fg.bright_yellow", "\x1b[93m")
        .replace("fg.bright_blue", "\x1b[94m")
        .replace("fg.bright _magenta", "\x1b[95m")
        .replace("fg.bright_cyan", "\x1b[96m")
        .replace("fg.bright_white", "\x1b[97m")
        // Bg Colors
        .replace("bg.black", "\x1b[40m")
        .replace("bg.red", "\x1b[41m")
        .replace("bg.green", "\x1b[42m")
        .replace("bg.yellow", "\x1b[43m")
        .replace("bg.blue", "\x1b[44m")
        .replace("bg.magenta", "\x1b[45m")
        .replace("bg.cyan", "\x1b[46m")
        .replace("bg.white", "\x1b[47m")
        .replace("bg.bright_black", "\x1b[100m")
        .replace("bg.bright_red", "\x1b[101m")
        .replace("bg.bright_green", "\x1b[102m")
        .replace("bg.bright_yellow", "\x1b[103m")
        .replace("bg.bright_blue", "\x1b[104m")
        .replace("bg.bright_magenta", "\x1b[105m")
        .replace("bg.bright_cyan", "\x1b[106m")
        .replace("bg.bright_white", "\x1b[107m")
        // Text Styles
        .replace("t.bold", "\x1b[1m")
        .replace("t.dim", "\x1b[2m")
        .replace("t.italic", "\x1b[3m")
        .replace("t.underline", "\x1b[4m")
        .replace("t.inverse", "\x1b[7m")
        .replace("t.hidden", "\x1b[8m")
        .replace("t.strike", "\x1b[9m")
        .replace("t.bold_off", "\x1b[21m")
        .replace("t.underline_off", "\x1b[24m")
        .replace("t.inverse_off", "\x1b[27m")
        .replace("all.reset", "\x1b[0m");

    return replaced_conf;
}

fn find_char(string: &str, target: char) -> bool {
    for char in string.chars() {
        if char == target {
            return true;
        }
    }
    false
}

fn get_option(token: &str) -> String {
    let mut output: String = String::new();
    for i in 0..CONTENTS.len() {
        let tokens = &CONTENTS[i];
        if find_char(tokens, '=') {
            let tokenizer: &Vec<&str> = &tokens.split("=").collect();
            let tokena: &Vec<&str> = &tokenizer[0].split(" ").collect();
            if tokena[0] == token && find_char(tokens, '"') {
                let mut newsplt: String = String::new();
                for tk in 1..tokenizer.len() {
                    newsplt += &tokenizer[tk];
                    output = newsplt.split('"').collect();
                    output.replace_range(0..1, "");
                    return output;
                }
            }
        } else {
            continue;
        }
    }
    output
}

fn lex_config(input: &str) -> String {
    let input = input.trim_start();
    let input = if input.starts_with("echo ") {
        input[5..].trim_start()
    } else {
        input
    };
    let mut result = String::new();
    let mut in_string = false;
    let mut current = String::new();
    for c in input.chars() {
        match c {
            '"' => {
                current.push(c);
                in_string = !in_string;
                if !in_string {
                    result.push_str(&current);
                    current.clear();
                }
            }
            ' ' if !in_string => {
                if !current.is_empty() {
                    result.push_str(&current);
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        result.push_str(&current);
    }

    result
}

fn write_fetch(ascii: Vec<String>, ascii_color: String) -> String {
    let max_width = ascii.iter().map(|line| line.len()).max().unwrap_or(0);
    let mut ascii_index = 0;
    for i in 0..CONTENTS.len() {
        let tokens = &CONTENTS[i];
        let lexed_conf: String = lex_config(&tokens);
        let replaced_conf = replace_syntax(&lexed_conf);

        if find_token(tokens, "echo") {
            let end_conf = format!(
                "{}{:<width$}\x1b[0m    {}\x1b[0m",
                replace_syntax(&ascii_color),
                ascii.get(ascii_index).unwrap_or(&"".to_string()),
                replaced_conf,
                width = max_width
            );
            ascii_index += 1;
            println!("{}", end_conf);
        }
    }

    while ascii_index < ascii.len() {
        let line = &ascii[ascii_index];
        println!(
            "{}{:<width$}",
            replace_syntax(&ascii_color),
            line,
            width = max_width
        );
        ascii_index += 1;
    }
    "".to_string()
}

fn main() {
    let ascii_text = get_option("ascii");
    let ascii = read_lines(&ascii_text);
    let ascii_color = get_option("ascii_color");
    write_fetch(ascii, ascii_color);
}

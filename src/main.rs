pub mod ffetch;
use std::{fs, io::Write, path::Path};

fn read_lines(filename: &str) -> Vec<String> {
    let conf_path = Path::new(filename);
    if !conf_path.exists() {
        if let Some(parent) = conf_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Parent directory create error.");
            }
        }

        let mut file = fs::File::create(filename).expect("file create error.");
        println!("Default configuration file generated.");
        println!("For configuration docs or example configs: \nhttps://github.com/0l3d/ffetch");
        let content = r#"
fg.black           #####
fg.black          ##fg.white###fg.black##
fg.black          #fg.white#fg.blackO#Ofg.white#fg.black#
fg.black          #fg.white#####fg.black#
fg.black        ##fg.white#######fg.black##
fg.black       ##fg.white#########fg.black##
fg.black      ##fg.white###########fg.black##
fg.black      ##fg.white############fg.black##
fg.black     ##fg.white#############fg.black##
fg.black   ##fg.white#################fg.black##
fg.black   ##fg.white#################fg.black##
fg.black    ###################
"#;
        file.write_all(content.as_bytes())
            .expect("file write error.");
    }

    let mut result = Vec::new();
    for line in fs::read_to_string(filename)
        .expect("Failed to read file.")
        .lines()
    {
        result.push(line.to_string())
    }
    result
}

fn get_config_path() -> String {
    format!(
        "/home/{}/.config/ffetch/ffetch.conf",
        ffetch::get_username()
    )
}

fn get_contents() -> Vec<String> {
    let path = get_config_path();
    let conf_path = Path::new(&path);

    if !conf_path.exists() {
        if let Some(parent) = conf_path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).expect("Parent directory create error.");
            }
        }

        let mut file = fs::File::create(&path).expect("file create error.");
        let content = r#"
# middle config
echo t.bold fg.yellow getUsername fg.black "@" fg.yellow getHostname
echo fg.blue "Distro: " fg.yellow t.bold getOsName
echo fg.blue "Platform: " fg.yellow t.bold getPlatform
echo fg.blue "Kernel: " fg.yellow t.bold getKernel
echo fg.blue "Memory: " fg.yellow t.bold getMemory " MB"
echo fg.blue "CPU: " fg.yellow t.bold getCpu
echo fg.blue "GPU: " fg.yellow t.bold getGpu
echo fg.blue "Packages: " fg.yellow t.bold getPackages
echo fg.blue "Disk: " fg.yellow t.bold getDisk(/)
echo fg.blue "Desktop: " fg.yellow t.bold getDesktop
echo fg.blue "Primary: " fg.yellow t.bold getMonitor(0)
echo fg.blue "Uptime: " fg.yellow t.bold getUptime
echo fg.blue "Shell: " fg.yellow t.bold getShell

ascii = "/home/getUsername/.config/ffetch/ascii.txt"
ascii_color = "fg.cyan"
"#;
        file.write_all(content.as_bytes())
            .expect("file write error.");
    }

    fs::read_to_string(&path)
        .expect("Failed to read file")
        .lines()
        .map(|s| s.to_string())
        .collect()
}

// LEXER (PERFORMANCE FOCUSED)
fn lex_string(s: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_str = false;

    for c in s.chars() {
        if c == '"' {
            if in_str {
                tokens.push(current.clone());
                current.clear();
                in_str = false;
            } else {
                in_str = true;
            }
            continue;
        } else if c == '#' {
            break;
        } else if (c == '(' || c == ')') && !in_str {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
            tokens.push(c.to_string());
        } else if c.is_whitespace() && !in_str {
            if !current.is_empty() {
                tokens.push(current.clone());
                current.clear();
            }
        } else {
            current.push(c);
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

fn parse_ansi_code(token: &str) -> &str {
    match token {
        "fg.black" => "\x1b[30m",
        "fg.red" => "\x1b[31m",
        "fg.green" => "\x1b[32m",
        "fg.yellow" => "\x1b[33m",
        "fg.blue" => "\x1b[34m",
        "fg.magenta" => "\x1b[35m",
        "fg.cyan" => "\x1b[36m",
        "fg.white" => "\x1b[37m",
        "fg.bright_black" => "\x1b[90m",
        "fg.bright_red" => "\x1b[91m",
        "fg.bright_green" => "\x1b[92m",
        "fg.bright_yellow" => "\x1b[93m",
        "fg.bright_blue" => "\x1b[94m",
        "fg.bright_magenta" => "\x1b[95m",
        "fg.bright_cyan" => "\x1b[96m",
        "fg.bright_white" => "\x1b[97m",
        "bg.black" => "\x1b[40m",
        "bg.red" => "\x1b[41m",
        "bg.green" => "\x1b[42m",
        "bg.yellow" => "\x1b[43m",
        "bg.blue" => "\x1b[44m",
        "bg.magenta" => "\x1b[45m",
        "bg.cyan" => "\x1b[46m",
        "bg.white" => "\x1b[47m",
        "bg.bright_black" => "\x1b[100m",
        "bg.bright_red" => "\x1b[101m",
        "bg.bright_green" => "\x1b[102m",
        "bg.bright_yellow" => "\x1b[103m",
        "bg.bright_blue" => "\x1b[104m",
        "bg.bright_magenta" => "\x1b[105m",
        "bg.bright_cyan" => "\x1b[106m",
        "bg.bright_white" => "\x1b[107m",
        "t.bold" => "\x1b[1m",
        "t.dim" => "\x1b[2m",
        "t.italic" => "\x1b[3m",
        "t.underline" => "\x1b[4m",
        "t.inverse" => "\x1b[7m",
        "t.hidden" => "\x1b[8m",
        "t.strike" => "\x1b[9m",
        "t.bold_off" => "\x1b[21m",
        "t.underline_off" => "\x1b[24m",
        "t.inverse_off" => "\x1b[27m",
        "all.reset" => "\x1b[0m",

        _ => "",
    }
}

fn parser(tokens: Vec<String>) -> String {
    let mut return_val: String = String::new();
    let mut i = 0;

    while i < tokens.len() {
        let token = &tokens[i];

        let ansi = parse_ansi_code(token);
        if !ansi.is_empty() {
            return_val.push_str(ansi);
            i += 1;
            continue;
        }

        match token.as_str() {
            "getUsername" => return_val.push_str(&ffetch::get_username()),
            "getBoardVendor" => match ffetch::get_board_vendor() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getBoardName" => match ffetch::get_board_name() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getBoardVersion" => match ffetch::get_board_ver() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getOsName" => match ffetch::get_os_name() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getArch" => match ffetch::get_cpu_arch() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getKernel" => match ffetch::get_kernel_version() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getCpu" => match ffetch::get_cpu_name() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getMemory" => match ffetch::get_memory() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getHostname" => match ffetch::get_hostname() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getDesktop" => return_val.push_str(&ffetch::get_desktop_env()),
            "getBackend" => return_val.push_str(&ffetch::get_compositor()),
            "getPlatform" => return_val.push_str(&ffetch::get_platform()),
            "getUptime" => match ffetch::get_uptime() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getPackages" => match ffetch::get_packages() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },
            "getInit" => match ffetch::get_init_system() {
                Ok(s) => return_val.push_str(&s),
                Err(e) => return_val.push_str(&format!("Error: {e}")),
            },

            "getGpu" => return_val.push_str(&ffetch::get_gpu()),
            "getMGpu" => return_val.push_str(&ffetch::get_m_gpu()),
            "getShell" => return_val.push_str(&ffetch::get_shell()),
            "getLocale" => return_val.push_str(&ffetch::get_locale()),
            "getTerm" => return_val.push_str(&ffetch::get_terminal()),
            "getGTK" => return_val.push_str(&ffetch::gtk_theme()),
            "getQT" => return_val.push_str(&ffetch::qt_theme()),
            "getMonitor" => {
                if i + 3 < tokens.len() && tokens[i + 1] == "(" && tokens[i + 3] == ")" {
                    let argstr = &tokens[i + 2];
                    let arg = argstr.parse().unwrap();
                    return_val.push_str(&ffetch::get_monitor(arg));
                    i += 3;
                } else {
                    return_val.push_str("Invalid getMonitor usage");
                }
            }
            "getDisk" => {
                if i + 3 < tokens.len() && tokens[i + 1] == "(" && tokens[i + 3] == ")" {
                    let argstr = &tokens[i + 2];
                    return_val.push_str(&ffetch::get_disks(argstr));
                    i += 3;
                } else {
                    return_val.push_str("Invalid getDiskusage");
                }
            }
            "echo" => {}
            _ => return_val.push_str(token),
        }
        i += 1;
    }

    return_val
}

fn find_token(string: &str, findstr: &str) -> bool {
    let tokens: Vec<&str> = string.split(" ").collect();
    for item in &tokens {
        if item == &findstr {
            return true;
        }
    }
    false
}

fn get_option(token: &str) -> String {
    for line in get_contents().iter() {
        let tokens = lex_string(line);

        if tokens.len() >= 3 && tokens[0] == token && tokens[1] == "=" {
            let mut value = tokens[2].clone();

            if value.contains("getUsername") {
                value = value.replace("getUsername", &ffetch::get_username());
            }

            return value;
        }
    }

    String::new()
}

fn ascii_colorizer_lexer(string: &str) -> Vec<String> {
    let color_tokens = [
        "fg.black",
        "fg.red",
        "fg.green",
        "fg.yellow",
        "fg.blue",
        "fg.magenta",
        "fg.cyan",
        "fg.white",
        "fg.bright_black",
        "fg.bright_red",
        "fg.bright_green",
        "fg.bright_yellow",
        "fg.bright_blue",
        "fg.bright_magenta",
        "fg.bright_cyan",
        "fg.bright_white",
        "bg.black",
        "bg.red",
        "bg.green",
        "bg.yellow",
        "bg.blue",
        "bg.magenta",
        "bg.cyan",
        "bg.white",
        "bg.bright_black",
        "bg.bright_red",
        "bg.bright_green",
        "bg.bright_yellow",
        "bg.bright_blue",
        "bg.bright_magenta",
        "bg.bright_cyan",
        "bg.bright_white",
        "t.bold",
        "t.dim",
        "t.italic",
        "t.underline",
        "t.inverse",
        "t.hidden",
        "t.strike",
        "t.bold_off",
        "t.underline_off",
        "t.inverse_off",
        "all.reset",
    ];
    let mut tokens = Vec::new();
    let mut i = 0;
    let input_chars: Vec<char> = string.chars().collect();

    while i < input_chars.len() {
        let mut matched = false;

        for &color in &color_tokens {
            if string[i..].starts_with(color) {
                tokens.push(color.to_string());
                i += color.len();
                matched = true;
                break;
            }
        }

        if !matched {
            let start = i;
            while i < input_chars.len() {
                let mut found_color = false;
                for &color in &color_tokens {
                    if string[i..].starts_with(color) {
                        found_color = true;
                        break;
                    }
                }
                if found_color {
                    break;
                }
                i += 1;
            }

            let normal_text: String = input_chars[start..i].iter().collect();
            tokens.push(normal_text);
        }
    }
    tokens
}

fn colorize_ascii(ascii: Vec<String>) -> Vec<String> {
    let mut new_ascii: Vec<String> = Vec::new();

    for asci in ascii.iter() {
        let tokens = ascii_colorizer_lexer(asci);

        let mut stringer: String = String::new();
        for token in tokens.iter() {
            let ansi = parse_ansi_code(token);
            if !ansi.is_empty() {
                stringer.push_str(ansi);
                continue;
            }
            stringer.push_str(token);
        }
        new_ascii.push(stringer);
    }
    new_ascii
}

fn clean_ansi_len(s: &str) -> usize {
    let mut visible_len = 0;
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&next_c) = chars.peek() {
                    chars.next();
                    if next_c == 'm' {
                        break;
                    }
                }
                continue;
            }
        }

        visible_len += 1;
    }

    visible_len
}

fn pad_ansi_line(line: &str, target_width: usize) -> String {
    let visible = clean_ansi_len(line);
    if visible >= target_width {
        line.to_string()
    } else {
        let padding = " ".repeat(target_width - visible);
        format!("{}{}", line, padding)
    }
}

// TODO: Image Support.



fn write_fetch(ascii: Vec<String>) -> String {
    let max_width = ascii
        .iter()
        .map(|line| clean_ansi_len(line))
        .max()
        .unwrap_or(0);
    let mut ascii_index = 0;
    for i in 0..get_contents().len() {
        let tokens = &get_contents()[i];
        let lexed_conf: Vec<String> = lex_string(tokens);
        let replaced_conf = parser(lexed_conf);

        if find_token(tokens, "echo") {
            let ascii_line = ascii
                .get(ascii_index)
                .cloned()
                .unwrap_or_else(|| "".to_string());
            let padded = pad_ansi_line(&ascii_line, max_width);
            let end_conf = format!("{}\x1b[0m   {}\x1b[0m", padded, replaced_conf);
            println!("{end_conf}");
            ascii_index += 1;
        }
    }

    while ascii_index < ascii.len() {
        let line = &ascii[ascii_index];
        println!("{}", pad_ansi_line(line, max_width));
        ascii_index += 1;
    }
    "".to_string()
}

fn main() {
    let ascii_text = get_option("ascii");
    let ascii_raw = read_lines(&ascii_text);
    let ascii = colorize_ascii(ascii_raw);
    write_fetch(ascii);
}

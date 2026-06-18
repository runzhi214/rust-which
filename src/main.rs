use std::env;
use std::path::Path;
use std::process;

// 默认的可执行扩展名列表（当 PATHEXT 环境变量不存在时使用）
const DEFAULT_PATHEXT: &str = ".COM;.EXE;.BAT;.CMD;.VBS;.VBE;.JS;.JSE;.WSF;.WSH;.MSC";

fn main() {
    let args: Vec<String> = env::args().collect();

    // 无参数时直接退出（行为同 GNU which）
    if args.len() < 2 {
        process::exit(0);
    }

    // 显示帮助
    if args[1] == "--help" || args[1] == "-h" {
        print_help();
        process::exit(0);
    }

    let path_var = env::var("PATH").unwrap_or_default();
    let pathext_var = env::var("PATHEXT").unwrap_or_else(|_| DEFAULT_PATHEXT.to_string());

    let mut all_found = true;

    // 遍历所有要查找的命令
    for cmd in &args[1..] {
        match find_command(cmd, &path_var, &pathext_var) {
            Some(full_path) => println!("{}", full_path),
            None => {
                all_found = false;
                // 静默跳过，不输出错误信息（同 GNU which）
            }
        }
    }

    if !all_found {
        process::exit(1);
    }
}

fn print_help() {
    println!("Usage: which [options] command...");
    println!("Locates the executable file associated with the given command.");
    println!();
    println!("Options:");
    println!("  --help, -h  Show this help message");
}

/// 在 PATH 或直接路径中查找命令
fn find_command(cmd: &str, path_var: &str, pathext_var: &str) -> Option<String> {
    // 如果命令包含路径分隔符，当作直接路径处理
    if cmd.contains('/') || cmd.contains('\\') {
        return find_direct(cmd, pathext_var);
    }

    let exts = parse_pathext(pathext_var);
    // 判断命令是否已自带扩展名（例如 notepad.exe）
    let has_ext = has_extension(cmd);

    // 遍历 PATH 中的每个目录
    for dir in path_var.split(';').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        let base = Path::new(dir);

        if has_ext {
            // 已有扩展名，只尝试原样匹配
            let candidate = base.join(cmd);
            if candidate.is_file() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        } else {
            // 无扩展名，按 PATHEXT 顺序尝试添加
            for ext in &exts {
                let fname = format!("{}{}", cmd, ext);
                let candidate = base.join(&fname);
                if candidate.is_file() {
                    return Some(candidate.to_string_lossy().into_owned());
                }
            }
        }
    }

    None
}

/// 处理直接路径模式（含 \ 或 / 的命令）
fn find_direct(cmd: &str, pathext_var: &str) -> Option<String> {
    let path = Path::new(cmd);

    // 检查是否已有非空扩展名（例如 .\foo.exe）
    let has_ext = path.extension().map_or(false, |e| !e.is_empty());

    if has_ext {
        if path.is_file() {
            return Some(cmd.to_string());
        }
    } else {
        let exts = parse_pathext(pathext_var);
        for ext in &exts {
            let fname = format!("{}{}", cmd, ext);
            if Path::new(&fname).is_file() {
                return Some(fname);
            }
        }
    }

    None
}

/// 解析 PATHEXT 环境变量，返回扩展名列表（如 ".exe", ".bat"）
fn parse_pathext(pathext_var: &str) -> Vec<String> {
    pathext_var
        .split(';')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}

/// 判断命令名是否已包含扩展名（例如 "notepad.exe" 返回 true，"notepad" 返回 false）
fn has_extension(cmd: &str) -> bool {
    // 如果包含点且点不是最后一个字符，则认为有扩展名
    if let Some(dot_pos) = cmd.rfind('.') {
        dot_pos < cmd.len() - 1 // 点后有字符
    } else {
        false
    }
}
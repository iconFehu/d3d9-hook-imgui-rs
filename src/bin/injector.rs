//! DLL 注入器示例程序
//! 
//! 使用方法: cargo run --bin injector -- <进程名>

use hudhook::inject::Process;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("使用方法: {} <进程名.exe>", args[0]);
        eprintln!("示例: {} YourGame.exe", args[0]);
        std::process::exit(1);
    }
    
    let process_name = &args[1];
    
    // 获取 DLL 路径（假设 DLL 在 target/release 目录下）
    let mut dll_path = env::current_exe()
        .expect("无法获取当前可执行文件路径");
    
    // 从 bin/injector.exe 转到 target/release/d3d9_hook_imgui_rs.dll
    dll_path.pop(); // 移除 injector.exe
    dll_path.pop(); // 移除 bin
    dll_path.push("release");
    dll_path.push("d3d9_hook_imgui_rs.dll");
    
    if !dll_path.exists() {
        eprintln!("错误: 找不到 DLL 文件: {:?}", dll_path);
        eprintln!("请先运行: cargo build --release");
        std::process::exit(1);
    }
    
    println!("正在查找进程: {}", process_name);
    
    match Process::by_name(process_name) {
        Ok(process) => {
            println!("找到进程，正在注入 DLL...");
            match process.inject(dll_path) {
                Ok(_) => {
                    println!("✅ DLL 注入成功!");
                    println!("按 INSERT 键切换菜单显示/隐藏");
                }
                Err(e) => {
                    eprintln!("❌ 注入失败: {:?}", e);
                    eprintln!("提示: 可能需要管理员权限");
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 找不到进程 '{}': {:?}", process_name, e);
            eprintln!("提示: 确保进程正在运行，并且名称正确");
            std::process::exit(1);
        }
    }
}


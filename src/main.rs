// Gitlet 驱动类，Git 版本控制系统的一个子集
// @author TODO

use std::env;

// 用法: cargo run -- ARGS, 其中 ARGS 包含
// <COMMAND> <OPERAND1> <OPERAND2> ...
fn main() {
    let args: Vec<String> = env::args().collect();
    
    // TODO: 如果 args 为空怎么办?
    if args.len() < 2 {
        eprintln!("请提供至少一个命令");
        return;
    }
    
    let first_arg = &args[1];
    match first_arg.as_str() {
        "init" => {
            // TODO: 处理 `init` 命令
        }
        "add" => {
            // TODO: 处理 `add [filename]` 命令
        }
        // TODO: 填写其余部分
        _ => {
            eprintln!("未知命令: {}", first_arg);
        }
    }
}

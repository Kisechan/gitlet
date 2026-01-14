// Gitlet 驱动类，Git 版本控制系统的一个子集
// @author Kisechan

use std::env;

use gitlet::repository::Repository;
use gitlet::commit::Commit;

// 用法: cargo run -- ARGS, 其中 ARGS 包含
// <COMMAND> <OPERAND1> <OPERAND2> ...
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Please enter a command.");
        return;
    }
    
    let repo = Repository::new();
    
    let first_arg = &args[1];
    match first_arg.as_str() {
        "init" => {
            if args.len() != 2 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }

            if repo.exists() {
                eprintln!("A Gitlet version-control system already exists in the current directory.");
                std::process::exit(0);
            }

            repo.init_dirs();
            let initial_commit = Commit::initial();
            repo.save_commit(&initial_commit);
            repo.create_branch("master", &initial_commit.get_id().as_str());
            repo.set_head("refs/heads/master");
        }
        "add" => {
            if args.len() != 3 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                std::process::exit(0);
            }
            let filename = &args[2];
            repo.add_file(filename);
        }
        "commit" => {
            if args.len() != 3 {
                eprintln!("Please enter a commit message.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            let message = &args[2];
            repo.commit(message);
        }
        "rm" => {
            if args.len() != 3 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            let filename = &args[2];
            repo.rm_file(filename);
        }
        "log" => {
            if args.len() != 2 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            repo.log();
        }
        "global-log" => {
            if args.len() != 2 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            repo.global_log();
        }
        _ => {
            eprintln!("No command with that name exists.");
        }
    }
}

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
        "find" => {
            if args.len() != 3 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            let message = &args[2];
            repo.find(message);
        }
        "status" => {
            if args.len() != 2 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            repo.status();
        }
        "checkout" => {
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            match args.len() {
                3 => {
                    let branch_name = &args[2];
                    repo.checkout_branch(branch_name);
                }
                4 => {
                    if args[2] != "--" {
                        eprintln!("Incorrect operands.");
                        std::process::exit(0);
                    }
                    let filename = &args[3];
                    repo.checkout_file(filename);
                }
                5 => {
                    if args[3] != "--" {
                        eprintln!("Incorrect operands.");
                        std::process::exit(0);
                    }
                    let commit_id = &args[2];
                    let filename = &args[4];
                    repo.checkout_file_from_commit(commit_id, filename);
                    }
                _ => {
                    eprintln!("Incorrect operands.");
                    std::process::exit(0);
                }
            }
        }
        "branch" => {
            if args.len() != 3 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            let branch_name = &args[2];
            repo.branch(branch_name);
        }
        "rm-branch" => {
            if args.len() != 3 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            let branch_name = &args[2];
            repo.rm_branch(branch_name);
        }
        "reset" => {
            if args.len() != 3 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            let commit_id = &args[2];
            repo.reset(commit_id);
        }
        "merge" => {
            if args.len() != 3 {
                eprintln!("Incorrect operands.");
                std::process::exit(0);
            }
            if !repo.exists() {
                eprintln!("Not in an initialized Gitlet directory.");
                std::process::exit(0);
            }
            let branch_name = &args[2];
            repo.merge(branch_name);
        }
        _ => {
            eprintln!("No command with that name exists.");
        }
    }
}

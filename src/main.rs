mod tools;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;

#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
struct App {
    #[arg(long)]
    no_default: bool,

    #[arg(long, default_value = None)]
    tool_file: Option<PathBuf>,

    #[command(subcommand)]
    sub_cmd: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    #[command(alias = "ls")]
    List,
    #[command(alias = "f")]
    Find,
    Add,
    Remove,
}

fn main() -> anyhow::Result<()> {
    let app = App::parse();
    match app.sub_cmd {
        SubCommands::List => {
            let (default_tools, user_tools) = tools::tools(app.tool_file)?;
            if !user_tools.actions().is_empty() {
                println!("{}", "User defined tools".yellow());
                print_tools(user_tools);
            } else {
                println!(
                    "{}",
                    "No user defined tool file found at ~/.tools.toml".red()
                );
            }
            println!();
            println!("{}", "Default tools".yellow());
            println!();
            print_tools(default_tools);
        }
        SubCommands::Find => todo!(),
        SubCommands::Add => todo!(),
        SubCommands::Remove => todo!(),
    }
    Ok(())
}

fn print_tools(tools: tools::Tools) {
    for action in tools.actions() {
        println!("{}", action.action().blue());
        let (installed, not_installed) = system_tools(action.tools());
        if installed.is_empty() {
            println!("{}", "No tools installed matching this action".red())
        } else {
            println!("{}", "Installed tools".green());
            for tool in installed {
                println!("- {}", tool);
            }
        }
        println!();
        if !not_installed.is_empty() {
            println!("{}", "Other tools to install".red());
            for tool in not_installed {
                println!("- {}", tool);
            }
            println!();
        }
    }
}

fn exists(tool: &str) -> bool {
    which::which(tool).is_ok()
}

fn system_tools(tools: &Vec<String>) -> (Vec<&String>, Vec<&String>) {
    let installed_tools = tools.iter().filter(|tool| exists(tool)).collect::<Vec<_>>();
    let not_installed_tools = tools
        .iter()
        .filter(|tool| !exists(tool))
        .collect::<Vec<_>>();
    (installed_tools, not_installed_tools)
}

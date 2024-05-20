mod tools;

use std::path::PathBuf;

use clap::{Parser, Subcommand};
use colored::Colorize;
use tools::Action;

/// `tools` is a simple utility to remember one of the installed cli tools that may have gone forgotten
#[derive(Debug, Parser)]
#[clap(version, about, long_about = None)]
struct App {
    /// Do not include the default tools in the search
    #[arg(long)]
    no_default: bool,

    /// Use another tool file which is not the default one at '~/.tools.toml'
    #[arg(long, default_value = None)]
    tool_file: Option<PathBuf>,

    #[command(subcommand)]
    sub_cmd: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    /// List all actions and their tools
    #[command(alias = "ls")]
    List,

    /// Find tools of the action matching the given query. The query is not case sensitive.
    #[command(alias = "f")]
    Find { query: String },
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
        SubCommands::Find { query } => {
            let matched_tools = find_tool(query.as_str(), app.tool_file)?;
            if matched_tools.is_empty() {
                println!(
                    "{}",
                    format!("No action(s) found matching query: '{}'", query).red()
                );
            } else {
                println!(
                    "{}",
                    format!("Found following tools matching query: '{}'", query).green()
                );
                for action in matched_tools {
                    println!("{}", action.action().blue());
                    for tool in action.tools() {
                        println!("- {}", tool);
                    }
                    println!();
                }
            }
        }
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

fn find_tool(query: &str, tool_file: Option<PathBuf>) -> anyhow::Result<Vec<Action>> {
    let lquery = query.to_lowercase();
    let (default_tools, user_tools) = tools::tools(tool_file)?;
    let mut all_actions = default_tools.into_actions();
    all_actions.append(&mut user_tools.into_actions());
    let mut matched_actions = vec![];
    let query_parts = lquery.split(" ").collect::<Vec<_>>();
    for action in all_actions {
        if action.action().to_lowercase() == lquery {
            matched_actions.push(action.clone());
            continue;
        }
        let mut all_match = true;
        for part in query_parts.iter() {
            if !all_match {
                break;
            }
            if !action.action().to_lowercase().contains(part) {
                all_match = false;
            }
        }
        if all_match {
            matched_actions.push(action.clone());
        }
    }
    Ok(matched_actions)
}

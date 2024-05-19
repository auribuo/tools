use std::{io, path::PathBuf, str::FromStr};

use anyhow::Result;

pub(crate) static DEFAULT_TOOLS: &'static str = include_str!("../tools.toml");

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Action {
    action: String,
    tools: Vec<String>,
}

impl Action {
    pub(crate) fn action(&self) -> &str {
        &self.action
    }

    pub(crate) fn tools(&self) -> &Vec<String> {
        &self.tools
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct Tools {
    actions: Vec<Action>,
}

impl Tools {
    pub(crate) fn actions(&self) -> &Vec<Action> {
        &self.actions
    }
}

fn default_tools() -> Result<Tools> {
    Ok(toml::from_str(DEFAULT_TOOLS)?)
}

fn user_tools(tool_file: Option<PathBuf>) -> Result<Tools> {
    let user_tool_path = tool_file.unwrap_or_else(|| PathBuf::from_str("~/.tools.toml").unwrap());
    let user_tool_content = match std::fs::read_to_string(user_tool_path) {
        Ok(content) => content,
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => "actions = []".to_string(),
            _ => return Err(err.into()),
        },
    };
    Ok(toml::from_str(&user_tool_content)?)
}

pub(crate) fn tools(tool_file: Option<PathBuf>) -> Result<(Tools, Tools)> {
    Ok((default_tools()?, user_tools(tool_file)?))
}

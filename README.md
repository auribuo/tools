# 🧰 tools

Forgot a nice cli-tool you installed and went on searching for a tool to solve your problem just ending up realizing
you already had it installed and forgotten?

Look no further `tools` is here to help!

## Installation

For now the only way to install `tools` is to clone the repo and `cargo build` the source.

## Tool management

By default the `tools` uses both the default tool file `tools.toml` which gets compiled into the binary to avoid having to ship it with an installer.

In the future I'll (maybe) implement the download of the tools from here to avoid having a big binary.

The user can define additional tools in the user config located at `~/.tools.toml`

## Contributing

If you want to add a tool available to everyone open a PR to add it to the default tools.

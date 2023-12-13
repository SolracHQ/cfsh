# CFSH - Config Shell

A Simple tool to manage aliases and environment variables across multiple shells.

## Installation

```bash
git clone https://github.com/CarlosEduardoL/cfsh.git
cd cfsh
cargo install .
```

## Usage

- On bash, add the following line to your `.bashrc` file:

```bash
eval "$(cfsh bash)"
```

- On zsh, add the following line to your `.zshrc` file:

```zsh
eval "$(cfsh zsh)"
```

- On fish, add the following line to your `config.fish` file:

```fish
cfsh fish | source
```

- On powershell, add the following line to your `Microsoft.PowerShell_profile.ps1` file:

```powershell
Invoke-Expression (& cfsh powershell | Out-String)
```

## Configuration

You can create your config file using ron, yaml or json. The default config file is on 
 - Linux: `$XDG_CONFIG_HOME/cfsh/config.ron` or `$HOME/.config/cfsh/config.ron`
 - MacOS: `$HOME/Library/Application Support/cfsh/config.ron`
 - Windows: `{FOLDERID_RoamingAppData}\cfsh\config.ron`

> [!WARNING]
> Yaml does not suport nested enums, so you can't use the `not` condition on yaml configs.

Example config:

- [ron](example.ron)
- [yaml](example.yaml)
- [json](example.json)
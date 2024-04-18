# alias-rs

A utility to manage aliases across shells.

## Installation

```
cargo install --git https://github.com/nomyfan/alias-rs
```

## Usage

### PowerShell

Add this into your `$PROFILE`.

```pwsh
Invoke-Expression (&als init powershell | Out-String)
```

### zsh

Add this into your `.zshrc`.

```shell
eval "$(als init zsh)"
```

### bash

Add this into your `.bashrc`.

```shell
eval "$(als init bash)"
```

### fish

Add this into your `config.fish`.

```shell
als init fish | source
```

## Alias config

Put a config file in `~/.config/alias.toml`.

### Format

All aliases are defined in the `aliases` table, with alias name as key.

### Example

```toml
[aliases]
# pnpm
p = "pnpm"
pi = "pnpm install"
pa = "pnpm add"
pb = "pnpm build"
# nvim
vim = "nvim"

[aliases.cls]
zsh = "clear"

[aliases.opengh]
zsh = "node $HOME/.ss/JavaScript/opengh.mjs"
powershell = "node (Join-Path $HOME -ChildPath .ss -AdditionalChildPath JavaScript,opengh.mjs)"

[aliases.rmrf]
zsh = "rm -rf"
powershell = "Remove-Item -Recurse -Force"

```

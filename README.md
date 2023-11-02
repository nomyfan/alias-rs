# alias-rs

A utility to manage aliases across platforms.

## Installation

```
cargo install --git https://github.com/nomyfan/alias-rs
```

## Usage

### PowerShell

Add this into your `$USERPROFILE`.

```pwsh
Invoke-Expression (&als init powershell | Out-String)
```

### zsh

Add this into your `.zshrc`.

```shell
eval "$(als init zsh)"
```

## Alias definations

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
powershell = "$js = (Join-Path $env:HOME -ChildPath .ss -AdditionalChildPath JavaScript,opengh.mjs); node $js $args"

[aliases.rmrf]
zsh = "rm -rf"
powershell = "Remove-Item -Recurse -Force"

```

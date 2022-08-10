# Minigrep
A mini version of grep as the name says that i made while learning rust. \
It isn't unique or anything so don't expect much.

## Installation
### NixOS
**Prerequisites**- Nix with [flake](https://nixos.wiki/wiki/Flakes) support
```
nix shell github:Krypion17/minigrep
```

### Non - NixOS
**Prerequisites**- rustc, rustup, cargo \
**Get from here**- https://www.rust-lang.org/tools/install
```
git clone https://github.com/Krypion17/minigrep.git
cd minigrep
cargo build --release
```
> For building in different directory just __cd__ into your desired directory

## Usage
Here's a simple gif showing the usage
<p align="center">
    <img src="media/usage.gif">
</p>

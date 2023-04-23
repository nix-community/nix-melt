# nix-melt

[![release](https://img.shields.io/github/v/release/nix-community/nix-melt?logo=github&style=flat-square)](https://github.com/nix-community/nix-melt/releases)
[![version](https://img.shields.io/crates/v/nix-melt?logo=rust&style=flat-square)](https://crates.io/crates/nix-melt)
[![deps](https://deps.rs/repo/github/nix-community/nix-melt/status.svg?style=flat-square&compact=true)](https://deps.rs/repo/github/nix-community/nix-melt)
[![license](https://img.shields.io/badge/license-MPL--2.0-blue?style=flat-square)](https://www.mozilla.org/en-US/MPL/2.0)
[![ci](https://img.shields.io/github/actions/workflow/status/nix-community/nix-melt/ci.yml?label=ci&logo=github-actions&style=flat-square)](https://github.com/nix-community/nix-melt/actions/workflows/ci.yml)

A ranger-like flake.lock viewer

![](https://user-images.githubusercontent.com/40620903/234416489-75f991a9-b6f0-490a-8b07-12297fe07bba.png)

## Usage

```bash
nix run github:nix-community/nix-melt
```

```
Usage: nix-melt [OPTIONS] [PATH]

Arguments:
  [PATH]  Path to the flake.lock or the directory containing flake.lock [default: flake.lock]

Options:
  -t, --time-format <TIME_FORMAT>  Format to display timestamps
                                   [default: "[year]-[month]-[day] [hour]:[minute] [offset_hour sign:mandatory]:[offset_minute]"]
  -h, --help                       Print help
  -V, --version                    Print version
```

## Changelog

See [CHANGELOG.md](CHANGELOG.md)

# LSD (LSDeluxe)

[![license](http://img.shields.io/badge/license-Apache%20v2-orange.svg)](https://raw.githubusercontent.com/Peltoche/ical-rs/master/LICENSE)
[![Latest version](https://img.shields.io/crates/v/lsd.svg)](https://crates.io/crates/lsd)

This project is heavily inspired by the super [colorls](https://github.com/athityakumar/colorls)
project but with some little differences. For example it is written in rust and not in ruby which makes it much faster.

![image](https://raw.githubusercontent.com/Peltoche/lsd/assets/screen_lsd.png)

## Installation

Install patched fonts from powerline nerd-font and/or font-awesome and configure your terminal emulator to use them.\
Have a look at the [Nerd Font README](https://github.com/ryanoasis/nerd-fonts/blob/master/readme.md) for more installation instructions.

See [this issue comment](https://github.com/Peltoche/lsd/issues/199#issuecomment-494218334) for detailed instructions on how to configure iTerm2 font settings correctly.

| Platform                     | Instructions                                                                                                                   |
| ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| ArchLinux                    | `pacman -S lsd`                                                                                                                |
| Fedora                       | `dnf install lsd`                                                                                                              |
| Ubuntu                       | Download latest `.deb` from [release page](https://github.com/Peltoche/lsd/releases) and do `dpkg -i lsd_<version>_<arch>.dev` |
| Gentoo                       | `emerge sys-apps/lsd`                                                                                                          |
| macOS                        | `brew install lsd`                                                                                                             |
| nix                          | `nix-env -iA nixos.lsd` or add `environment.systemPackages = with pkgs; [ ... , lsd ];` in config                              |
| FreeBSD                      | `pkg install lsd`                                                                                                              |
| Windows                      | `scoop install lsd`                                                                                                            |
| From binaries                | The [release page](https://github.com/Peltoche/lsd/releases) includes precompiled binaries for Linux and macOS                 |
| From source (latest release) | `cargo install lsd`                                                                                                            |
| From source (master)         | `cargo install --git https://github.com/Peltoche/lsd.git --branch master`                                                      |

## Configurations

### Recommended aliases

In order to use lsd when entering the `ls` command, you need to add this to your shell
configuration file (~/.bashrc, ~/.zshrc, etc.):

```sh
alias ls='lsd'
```

Some further examples of useful aliases:

```sh
alias l='ls -l'
alias la='ls -a'
alias lla='ls -la'
alias lt='ls --tree'
```

## F.A.Q.

### Default Colors

In the future the possibility to customize the colors might be implemented.
For now, the default colors are:

| User/Group                                                     | Permissions                                                                     | Last time Modified                                                            | File Size                                                            |
| :------------------------------------------------------------- | :------------------------------------------------------------------------------ | :---------------------------------------------------------------------------- | :------------------------------------------------------------------- |
| ![#ffffd7](https://placehold.it/17/ffffd7/000000?text=+) User  | ![#00d700](https://placehold.it/17/00d700/000000?text=+) Read                   | ![#00d700](https://placehold.it/17/00d700/000000?text=+) within the last hour | ![#ffffaf](https://placehold.it/17/ffffaf/000000?text=+) Small File  |
| ![#d7d7af](https://placehold.it/17/d7d7af/000000?text=+) Group | ![#d7ff87](https://placehold.it/17/d7ff87/000000?text=+) Write                  | ![#00d787](https://placehold.it/17/00d787/000000?text=+) within the last day  | ![#ffaf87](https://placehold.it/17/ffaf87/000000?text=+) Medium File |
|                                                                | ![#af0000](https://placehold.it/17/af0000/000000?text=+) Execute                | ![#00af87](https://placehold.it/17/00af87/000000?text=+) older                | ![#d78700](https://placehold.it/17/d78700/000000?text=+) Large File  |
|                                                                | ![#ff00ff](https://placehold.it/17/ff00ff/000000?text=+) Execute with Stickybit |                                                                               | ![#ffffff](https://placehold.it/17/ffffff/000000?text=+) Non File    |
|                                                                | ![#d75f87](https://placehold.it/17/d75f87/000000?text=+) No Access              |                                                                               |                                                                      |

## Credits

Special thanks to:

- [meain](https://github.com/meain) for all his contributions and reviews
- [danieldulaney](https://github.com/danieldulaney) for the Windows integration
- [sharkdp](https://github.com/sharkdp) and his superb [fd](https://github.com/sharkdp/fd) from which I have stolen a lot of CI stuff.
- [athityakumar](https://github.com/athityakumar) for the project [colorls](https://github.com/athityakumar/colorls)
- All the other [contributors](https://github.com/Peltoche/lsd/graphs/contributors)

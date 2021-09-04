<h1 align="center">Bonnie</h1>

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![Test](https://github.com/arctic-hen7/bonnie/actions/workflows/ci.yml/badge.svg)](https://github.com/arctic-hen7/bonnie/actions/workflows/ci.yml)
[![Build and Release](https://github.com/arctic-hen7/bonnie/actions/workflows/cd.yml/badge.svg)](https://github.com/arctic-hen7/bonnie/actions/workflows/cd.yml) [![Join the chat at https://gitter.im/bonnie-cli/community](https://badges.gitter.im/bonnie-cli/community.svg)](https://gitter.im/bonnie-cli/community?utm_source=badge&utm_medium=badge&utm_campaign=pr-badge&utm_content=badge)

> **Simple, cross-platform, and fast command aliases with superpowers.**

[Documentation][docs] • [Releases][releases] • [Contributing][contrib]

Bonnie is a command aliasing tool that allows you to quickly and efficiently define short aliases for long commands that you have to repeatedly run. Here's a quick feature overview:

-   ✨ Supports simple key-value aliasing
-   ✨ Supports inserting custom arguments into commands
-   ✨ Supports interpolating environment variables
-   ✨ Supports adding any and all arguments given into a single place
-   ✨ Supports using different commands on different operating systems
-   ✨ Supports specifying custom shells for individual commands
-   ✨ Supports specifying default shells for different operating systems on a per-file basis
-   ✨ Supports infinitely nestable subcommands
-   ✨ Supports subcommands executed in a certain order based on their exit codes
-   ✨ Supports caching large config files after they've been parsed for performance
-   ✨ Supports initializing new config files from templates
-   ✨ Supports global template
-   ✨ Supports debug mode
-   ✨ Supports self-documenting configuration files

Basically, if you have commands that you routinely run in a project, Bonnie is for you. Bonnie has support for both extremely simple and extremely complex use cases, all while maintaining top-notch performance.

## What does it look like?

The simplest possible Bonnie configuration file would look like this:

```toml
version = "0.3.1"

[scripts]
build = "echo Building"
```

This syntax easily expands as your scripts grow in complexity to support everything up to infinitely nestable subcommands that automatically execute in a certain order based on their exit codes, running in multiple stages using custom shells and different commands on different operating systems, all while interpolating given arguments and a few environment variables. No, that's not an exaggeration, you can see an example in the [documentation][docs]!

## How is this different from Bash aliases?

Bash aliases are a great way of turning long commands into short commands, but they can't be easily customized on a per-folder basis. Bonnie can be. Further, if you want to interpolate custom arguments or environment variables, you'll have to write a small script for each alias. If you want subcommands, good luck! Bonnie solves all these problems with simple, intuitive syntax that just works.

Bonnie is also cross-platform, even supporting running different commands on different operating systems for the same command alias.

## How is this different from `Make`?

GNU Make was designed to recompile parts of a project when certain files change, however its capacity to specify command aliases has drawn many to use it for that purpose. However, Make is old and has a clunky, inflexible syntax. For even simple purposes, it can be hugely overkill.

By contrast, Bonnie uses TOML, which is designed specifically to be highly readable by humans, as well as a simple syntax that expands as you need it to. You don't have to write out configuration for things you'll never use.

## Installation

You can install Bonnie easily from the [releases][releases] page for Windows, MacOS, Linux, and musl. If you need Bonnie for another system, you can clone this repository and build the project using Cargo as needed. If you think we should support a particular OS in the default releases, please [open an issue][newissue] and let us know!

After you've downloaded or built the binary, move it to a location where it's easily executable (e.g. `/usr/local/bin` on Linux). You'll also need to make it executable (`chmod +x ./[BINARY_NAME]` on Linux). Then, you should be able to run Bonnie from the terminal with `bonnie`!

### Installing in Docker

Bonnie provides pre-built executables for Linux and musl (e.g. Alpine Linux), which can be easily installed in a Dockerfile with this command:

```Dockerfile
RUN curl -L https://github.com/arctic-hen7/bonnie/releases/download/[VERSION]/bonnie-[OS]-amd64
```

Just replace `[VERSION]` with the latest version (see the [releases page][releases]) and `[OS]` with the operating system you want the binary for (one of: `windows`, `macos`, `linux`, or `musl`).

## Why 'Bonnie'?

No particular reason, the name just sounds nice. No offense intended whatsoever to those named Bonnie!

## Stability

Bonnie is very fully-featured already, though the project is still under active development, and there are a few key features still to be added before v1.0.0. Also, Bonnie's ordered subcommands system (see the [documentation][docs]) is very novel, and we need to see how it performs over a longer period of time in production.

Bonnie was originally intended to move to stable in May 2021, though a full rewrite of the program and the introduction of nearly every major feature the program now has occurred in June, delaying that deadline. Right now, Bonnie is scheduled to go to stable by the end of September 2021, but the program is used daily and is actively maintained, so don't let that discourage you!

## Roadmap

-   [x] Support default global template in `~/.bonnie/template.toml`
-   [x] Support debug mode
-   [ ] Support self-documenting configurations

*   [ ] Support optional arguments
*   [ ] Support giving default values for optional arguments
*   [ ] Support piping data into Bonnie scripts with a special opening flag (maybe `%[stdin]`?)

## Changelog

You can see all the recent updates to the project in the [changelog](./CHANGELOG.md).

If there's anything you think should be on here, or if you find a bug, please [open an issue][newissue] and let us know!

## Contributing

Thanks so much! You can learn about how to contribute to Bonnie by reading [the contributing guide][contrib], and please remember to stick to the [code of conduct](./CODE_OF_CONDUCT.md).

## License

See [LICENSE](./LICENSE).

[docs]: https://github.com/arctic-hen7/bonnie/wiki
[releases]: https://github.com/arctic-hen7/bonnie/releases
[contrib]: ./CONTRIBUTING.md
[newissue]: https://github.com/arctic-hen7/bonnie/issues/new/choose

# Bonnie Specification

> **WARNING:** This syntax is highly experimental and has not yet been introduced into Bonnie! This document primarily serves to unify development efforts on this syntax moving forward!

This file contains the full Bonnie syntax specification. While what's in the README and the help page is usually enough, if you want to use Bonnie to its full potential, this is your guide. Examples of the syntax in this specification may be found in `src/bonnie.toml`, which is used for testing the program.

Please note that the terms _alias_ and _command_ are used interchangeably in this document.

## Parsed Specification

This is the form that all Bonnie configuration files will be parsed into internally. This itself is not valid syntax in itself, this is what the compiler gets to. Functions on structs are elided here.

Note that some top-level properties like `version` are not included in this because they will be used for side-effects in the checking of the configuration file's validity.

```rust
struct Config {
    default_shell: DefaultShell,
    scripts: Scripts,
}
struct DefaultShell {
    generic: Shell,
    targets: HashMap<String, Shell>, // If the required target is not found, `generic` will be tried
}
impl DefaultShell {
    fn get_shell_for_target(&self, target: TargetString) { ... }
}
struct Shell {
    form: String,
}
impl Shell {
    fn interpolate_command(&self, command: String) { ... }
}
type TargetString = String; // A target like `linux` or `x86_64-unknown-linux-musl` (see `rustup` targets)
type Scripts = HashMap<String, Command>;

struct Command {
    args: Vec<String>,
    env_vars: Vec<String>,
    domains: Option<Scripts>, // Subcommands are fully-fledged commands (mostly)
    order: Option<OrderString>, // If this is specified, subcomands must not specify the `args` property, it may be specified at the top-level of this script as a sibling of `order`
    cmd: CommandWrapper,
}
type OrderString = String; // A string of as yet undefined syntax that defines the progression between domains
struct CommandWrapper {
    generic: CommandCore,
    targets: HashMap<TargetString, CommandCore>, // If empty or target not found, `generic` will be used
}
// This is the lowest level of command specification, there is no more recursion allowed here (thus avoiding circularity)
// Actual command must be specified here are strings (with potnetial interpolation of arguments and environment variables)
struct CommandCore {
    cmd: String, // This is the actual command that will be run
    shell: Option<Shell>, // If given, this is the shell it will be run in, or the `default_shell` config for this target will be used
}
```

The following key points should be noted from the above:

-   All commands are transformed into vectors at the lowest level, meaning simple one-command aliases become a vector with a single element
-   If the `order` property is specified on a command, the `domains` property must be specified, and no domain or subdomain may specify the `args` property, which may instead be specified at the top-level as a sibling of `order`; all subdomains must specify the `order` property as well

## Configuration File Syntax

This syntax specifies the actual form users will write in Bonnie configuration files, which is designed to be as easy as possible to use, with most features being optional. Name collisions between structs/enums/etc. defined here and those in the final form are deliberate.

```rust
struct Config {
    version: String, // This will be used to confirm compatibility
    env_files: Option<Vec<String>>, // Files specified here have their environment variables loaded into Bonnie
    default_shell: Option<DefaultShell>,
    scripts: Scripts,
}
enum DefaultShell {
    Simple(String), // Just a generic shell
    Complex {
        generic: Shell, // A generic shell must be given
        targets: Option<HashMap<String, Shell>>
    }
}
impl DefaultShell {
    fn get_shell_for_target(&self, target: TargetString) { ... }
}
type Shell = String; // The gets decoded into a proper struct with methods, the location for command interpolation is specified with '{COMMAND}'
type TargetString = String; // A target like `linux` or `x86_64-unknown-linux-musl` (see `rustup` targets)
type Scripts = HashMap<String, Command>;

enum Command {
    Simple(CommandBox), // Might be just a string command to run on the default generic shell
    Complex {
        args: Option<Vec<String>>,
        env_vars: Option<Vec<String>>,
        domains: Option<Scripts>, // Subcommands are fully-fledged  commands (mostly)
        order: Option<OrderString>, // If this is specified,    subcomands must not specify the `args` property, it may be specified at the top-level of this script as a sibling of `order`
        cmd: CommandBox,
    },
}
type OrderString = String; // A string of as yet undefined syntax that defines the progression between domains
// This wraps the complexities of having different shell logic for each command in a multi-stage context
// Domains are specified above this level (see `Command::Complex`)
enum CommandBox {
    Simple(CommandWrapper),
    MultiStage(Vec<CommandWrapper>),
}
enum CommandWrapper {
    Universal(CommandCore), // Just a given command
    Specific {
        generic: CommandCore,
        targets: Option<HashMap<TargetString, CommandCore>>
    },
}
enum CommandCore {
    Simple(String), // No shell configuration
    WithShell {
        cmd: String,
        shell: Option<Shell>
    },
}
```

One of the key differences between the syntax written and the interpreted final form is that the final form specifies all commands as vectors for simplicity of running them with a `for` loop, while the written syntax may specify them in a simpler form.

The level of syntactic complexity supported by Bonnie allows for great flexibility, and permits extremely granular control. For example, a user could specify a series of commands that flow together in a certain way, each of which has a different version for each major OS, some of which run in different shells.

## Bones Syntax

Bones is Bonnie's internal scripting language for specifying control flow between subcommands. If the `order` property is specified on a command, its subcommands will be called according to the Bones program provided in that property.

Bones syntax is currently entirely undeveloped. Bonnie will act as if it is, though the Bones parser is currently unwritten and any attempted usage will result in a warning. The attempted invocation of a command with ordered subcommands will result in an error until the Bones language has been given a formal structure.

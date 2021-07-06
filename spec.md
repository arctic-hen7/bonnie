# Bonnie Specification

This file contains the full Bonnie syntax specification as Rust code. This is a very technical document is not necessary at all to use Bonnie! We recomend referring to the rest of the documentation, this is mostly to help people understand how Bonnie works under the hood.

Please note that the terms _alias_ and _command_ are used interchangeably in this document.

## Configuration File Syntax

This syntax specifies the actual form users will write in Bonnie configuration files, which is designed to be as easy as possible to use, with most features being optional. Due to the large number of `enum`s here, meaning many differing possibilities, this syntax is unified into a final form without this ambiguity, making later processing easier. THat transfer process also allows the abstraction of nearly all Bonnie logic to a something akin to a compile stage for the user's Bonnie configuration file. In future, that will allow caching for complex systems.

```rust
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    version: String,                // This will be used to confirm compatibility
    env_files: Option<Vec<String>>, // Files specified here have their environment variables loaded into Bonnie
    default_shell: Option<DefaultShell>,
    scripts: Scripts,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum DefaultShell {
    Simple(Shell), // Just a generic shell
    Complex {
        generic: Shell, // A generic shell must be given
        targets: Option<HashMap<String, Shell>>,
    },
}
type Shell = Vec<String>; // A vector of the executable followed by raw arguments thereto, the location for command interpolation is specified with '{COMMAND}'
type TargetString = String; // A target like `linux` or `x86_64-unknown-linux-musl` (see `rustup` targets)
type Scripts = HashMap<String, Command>;

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum Command {
    Simple(CommandWrapper), // Might be just a string command to run on the default generic shell
    Complex {
        args: Option<Vec<String>>,
        env_vars: Option<Vec<String>>,
        subcommands: Option<Scripts>, // Subcommands are fully-fledged  commands (mostly)
        order: Option<OrderString>, // If this is specified,subcomands must not specify the `args` property, it may be specified at the top-level of this script as a sibling of `order`
        cmd: Option<CommandWrapper>,    // This is optional if subcommands are specified
    },
}
type OrderString = String; // A string of as yet undefined syntax that defines the progression between subcommands
                           // This wraps the complexities of having different shell logic for each command in a multi-stage context
                           // subcommands are specified above this level (see `Command::Complex`)
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandWrapper {
    Universal(CommandCore), // Just a given command
    Specific {
        generic: CommandCore,
        targets: Option<HashMap<TargetString, CommandCore>>,
    },
}
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandCore {
    Simple(CommandBox), // No shell configuration
    WithShell {
        exec: CommandBox, // We can't call this `cmd` because otherwise we'd have a collision with the higher-level `cmd`, which leads to misinterpretation
        shell: Option<Shell>,
    },
}
// This represents the possibility of a vector or string at the lowest level
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum CommandBox {
    Simple(String),
    MultiStage(Vec<String>),
}
```

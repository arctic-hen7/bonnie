<h1 align="center">Bonnie</h1>

[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.0-4baaaa.svg)](CODE_OF_CONDUCT.md)
[![Test](https://github.com/arctic-hen7/bonnie/actions/workflows/ci.yml/badge.svg)](https://github.com/arctic-hen7/bonnie/actions/workflows/ci.yml)
[![Build and Release](https://github.com/arctic-hen7/bonnie/actions/workflows/cd.yml/badge.svg)](https://github.com/arctic-hen7/bonnie/actions/workflows/cd.yml)

Bonnie is a command aliasing tool. If you have a super-long command that you have to run all the time, Bonnie is for you! Just define the command in `bonnie.toml` at the root of your project, and you're good to go!

```toml
[scripts]
short = "really long command..."
```

For example, if you're running `docker-compose` a lot, and you have a custom environment variables file, it's annoying to type out `docker-compose --env-file .my.env ...` every time you want to do something! Bonnie can shorten this easily!

```toml
[scripts]
dc = "docker-compose --env-file .my.env %%"
```

The double percent sign (`%%`) at the end tells Bonnie to append any arguments you provide to the end of the command.

You can even insert custom arguments into custom places in a custom order!

```toml
[scripts]
greet.cmd = "echo \"Greetings %lastname. I see your first name is %firstname?\""
greet.args = [
	"firstname",
	"lastname"
]
```

Now if you run `bonnie greet Donald Knuth` you should get `Greetings Knuth. I see your first name is Donald?`

## Installation

Bonnie is built in [Rust](https://rust-lang.org), and you can either download a pre-compiled binary from [here](https://github.com/arctic-hen7/bonnie/releases) if you're running on one of the three major OSes on a 64-bit architecture, or you can clone this repository and build the code yourself. I'm happy to add more binaries for more OSes by request, please file an issue for the matter.

### Using a pre-compiled binary

If you head over to the [releases page](https://github.com/arctic-hen7/bonnie/releases) on this repository, you'll be able to download binaries for the most recent version of Bonnie for the three major operating systems (Linux, MacOS, and Windows). GitHub Actions builds these automatically whenever a new tag is pushed, though I personally develop on Linux, so if the Windows or MacOS binaries don't work for some reason, please let me know! After downloading the binary, it should work immediately, though you may need to allow executing it as a program on Unix systems with `chmod +x ./bonnie-linux-amd64` (or whatever your download is called). Then, you can move it to a location that suits you, like `/usr/local/bin` on Linux so you can execute it from anywhere on your system.

### Building manually

1. Clone this repository to your local machine.
2. Ensure you have Rust and Cargo installed.
3. Run `cargo build --release && cp target/releases/bonnie bonnie`. The built executable will be at `./bonnie`. You can then copy this to somewhere you put binaries, like `/usr/local/bin`.

## Syntax

### Basics

Bonnie is a simple command-line interface that looks for a `bonnie.toml` file in whatever directory you run the command in. All configuration is defined in there. If you want to put your config file somewhere else, or name it something else, see _Using a custom config file_ below. A basic Bonnie config file should look like this:

```toml
[scripts]
```

You can define all your command aliases, which Bonnie calls _scripts_ under the `[scripts]` heading. This file uses TOML, which is like JSON, but much easier to read for humans. You can learn more about it and the syntax it supports at [toml.io](https://toml.io). If you use any invalid TOML syntax or you don't define scripts properly in your config, Bonnie will tell you straight away, without trying to run the command you specified, so don't worry about something blowing up!

### Command with arguments

Bonnie supports inserting arguments into commands. This requires you to use the following syntax in `bonnie.toml`:

```toml
[scripts]
greet.cmd = "echo \"Greetings %lastname. I see your first name is %firstname?\""
greet.args = [
	"firstname",
	"lastname"
]
```

In that example, you denote the script by specifying the system command to run with the key `greet.cmd` (where `greet` is the script's name). You can then specify arguments with `greet.args`, setting that equal to an array containing the names of all the arguments in the script. You can have as many arguments as you want! Back in the `greet.cmd` value, you define where each argument goes by writing `%firstname`, where `firstname` is the name of the argument. You can do this in any order, the order that matters is that of the `greet.args` array, that's the order you'll be expected to supply arguments in when you run the script. If you stuff up and forget to put in an argument placeholder, Bonnie will tell you and not run the command. If you misspell an argument placeholder, Bonnie will tell you.

You can if you want to insert one argument more than once, Bonnie just replaces all instances of `%firstname` or the like, so that should work fine (not part of the test suite though).

As yet, Bonnie doesn't support optional arguments or default values, but hopefully that won't inconvenience you too much! If enough people want those features, I'll implement them at some point in the future.

### Shorthand

Many of the commands you specify won't need arguments, they'll just be simple aliases. These can be specified trivially by just writing a key-value pair:

```toml
[scripts]
foobar = "echo Hello World"
```

You don't need `.cmd` or `.args`, you can just define it as `foobar`, where that's the name of the script. This is the most common way to define scripts with Bonnie.

### Appending arguments

There are a lot of use cases where you'd want any arguments you provide to be appended to the end of a Bonnie script (this is the default behavior of NPM and Yarn scripts). You can do this easily in Bonnie by using shorthand syntax and adding a `%%` to the end of the command like so:

```toml
[scripts]
dc = "docker-compose --env-file .my.env %%"
```

Note that you cannot combine this appending behavior with custom arguments yet, and Bonnie will tell you if you try to (and won't run the command). If enough people want this combination to be possible, I'm happy to implement the feature at some point in the future.

Note that when you run a script that appends arguments, it will accept any number of arguments, including 0. Bonnie will tell you neither about too many or too few arguments provided. If enough people want support for the allowed number of arguments to be able to be specified by a range, I'm happy to implement that feature.

As of yet, you can only append all arguments at the **end** of a command, nowhere else. This does mean that if you try `echo '%%'` (note the ending `'`), Bonnie will not append any arguments, and will treat that as ordinary shorthand syntax. Support for appending arguments inside a script is on the roadmap.

## Escaping %

Bonnie uses `%` to denote arguments in commands. If you have something like `%firstnameBlah`, where `firstname` is the argument, Bonnie will only replace that. Any instances of `%` signs not connected to arguments or other special flags (right now only the `%%` append flag) will be left untouched. If you need to put `%firstname` in and not have it replaced by an argument named `firstname`, you'll have to pick a different argument name.

Slightly surprising behaviour can be if you have two arguments, one `firstname` and the other `first`. If `first` is provided before `firstname`, Bonnie will replace the `%first` part of `%firstname` and then tell you it couldn't insert the `firstname` argument. In other words, if one argument is a substring of another from index 0 (e.g. `first` and `firstname`) and you provide the substring before the superstring, Bonnie will accidentally mangle instances of the superstring argument and then complain that said argument couldn't be inserted. Basically, **don't have arguments that are substrings of each other from index 0**, they'll behave weirdly in certain cases.

## Running commands

After you've set up your `bonnie.toml` file, you can run any of the commands you've defined easily like so:

```
bonnie [script_name] [arg1] [arg2] [etc.]
```

Just type the name of the script, and then add any arguments it expects. If you don't give enough arguments, Bonnie will tell you and won't run the command. If you provide too many, Bonnie **will** run the command, and will add a warning telling you you've provided too many arguments. That'll be written to `stdout` rather than `stderr`, so if you're parsing the output of a Bonnie script, make sure you've got the right number of arguments, or you're ready to handle that warning.

The output of the command (`stdout` and `stderr`) will be piped directly to the corresponding properties of the Bonnie process. All commands are run as child processes, and they do **not** inherit the `stdin` of Bonnie. If you're trying to pipe data into a script, unfortunately that isn't yet possible. If enough people would find this helpful, then I'm happy to implement it at some point in the future.

## Using a custom config file

There may be cases in which your bonnie config file is in a different directory, and these can easily be handled by setting the `BONNIE_CONF` environment variable. On Unix systems (MacOS and Linux), this can be done by running `BONNIE_CONF=path/to/bonnie.toml bonnie ...`.

## Reserved commands

Bonnie does have a few internal commands, and your own scripts defined in `bonnie.toml` cannot conflict with these. You won't be warned about this, Bonnie will just completely ignore your scripts whenever you run one of these commands:

- `bonnie help` - displays the Bonnie documentation
- `bonnie init` - creates a new `bonnie.toml` file in the current directory (won't override existing files)

## Motivation

I used to use JavaScript a lot, where I had access to beautiful tools like Yarn and NPM scripts. I learned to love those a lot, and then I switched to Elm and Rust, and I was suddenly deprived of a nice script tool! Make was the most obvious option, but it felt a little dated and cumbersome for something so simple, so I decided to make my own version while Ia was learning Rust! This is that.

## Aim

Right now, Bonnie is a really simple development automation tool, but in future, I may expand its capabilities so that it becomes a proper build tool with highly extensible scripting functionalities!

## Why 'Bonnie'?

No particular reason, the name just sounded nice, and I thought I may as well name this program Bonnie. No offense intended whatsoever to those named Bonnie!

## Stability

Right now, Bonnie is in beta because of the youth of the project and to allow time for bugs to be reported. I use the project daily, and I'll continue adding features on the roadmap as I do so. Right now, there are some parts of Bonnie that aren't covered by automated tests (though I test those parts manually before any releases), so I'll complete those before the project moves into stable 1.0. Right now, v1.0 will probably be released some time in May 2021, but the program is perfectly fine to use up until that point!

## Roadmap

- [ ] Add automated tests for the warning system to when too many arguments are provided
- [ ] Add automated tests for the command running system
- [ ] Support inserting all arguments somewhere into a command rather than just at the end
- [ ] Support a combination of custom arguments and appending arguments
- [ ] Support optional arguments
- [ ] Support giving default values for optional arguments
- [ ] Support specifying the upper and lower bounds of acceptable numbers of appended arguments with a range
- [ ] Support piping data into Bonnie scripts with a special opening flag (maybe `%[stdin]`?)

## Authors

- arctic_hen7

## License

See [LICENSE.txt](./LICENSE.txt)

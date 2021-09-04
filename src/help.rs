use crate::version::BONNIE_VERSION;

pub fn help(output: &mut impl std::io::Write) {
    writeln!(
        output,
        "Bonnie v{version} help page:
------------------------

Bonnie is a command aliasing tool that supports extremely simple and extremely advanced syntax. For the full reference, please see the documentation at https://github.com/arctic-hen7/bonnie/wiki.
This just summarizes the functionality of this command, not the syntax of Bonnie configuration files!

-h, --help                                      prints this help page
-v, --version                                   prints the current version of Bonnie
-i, --init [-t, --template <template-file>]     creates a new `bonnie.toml` configuration (or whatever's set in `BONNIE_CONF`), using the specified template file if provided
-c, --cache                                     caches the Bonnie configuration file to `.bonnie.cache.json` for performance (this cache must be MANUALLY updated by re-running this command!)

help [command-name]                             prints the help page for the current Bonnie configuration or for the given command

The expected location of a Bonnie configuration file can be changed from the default `./bonnie.toml` by setting the `BONNIE_CONF` environment variable.
The expected location of a Bonnie cache file can be changed from the default `./.bonnie.cache.json` by setting the `BONNIE_CACHE` environment variable.
The expected location of your default template can be changed from the default `~/.bonnie/template.toml` by setting the `BONNIE_TEMPLATE` environment variable.

Further information can be found at https://github.com/arctic-hen7/bonnie/wiki.",
        version = BONNIE_VERSION
    )
    .expect("Failed to write help page.")
}

// This file just defines a single constant for the help page that'll be served to users with `bonnie help`

use crate::version::BONNIE_VERSION;

pub fn get_help_page() -> String {
"
Example Usage:
    Commands can be specified in bonnie.toml:

        version = \"".to_string() + BONNIE_VERSION + "\"
        [scripts]
        greet.cmd = \"echo \\\"Greetings %lastname. I see your first name is %firstname?\\\"\"

        Arguments can be parsed into a command as well:

        version = \"" + BONNIE_VERSION + "\"
        [scripts]
        greet.args = [
            \"firstname\",
            \"lastname\"
        ]

    To run the command in the terimal:
        'bonnie greet Donald Knuth'

    Arguments can be used multiple times in a command:

        version = \"" + BONNIE_VERSION + "\"
        [scripts]
        greet.cmd = \"echo \"Greetings %lastname. I see your first name is %firstname?\" and not %lastname\"

    If a script doesn't need any arguments, you can use shorthand syntax:

        version = \"" + BONNIE_VERSION + "\"
        [scripts]
        foobar = \"echo Hello World\"

    To run that, use the same syntax as before, just without any arguments:
        'bonnie foobar'.
    Appending arguments:
        To append arguments at the end of a script, use shorthand syntax and add a '%%' to the end of the command:

        version = \"" + BONNIE_VERSION + "\"
        [scripts]
        dc = \"docker-compose --env-file .my.env %%\"

    Environment variables can also be inserted from custom files, like .env:

        version = \"" + BONNIE_VERSION + "\"
        env_files = [
            \".env\"
        ]

        [scripts]
        interpolation.cmd = \"echo \\\"%GREETING %firstname!\\\"\"
        interpolation.args = [
            \"firstname\"
        ]
        interpolation.env_vars = [
            \"GREETING\"
        ]

    To run that, use normal arguments syntax:
        'bonnie interpolation Donald'.
"
}

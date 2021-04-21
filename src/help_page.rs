// This file just defines a single constant for the help page that'll be served to users with `bonnie help`

pub fn help_command()->String{
    let bonnie_help_page = format!("
Example Usage:
    Commands can be supplied by specifying in the bonnie.toml: 

        [script]
        greet.cmd = \"echo \"Greetings %lastname. I see your first name is %firstname?\"\"

    arguments to the command can also be passed in the bonnie.toml eg

        [script]
        greet.args = [\"firstname\",\"lastname\"]

    To run the command in the terimal:
        'bonnie greet Donald Knuth'
        
    arguments can also be supplied multiple times in the command eg:

        [script]
        greet.cmd = \"echo \"Greetings %lastname. I see your first name is %firstname?\" and not %lastname\"

Shorthand:
    for commong scripts that require no args or cmd eg:

        [scripts]
        foobar = \"echo Hello World\"

    To execute, run 'bonnie foobar'
Appending arguments:
    To append arguments at the end of a bonnie script, you can do this easily in Bonnie by using shorthand syntax and adding a %% to the end of the command like so:
        
        [scripts]
        dc = \"docker-compose --env-file .my.env %%\"
");

bonnie_help_page
}

use crate::version::BONNIE_VERSION;

pub fn help(output: &mut impl std::io::Write) {
    writeln!(
        output,
        "Bonnie v{version} help page:
------------------------

Hello World!
        ",
        version = BONNIE_VERSION
    )
    .expect("Failed to write help page.")
}

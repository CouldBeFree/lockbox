use clap::Parser;
use lockbox::{
    cli::{
        args::{Args, DEFAULT_PASSWORD_FILE_NAME},
        io::RpasswordPromptPassword,
        run_cli,
    },
    repl::repl,
};

fn main() {
    let mut input = std::io::stdin().lock();
    let mut output = std::io::stdout().lock();
    if std::env::args().len() == 1 {
        repl(
            &mut input,
            &mut output,
            &RpasswordPromptPassword,
            DEFAULT_PASSWORD_FILE_NAME.to_string(),
        )
    } else {
        let args = Args::parse();
        run_cli(&mut input, &mut output, args);
    }
}

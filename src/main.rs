use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo", about = "Print text to the console")]
struct Options {
    #[structopt(short, long, help = "Suppress the trailing newline character")]
    no_newline: bool,

    #[structopt(help = "The text to print")]
    text: String,
}

fn main() {
    let options = Options::from_args();
    let output = if options.no_newline {
        format!("{}", options.text)
    } else {
        format!("{}\n", options.text)
    };

    std::io::stdout()
        .write_all(output.as_bytes())
        .expect("Failed to write to stdout");
}

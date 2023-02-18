/* runs pig latin command line tool */
use clap::Parser;
// import lib
use piglatin::pig_latin;

#[derive(Parser)]
#[clap(version = "1.0", author = "Liam Gift", about = "Pig Latin")]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Liam Gift")]
    Speak {
        #[clap(short, long)]
        phrase: String,
    },
}
fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Speak { phrase }) => {
            println!("{} says {}", phrase, pig_latin(&phrase));
        }
        None => {
            println!("No command given");
        }
    }
}

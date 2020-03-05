
extern crate structopt;
extern crate colored;

use structopt::StructOpt;
use colored::*;

#[derive(StructOpt)]
struct Options {

    #[structopt(default_value="Cluck! Cluck!")]
    /// What does chicken say?
    message: String,
    
    #[structopt(short = "d", long="dead")]
    /// Make chicken appear dead
    dead: bool,

    #[structopt(short = "f", long = "file", parse(from_os_str))]
    /// Load chicken picture from specific file
    file: Option<std::path::PathBuf>,
}

fn main() {
    let options = Options::from_args();
    let message = options.message;
    let eye = if options.dead {"x"} else {"o"};
    println!("");
    println!("{}", message.black().underline().on_yellow());
    println!("   \\");
    println!("    \\");
    println!("       .=\"\"=.");
    println!("      / _  _ \\");
    println!("     |  {eye}  {eye}  |", eye=eye.red().bold());
    println!("     \\   /\\   /");
    println!("    ,/'-=\\/=-'\\,");
    println!("   / /        \\ \\");
    println!("  | /          \\ |");
    println!("  \\/ \\        / \\/");
    println!("      '.    .'");
    println!("       |`~~`|");
    println!("      /|\\  /|\\");
}

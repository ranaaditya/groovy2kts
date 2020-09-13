use structopt::StructOpt;
use std::fs::File;
use std::io::Write;

/// Structure for the Commandline arguments
#[derive(StructOpt)]
struct Cli {

    /// The path to the groovy build.gradle file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

}

fn main() -> std::io::Result<()> {
    println!("Hola, Work In Progress !");

    let args = Cli::from_args();

    // debugging the flags 
    println!("file = {}",args.path.display().to_string());

    let ktsfile_name = args.path.display().to_string() + ".kts";

    println!("new file = {}", ktsfile_name);

    let mut ktsfile = File::create(ktsfile_name)?;
    // ktsfile.write_all("Hola, Work In Progress")?;

    Ok(())

}

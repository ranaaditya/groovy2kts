use structopt::StructOpt;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::Write;
use std::fs;
use std::process;

/// Structure for the Commandline arguments
#[derive(StructOpt)]
struct Cli {

    /// The path to the groovy build.gradle file
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,

}

/// Utility functionn to check if the given file in args exists
pub fn path_exists(path: &String) -> bool {
    fs::metadata(path).is_ok()
}

pub fn read_from_buildfile(path : &String)-> String {
    if(!path_exists(path)) {
        println!("{}", "The given file doesnot exists !");
        process::exit(1);
    }

    let mut file = File::open(path).expect("unable to open the file");
    let mut content = String::new();
    file.read_to_string(&mut content);
    
    // Debugging
    //println!("Content of the {} is {}", path, content);
    return content;

}


fn main() -> std::io::Result<()> {
    println!("Hola, Work In Progress !");

    let args = Cli::from_args();

    // debugging the flags 
    println!("file = {}",args.path.display().to_string());

    let ktsfile_name = args.path.display().to_string() + ".kts";

    println!("new file = {}", ktsfile_name);

    println!("Exist = {}", path_exists(&ktsfile_name));

    if(path_exists(&args.path.display().to_string()))  {

    ///creating build.gradle.kts file
    let mut ktsfile = File::create(ktsfile_name)?;

    }

    // ktsfile.write_all("Hola, Work In Progress")?;

    //println! (" CONTENT - {}",read_from_buildfile(&args.path.display().to_string()));

    Ok(())

}

use std::{fs, fs::*, process::exit, path::Path};
use clap::Parser;
use colored::Colorize;
use text_io::read;

#[derive(Debug, Parser)]
#[command(author = "Garcez", version = "0.1.0", about = "Command-Line Dictionary Manager", long_about = None)]
struct Args {
    #[arg(short = 'd', long = "database")]
    database: String,
}

fn err_msg(b: bool, err_msg: &str) -> Result<String, &'static str> {
    let msg: String;
    if b { 
	msg = format!("{} {} {}", "[CLDM]".bold(), "error:".bold().red(), err_msg);
	Ok(msg)
    } else {
	Err("It seems the err_msg bool is false. If this is not an error, then you shouldn't use this.")
    }
}

fn main() {
    // Parse arguments
    let args = Args::parse();
    // Check if argument is alright
    if !args.database.contains(".dm") {
	println!("{}", err_msg(true, ".dm file is needed to be readen. Add (or create) a .dm file with the -d flag").unwrap());
	exit(0)
    }
    // Add ./dicts/ to arguments as default folder (for now)
    let mut dm = "./dicts/".to_string();
    dm.push_str(&args.database);
    
    println!("{:#?}", database(&dm).unwrap());
}

fn database(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
	match fs::read_to_string(path) {
	Err(e) => return Err(format!("{}", err_msg(true, &e.to_string()).unwrap())),
	Ok(contents) => Ok(contents)
	}
    } else {
	print!("{} Create your headers, separating them with vertical bars (no spaces).\nFor example, «Name|Meaning|Etymology»\n\n> ", "[CLDM]:".bold());
	let mut headers: String = read!();
	headers.push_str(&"\n");
	let _ = match File::create(path) {
	    Err(e) => Err(format!("{}", err_msg(true, &e.to_string()).unwrap())),
	    Ok(f) => Ok(f)
	};
	let _ = fs::write(path, headers);
	match fs::read_to_string(path) {
	    Err(e) => return Err(format!("{}", err_msg(true, &e.to_string()).unwrap())),
	    Ok(contents) => Ok(contents)
	}
    }
}

fn add_items()

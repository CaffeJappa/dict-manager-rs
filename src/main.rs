use std::{fs, fs::*, io::{stdin, stdout, BufRead, Error, Write}, process::exit, path::Path};
use clap::Parser;
use colored::Colorize;
use text_io::read;
use clearscreen;

#[derive(Debug, Parser)]
#[command(author = "Garcez", version = "0.1.0", about = "Command-Line Dictionary Manager", long_about = None)]
struct Args {
    #[arg(short = 'd', long = "database")]
    database: String,
}

fn err_msg(err_msg: &str) -> String {
    format!("{} {} {}", "[CLDM]".bold(), "error:".bold().red(), err_msg)
}

fn main() -> Result<(), String> {
    // Parse arguments
    let args = Args::parse();

    // Clear the screen
    clearscreen::clear().expect(&format!("{} Failed to clear the screen.", "[CLDM]:".bold()));

    // Check if argument is alright
    if !args.database.contains(".dm") {
	println!("{}", err_msg(".dm file is needed to be readen. Add (or create) a .dm file with the -d flag"));
	exit(0)
    }

    // Add ./dicts/ to arguments as default folder (for now)
    let mut dm = "./dicts/".to_string();

    // Check path dm exists
    match check_path(&dm) {
	Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
	Ok(()) => ()
    }
    dm.push_str(&args.database);
    
    let result: Result<String, Error> = match database(&dm) {
	Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
	Ok(s) => Ok(s)
    };
    println!("{} Current entries \n\n{}", "[CLDM]:".bold(), result.unwrap());
    match add_entries(&dm) {
	Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
	Ok(()) => Ok(())
    }
}

fn database(path: &str) -> Result<String, String> {
    if Path::new(path).exists() {
	match fs::read_to_string(path) {
	Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
	Ok(contents) => Ok(contents)
	}
    } else {
	print!("{} Create your headers, separating them with vertical bars (no spaces).\nFor example, «Name|Meaning|Etymology»\n\n> ", "[CLDM]:".bold());
	let mut headers: String = read!();
	headers.push_str(&"\n\n");
	let _ = match File::create(path) {
	    Err(e) => Err(format!("{}", err_msg(&e.to_string()))),
	    Ok(f) => Ok(f)
	};
	let _ = fs::write(path, headers);
	match fs::read_to_string(path) {
	    Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
	    Ok(contents) => Ok(contents)
	}
    }
}

fn check_path(path: &str) -> Result<(), String> {
    if !Path::new(path).exists() {
	loop {
	    print!("{}", err_msg("Default dictionaries' path doesn't exist, do you want to create one? [Y/n] "));
	    let read: String = read!();
	    if read.to_lowercase().contains('y') {
		match fs::create_dir(path) {
		    Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
		    Ok(_) => println!("{} Path created at {:?}.", "[CLDM]:".bold(), path)
		};
		break
	    } else if read.to_lowercase().contains('n') {
		exit(1)
	    } else {
		continue
	    }
	}
    };
    Ok(())
}

fn add_entries(path: &str) -> Result<(), String> {
    loop {
	print!("{} Do you want to create new entries in your dictionary? [Y/n] ", "[CLDM]:".bold());
	let answer: String = read!();
	if answer.to_lowercase().contains('y') {
	    print!("{} Type in your new entry. When you're done, type Control-D or leave it blank and press Enter to exit.\n\n", "[CLDM]:".bold());
	    let mut file = OpenOptions::new().append(true).open(&path).unwrap();
	    loop {
		print!("{} > ", "[CLDM]".bold());
		let _ = stdout().flush().expect(&format!("{} Couldn't flush the stdout.", "[CLDM]:".bold()));
		let mut buffer: String = String::new();
		let stdin = stdin();
		let mut handle = stdin.lock();
		let bytes = handle.read_line(&mut buffer).expect(&format!("{} Couldn't read the line.", "[CLDM]:".bold()));
		match write!(file, "{}", &buffer) {
		    Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
		    Ok(()) => ()
		};
		if bytes <= 1 {
		    println!("\n{} Your changes have been saved.", "[CLDM]:".bold());
		    break;
		}
		continue
	    }
	} else if answer.to_lowercase().contains('n') {
	    match main() {
		Err(e) => return Err(format!("{}", err_msg(&e.to_string()))),
		Ok(()) => ()
	    }
	} else {
	    continue
	}
    }
}

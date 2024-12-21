use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    get_args(1);
}


fn get_args(number_ofargs: usize) {
    // args
    let args: Vec<String> = env::args().collect();
    let argument = &args[number_ofargs].as_str();

    // Get args
    match args[1].as_str() {
        "-h" | "--help" => get_help(),
        "-add" => add_anime(),
        "--add" => add_anime_nomenu(),
        _ => println!("Argument: {argument} not found")
    }
}

fn get_anime_file(name_arg: usize,add_string: &str) {
     // args
    let args: Vec<String> = env::args().collect();

    // set anime name
         let anime_name = &args[name_arg].as_str();
    // set default path var
    let default_path = "/usr/share/anicli-list/animes/db/"; 
    // set vars
    let mut anime_file = String::new();        
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};
        //// redirect the input for a file
        // create the file anime.conf
        anime_file = {anime_path}.to_owned()+"/anime.conf";
 
        let mut edit_anime_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(anime_file)
            .unwrap();
            if let Err(anime_file) = writeln!(edit_anime_file, "\n{}", add_string) {
        eprintln!("Couldn't write to file: {}", anime_file);    
    }    
}

fn get_help() {
    // File
    let mut help_file = File::open("/usr/share/anicli-list/docs/help.txt").expect("File help.txt not founded");
    // File content
    let mut help_content = String::new();
    help_file.read_to_string(&mut help_content)
        .expect("help file can not be read");

    // Print help
    println!("{help_content}");
}


fn add_anime() {
    // set default path var
    let default_path = "/usr/share/anicli-list/animes/db/"; 

    // vars
    let mut anime_name        = String::new();
    let mut anime_status      = String::new();
    let mut _edit_anime_file  = String::new();
    let mut anime_episode     = String::new();
    let mut anime_season      = String::new();
    let mut _len              = String::new();
    let mut _anime_path       = String::new();
    let mut anime_file        = String::new();

        // add a anime
// anime name
        print!("What is the anime name?:");
        io::stdout().flush().unwrap();

        //// user input
        let _ = io::stdin()
            .read_line(&mut anime_name)
            .expect("Failed to read line");
        
        //// delete newlines
        let mut len = anime_name.len();
        anime_name.truncate(len - 1);
        
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};
        let _ = fs::create_dir(anime_path.clone());
        //// redirect the input for a file
        // create the file anime.conf
        let anime_name_compose = "name: ".to_owned()+{&anime_name};
        anime_file = {anime_path}.to_owned()+"/anime.conf";

        let _ = File::create(anime_file.clone());
        // redirect input
        fs::write(anime_file.clone(), anime_name_compose)
            .expect("The file anime.conf not exist");
// anime name end



// anime status
        print!("What is the anime status?:");
        io::stdout().flush().unwrap();

        //// user input
        let _ = io::stdin()
            .read_line(&mut anime_status)
            .expect("Failed to read line");
        
        //// delete newlines
        len = anime_status.len();
        anime_status.truncate(len - 1);
        //// format anime status
        anime_status = "status: ".to_owned()+(&anime_status);
        //// redirect the input for a file       
        let mut edit_anime_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(anime_file)
            .unwrap();
            if let Err(anime_file) = writeln!(edit_anime_file, "\n{}", anime_status) {
        eprintln!("Couldn't write to file: {}", anime_file);
    }
// anime status end



// anime episode
        print!("What is the anime episode?:");
        io::stdout().flush().unwrap();

        //// user input
        let _ = io::stdin()
            .read_line(&mut anime_episode)
            .expect("Failed to read line");
        
        //// delete newlines
        len = anime_episode.len();
        anime_episode.truncate(len - 1);
        //// format anime episode
        anime_episode = "episode: ".to_owned()+(&anime_episode);
        //// redirect the input for a file
            if let Err(anime_file) = writeln!(edit_anime_file, "{}", anime_episode) {
        eprintln!("Couldn't write to file: {}", anime_file);
    }

// anime episode end



// anime season
        print!("What is the anime season?:");
        io::stdout().flush().unwrap();
        
        //// user input
        let _ = io::stdin()
            .read_line(&mut anime_season)
            .expect("Failed to read line");
        
        //// user input
        len = anime_season.len();
        anime_season.truncate(len - 1);
        //// format anime season
        anime_season = "season: ".to_owned()+(&anime_season);
        //// redirect anime for a file
            if let Err(anime_file) = writeln!(edit_anime_file, "{}", anime_season) {
        eprintln!("Couldn't write to file: {}", anime_file);
    }
// anime season end
}

fn add_anime_nomenu() {
// set args
    let args: Vec<String> = env::args().collect();
    // set anime name
         let anime_name = &args[2].as_str();
    // set default path var
    let default_path = "/usr/share/anicli-list/animes/db/"; 
    // set vars
    let mut anime_file = String::new();

        
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};
        let _ = fs::create_dir(anime_path.clone());
        //// redirect the input for a file
        // create the file anime.conf
        let anime_name_compose = "name: ".to_owned()+{&anime_name};
        anime_file = {anime_path}.to_owned()+"/anime.conf";

        let _ = File::create(anime_file.clone());
        // redirect input
        fs::write(anime_file.clone(), anime_name_compose)
            .expect("The file anime.conf not exist");

// anime name end

   
// anime status
     // args check
     match args[3].as_str() {
        "--status" => status_anime_nomenu(),
        _ => println!("Argument: {} not found", &args[3].as_str())

}
        fn status_anime_nomenu() {
// set args
    let args: Vec<String> = env::args().collect();
// set anime status
        let anime_status = &args[4].as_str();
// anime status
        //// format anime status
        let anime_status = "status: ".to_owned()+{&anime_status};
        //// redirect the input for a file       
        get_anime_file(2, &anime_status); 
        println!("{}", &anime_status);
        }
// anime status end
    }


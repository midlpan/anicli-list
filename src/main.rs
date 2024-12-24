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
        "--add" => add_anime_nointeractive(),
        "-del" => del_anime(),
        _ => println!("Argument: {argument} not found")
    }
}



fn add_anime_file(name_arg: usize,new_line: usize,add_string: &str) {
     // args
    let args: Vec<String> = env::args().collect();

    // set anime name
         let anime_name = &args[name_arg].as_str();

    // set default path var
    let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
    let mut default_path = String::new();

    db_conf_file.read_to_string(&mut default_path)
        .expect("Can not read the file: path.conf");
    //// format default_path
    let default_path    = default_path.replace("\n", "");


        //// anime path
        let default_path = {default_path}.to_owned()+{&anime_name};
        //// redirect the input for a file
        // create the file anime.conf
        let anime_file = {default_path}.to_owned()+"/anime.conf";

        if new_line == 1 {
        let mut edit_anime_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(anime_file)
            .unwrap();
            if let Err(anime_file) = writeln!(edit_anime_file, "\n{}", add_string) {
        eprintln!("Couldn't write to file: {}", anime_file);
    }   } else if new_line == 0 {
         let mut edit_anime_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(anime_file)
            .unwrap();
            if let Err(anime_file) = writeln!(edit_anime_file, "{}", add_string) {
        eprintln!("Couldn't write to file: {}", anime_file);
        }
}
}


fn get_help() {
    // File
    let help_file     = "docs/help.txt";
    let mut help_file = File::open(help_file).expect("File help.txt not founded");
    

    // File content
    let mut help_content = String::new();
    help_file.read_to_string(&mut help_content)
        .expect("help file can not be read");

    // Print help
    println!("{help_content}");
}


fn add_anime() {

    // vars
    let mut anime_name        = String::new();
    let mut anime_status      = String::new();
    let mut _edit_anime_file  = String::new();
    let mut anime_episode     = String::new();
    let mut anime_season      = String::new();
    let mut _len              = String::new();

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


        // set default path var
        let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
        let mut default_path = String::new();

        db_conf_file.read_to_string(&mut default_path)
            .expect("Can not read the file: db_path.txt");
        //// format default_path
        let default_path    = default_path.replace("\n", "");


        //// redirect the input for a file
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};
        let _ = fs::create_dir(anime_path.clone());

        // create the file anime.conf
        let anime_name_compose = "name: ".to_owned()+{&anime_name};
        let anime_file = {anime_path}.to_owned()+"/anime.conf";


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

fn add_anime_nointeractive() {
    // Set args
    let args: Vec<String> = env::args().collect();
    // Set anime name
    let anime_name = &args[2].as_str();
    // Set anime file

        // set default path var
        let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
        let mut default_path = String::new();

        db_conf_file.read_to_string(&mut default_path)
            .expect("Can not read the file: db_path.txt");
        //// format default_path
        let default_path    = default_path.replace("\n", "");



        //// redirect the input for a file
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};
        let _ = fs::create_dir(anime_path.clone());

        // create the file anime.conf
        let anime_name_compose = "name: ".to_owned()+{&anime_name};
        let anime_file = {anime_path}.to_owned()+"/anime.conf";

        let _ = File::create(anime_file.clone());
        // redirect input
        fs::write(anime_file.clone(), anime_name_compose)
            .expect("The file anime.conf not exist");

// anime name nointeractive end


// anime status nointeractive
     // args check
     match args[3].as_str() {
        "--status" => status_anime_nointeractive(),
        _ => println!("Argument: {} not found", &args[3].as_str())

}
        fn status_anime_nointeractive() {
        // set args
        let args: Vec<String> = env::args().collect();
        // set anime status
        let anime_status = &args[4].as_str();
        //// format anime status
        let anime_status = "status: ".to_owned()+{&anime_status};

        //// redirect the input for a file
        add_anime_file(2,1, &anime_status);
    }
// anime status nointeractive end

// anime episode nointeractive
    match args[5].as_str() {
        "--ep" => add_anime_episode_nointeractive(),
        _ => println!("Argument: {} not found", &args[5].as_str())
}

        fn add_anime_episode_nointeractive() {
        // set args
        let args: Vec<String> = env::args().collect();

        // set anime episode
        let anime_episode = &args[6].as_str();

        // format anime episode
        let anime_episode = "episode: ".to_owned()+{&anime_episode};

        //// redirect the input for a file
        add_anime_file(2,0, &anime_episode);
        }
// anime episode nointeractive end

// anime season nointeractive
    match args[7].as_str() {
        "--season" => add_anime_season_nointeractive(),
        _ => println!("Argument: {} not found", &args[7].as_str())
}

        fn add_anime_season_nointeractive() {
        // set args
        let args: Vec<String> = env::args().collect();


        // set anime season
        let anime_season = &args[8].as_str();

        // format anime season
        let anime_season = "season: ".to_owned()+{&anime_season};

        //// redirect the input for a file
        add_anime_file(2,0, &anime_season)
        }
// anime season nointeractive end
}





// DELETE A ANIME

fn del_anime() {
        // variables
        let mut anime_name = String::new();
        let mut question   = String::new();

        print!("What is the anime name?:");
        io::stdout().flush().unwrap();

        //// user input
        let _ = io::stdin()
            .read_line(&mut anime_name)
            .expect("Failed to read line");
        //// Set anime file
        // Set anime path
        let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
        let mut default_path = String::new();
        db_conf_file.read_to_string(&mut default_path)
            .expect("Can not read the file: db_path.txt");
        //// format default_path
        let default_path    = default_path.replace("\n", "");

        // Set anime path
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};

        let anime_file = anime_path.replace("\n", "");


        //// find anime file
        match fs::exists(anime_file.clone()) {
         Ok(true) => println!("The anime archive exists"),
         Ok(false) => {
             println!("Anime file does not exist");
             std::process::exit(0);
         },
         Err(e) => println!("An error occurred when checking the anime file: {}", e),
        }

        //// Do you really want to remove the anime?
        print!("Do you really want to remove the anime[Y|N]?:");
        io::stdout().flush().unwrap();

        // user input
        let _ = io::stdin()
            .read_line(&mut question)
            .expect("Failed to read line");

        let question: &str = question.as_str();

    match question {
        "Y\n"  => {
            match fs::remove_dir_all(anime_file) {
                Ok(_) => println!("Anime has be deleted"),
                Err(e) => println!("An error occurred while removing the anime: {}", e),
            }
        },
        "N\n" => println!("Anime not removed."),
        _ => println!("Option not valid\nOption: {}", question),
    }
}

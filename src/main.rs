use std::path::Path;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::io;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

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
        "-add"  => add_anime(),
        "--add" => add_anime_nointeractive(),
        "-del"  => del_anime(),
        "--del" => del_anime_nointeractive(),
        "-rconf" => reconfig_anime().expect("A error ocured reconfiguring"),
        "--rconf" => reconfig_anime_nointeractive(),
        _ => println!("Argument: {argument} not found")
    }
}



fn add_anime_file(name_arg: usize,new_line: usize,add_string: &str) {
    // args
    let args: Vec<String> = env::args().collect();


    // set anime name
    let anime_name = &args[name_arg].as_str()
        .replace("/",    "#")
        .replace("\\",  "##")
        .replace("*",  "###")
        .replace("?", "####");


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
            .expect("Can not read the file: path.conf");
        //// format default_path
        let default_path    = default_path.replace("\n", "");


        //// redirect the input for a file
        //// anime path
        // Create anime name
        let path_anime_name = &anime_name.clone()
            .replace("/",   "#")
            .replace("\\",  "##")
            .replace("*",  "###")
            .replace("?", "####");
        let anime_path = {default_path}.to_owned()+{&path_anime_name};
        let _ = fs::create_dir(anime_path.clone());

        // create the file anime.conf
        let anime_name_compose = "name: ".to_owned()+{&anime_name};
        let anime_file = {anime_path}.to_owned()+"/anime.conf";


        let _ = File::create(anime_file.clone());
        // redirect input
        fs::write(anime_file.clone(), anime_name_compose)
            .expect("The file anime.conf not exists");
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
            .expect("Can not read the file: path.conf");
        //// format default_path
        let default_path    = default_path.replace("\n", "");



        //// redirect the input for a file
        // Set path name
        let anime_path_name = &anime_name
            .replace("/",    "#")
            .replace("\\",  "##")
            .replace("*",  "###")
            .replace("?", "####");

        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_path_name};
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
            .expect("Can not read the file: path.conf");
        //// format default_path
        let default_path    = default_path.replace("\n", "");

        // Set anime path
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};

        let anime_file = anime_path.replace("\n", "");

        anime_file_exists(anime_file.clone());

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



// Delete anime non-interactively

fn del_anime_nointeractive() {
        // Set question var
        let mut question          = String::new();

        //// Set args
        let args: Vec<String> = env::args().collect();
        // Set anime name
        let anime_name        = &args[2].as_str();

        //// Set anime file
        // Set anime path
        let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
        let mut default_path = String::new();
        db_conf_file.read_to_string(&mut default_path)
            .expect("Can not read the file: path.conf");
        //// format default_path
        let default_path    = default_path.replace("\n", "");

        // Set anime path
        //// anime path
        let anime_path = {default_path}.to_owned()+{&anime_name};

        let anime_file = anime_path.replace("\n", "");

        anime_file_exists(anime_file.clone());

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


fn replace_string_infile(new_string: &str,old_string: &str,file: &str) -> io::Result<()> {
    // Read the file
    let content = fs::read_to_string(file)?;
    // Replace the string
    let new_content = content.replace(old_string, new_string);

    // Rewrite the file
    let mut file = fs::File::create(file)?;
    file.write_all(new_content.as_bytes())?;
    Ok(())
}



fn reconfig_anime() -> Result<(), Box<dyn Error>>  {
        // Set default vars
        let mut new_anime_status   = String::new();
        let mut anime_name         = String::new();
        let mut option             = String::new();
        let mut old_anime_name     = String::new();
        let mut new_anime_name     = String::new();
        let mut new_anime_episode  = String::new();
        let mut new_anime_season   = String::new();

        // Set anime path
        let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
        let mut default_path = String::new();
            db_conf_file.read_to_string(&mut default_path)
                    .expect("Can not read the file: path.conf");
        //// Format default path
        let default_path   = default_path.replace("\n", "");

        // Print options
        println!("[name]");
        println!("[status]");
        println!("[episode]");
        println!("[season]");
        // Select option
        print!("Option:");
        io::stdout().flush().unwrap();

        //// user input
        let _ = io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");


        match option.as_str() {
            // Reconfig anime name
            "name\n" => {
                        print!("What is the anime name?:");
                        io::stdout().flush().unwrap();

                        //// user input
                        let _ = io::stdin()
                            .read_line(&mut old_anime_name)
                            .expect("Failed to read line");
                        let old_anime_name = old_anime_name
                            .replace("\n", "")
                            .replace("/",    "#")
                            .replace("\\",  "##")
                            .replace("*",  "###")
                            .replace("?", "####");
                        //// anime path
                        let old_anime_path = {default_path.clone()}.to_owned()+{&old_anime_name};

                        //// Check if anime path exists
                        anime_file_exists(old_anime_path.clone());

                        //// user input
                        print!("What is the new anime name?:");
                        io::stdout().flush().unwrap();
                        let _ = io::stdin()
                            .read_line(&mut new_anime_name)
                            .expect("Failed to read line");
                        let new_anime_name = new_anime_name
                            .replace("\n", "")
                            .replace("/",    "#")
                            .replace("\\",  "##")
                            .replace("*",  "###")
                            .replace("?", "####");

                        // New anime path and file
                        let new_anime_path = {default_path}.to_owned()+{&new_anime_name};
                        let new_anime_file = {new_anime_path.clone()}.to_owned()+"/anime.conf";
                        // Rename the old anime path
                        let _ = fs::rename(old_anime_path, new_anime_path);

                        // Format new_anime_name and old_anime_name
                        let new_anime_name = "name: ".to_owned()+{&new_anime_name};
                        let old_anime_name = "name: ".to_owned()+{&old_anime_name};

                        // Config variables
                        let new_anime_name = new_anime_name.as_str();
                        let new_anime_file = new_anime_file.as_str();
                        let old_anime_name = old_anime_name.as_str();

                        // replace
                        let _ = replace_string_infile(&new_anime_name, &old_anime_name, &new_anime_file);
                        Ok(())
        },
        // Reconfig anime status
            "status\n" => {
                        print!("What is the anime name?:");
                        io::stdout().flush().unwrap();

                        //// user input
                        let _ = io::stdin()
                            .read_line(&mut anime_name)
                            .expect("Failed to read line");
                        let anime_name = anime_name.replace("\n", "");

                        //// Set anime path
                        let anime_path     = {default_path}.to_owned()+{&anime_name};
                        //// Set anime file
                        let anime_file     = {anime_path.clone()}.to_owned()+"/anime.conf";
                        // find anime file
                        anime_file_exists(anime_file.clone());

                        //// user input
                        print!("What is the new anime status?:");
                        io::stdout().flush().unwrap();

                        let _ = io::stdin()
                            .read_line(&mut new_anime_status)
                            .expect("Failed to read line");
                        let new_anime_status = new_anime_status
                            .replace("\n", "")
                            .replace("/",    "#")
                            .replace("\\",  "##")
                            .replace("*",  "###")
                            .replace("?", "####");


                        // Set old_anime status
                        //// Set regex
                        let regex = Regex::new("status: ").unwrap();

                        // Format variables
                        let anime_file          = anime_file.as_str();
                        let new_anime_status    = "status: ".to_owned()+&new_anime_status;
                        let new_anime_status    = new_anime_status.as_str();

                        let _ = grep_string_in_file_and_replace(regex, anime_file, new_anime_status);

                        Ok(())
        },
        // Reconfig anime episode
            "episode\n" => {

                        print!("What is the anime name?:");
                        io::stdout().flush().unwrap();

                        //// user input
                        let _ = io::stdin()
                            .read_line(&mut anime_name)
                            .expect("Failed to read line");
                        let anime_name = anime_name
                            .replace("\n", "")
                            .replace("/",    "#")
                            .replace("\\",  "##")
                            .replace("*",  "###")
                            .replace("?", "####");



                        //// Set anime path
                        let anime_path     = {default_path}.to_owned()+{&anime_name};
                        //// Set anime file
                        let anime_file     = {anime_path.clone()}.to_owned()+"/anime.conf";
                        // find anime file
                        anime_file_exists(anime_file.clone());

                        //// user input
                        print!("What is the new anime episode?:");
                        io::stdout().flush().unwrap();

                        let _ = io::stdin()
                            .read_line(&mut new_anime_episode)
                            .expect("Failed to read line");
                        let new_anime_episode = new_anime_episode.replace("\n", "");

                        // Set old_anime episode
                        //// Set regex
                        let regex = Regex::new("episode: ").unwrap();

                        // Format variables
                        let anime_file           = anime_file.as_str();
                        let new_anime_episode    = "episode: ".to_owned()+&new_anime_episode;
                        let new_anime_episode    = new_anime_episode.as_str();

                        let _ = grep_string_in_file_and_replace(regex, anime_file, new_anime_episode);
                        Ok(())
        },
        // Reconfig anime season
            "season\n" => {

                        print!("What is the anime name?:");
                        io::stdout().flush().unwrap();

                        //// user input
                        let _ = io::stdin()
                            .read_line(&mut anime_name)
                            .expect("Failed to read line");
                        let anime_name = anime_name
                            .replace("\n", "")
                            .replace("/",    "#")
                            .replace("\\",  "##")
                            .replace("*",  "###")
                            .replace("?", "####");

                        //// Set anime path
                        let anime_path     = {default_path}.to_owned()+{&anime_name};
                        //// Set anime file
                        let anime_file     = {anime_path.clone()}.to_owned()+"/anime.conf";
                        // find anime file
                        anime_file_exists(anime_file.clone());

                        //// user input
                        print!("What is the new anime season?:");
                        io::stdout().flush().unwrap();

                        let _ = io::stdin()
                            .read_line(&mut new_anime_season)
                            .expect("Failed to read line");
                        let new_anime_season = new_anime_season.replace("\n", "");

                        // Set old_anime episode
                        //// Set regex
                        let regex = Regex::new("season: ").unwrap();

                        // Format variables
                        let anime_file         = anime_file.as_str();
                        let new_anime_season   = "season: ".to_owned()+&new_anime_season;
                        let new_anime_season   = new_anime_season.as_str();

                        let _ = grep_string_in_file_and_replace(regex, anime_file, new_anime_season);
                        Ok(())

    },
        _ => {
                let option = option.replace("\n", "");
                println!("Option: '{}' not exists", option);
                Ok(())
    },
  }
}


fn reconfig_anime_nointeractive() {

        // Set args
        let args: Vec<String> = env::args().collect();

        // Set anime path
        let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
        let mut default_path = String::new();
            db_conf_file.read_to_string(&mut default_path)
                    .expect("Can not read the file: path.conf");
        //// Format default path
        let default_path   = default_path.replace("\n", "");

        // Set anime name
        let anime_name = &args[2].as_str()
            .replace("/",    "#")
            .replace("\\",  "##")
            .replace("*",  "###")
            .replace("?", "####");


        // Set anime file
        //// Set anime path
        let anime_path = {default_path.clone()}.to_owned()+{&anime_name};

        //// Check if anime path exists
        anime_file_exists(anime_path.clone());

        //// Set anime file
        let anime_file = {anime_path.clone()}.to_owned()+"/anime.conf";



        // Reconfig anime status without interaction
        match args[3].as_str() {
            "--name"    => {
                let old_anime_name = anime_name;
                let new_anime_name = &args[4].as_str()
                    .replace("/",    "#")
                    .replace("\\",  "##")
                    .replace("*",  "###")
                    .replace("?", "####");

                //// anime path
                let old_anime_path = {default_path.clone()}.to_owned()+{&old_anime_name};


                // New anime path and file
                let new_anime_path = {default_path}.to_owned()+{&new_anime_name};
                let new_anime_file = {new_anime_path.clone()}.to_owned()+"/anime.conf";
                // Rename the old anime path
                let _ = fs::rename(old_anime_path, new_anime_path);

                // Format new_anime_name and old_anime_name
                let new_anime_name = "name: ".to_owned()+{&new_anime_name};
                let old_anime_name = "name: ".to_owned()+{&old_anime_name};

                // Config variables
                let new_anime_name = new_anime_name.as_str();
                let new_anime_file = new_anime_file.as_str();
                let old_anime_name = old_anime_name.as_str();

                // replace
                let _ = replace_string_infile(&new_anime_name, &old_anime_name, &new_anime_file);
        },
         "--status"  => {
                 // Set new anime status
                 let new_anime_status = &args[4].as_str();

                 // Set old_anime status
                 //// Set regex
                 let regex = Regex::new("status: ").unwrap();

                 // Format variables
                 let anime_file          = anime_file.as_str();
                 let new_anime_status    = "status: ".to_owned()+&new_anime_status;
                 let new_anime_status    = new_anime_status.as_str();

                 let _ = grep_string_in_file_and_replace(regex, anime_file, new_anime_status);

         },
         "--ep" => {
                 // Set new anime seaspm 
                 let new_anime_episode = &args[4].as_str();

                 // Set old_anime season 
                 //// Set regex
                 let regex = Regex::new("episode: ").unwrap();

                 // Format variables
                 let anime_file          = anime_file.as_str();
                 let new_anime_episode   = "episode: ".to_owned()+&new_anime_episode;
                 let new_anime_episode   = new_anime_episode.as_str();

                 let _ = grep_string_in_file_and_replace(regex, anime_file, new_anime_episode);
         },
         "--season" => {
                 // Set new anime season 
                 let new_anime_season = &args[4].as_str();

                 // Set old_anime season 
                 //// Set regex
                 let regex = Regex::new("season: ").unwrap();

                 // Format variables
                 let anime_file          = anime_file.as_str();
                 let new_anime_season    = "season: ".to_owned()+&new_anime_season;
                 let new_anime_season    = new_anime_season.as_str();

                 let _ = grep_string_in_file_and_replace(regex, anime_file, new_anime_season);

         },
         _ => println!("Argument not exist: {}", &args[3].as_str()),
    }
}


fn anime_file_exists(file: String) {
//// Check if the file exist
match fs::exists(file) {
        Ok(true) => println!("The anime archive exists"),
        Ok(false) => {
                            println!("Anime file does not exist");
                            std::process::exit(1);
                     },
        Err(e) => println!("An error occurred when checking the anime file: {}", e),
     }
}



fn grep_string_in_file_and_replace(regex: Regex, file: &str, new_string: &str) -> io::Result<()> {

    // Format vars
    //// set new string
    let new_string = &new_string;
    //// set anime file
    let anime_file = &file;
    //// set file to search
    let path = Path::new(&file);
    let file = File::open(&path)?;

    // Find string
    let reader = io::BufReader::new(file);
                for (_index, line) in reader.lines().enumerate() {
                    let line = line?;
                        if regex.clone().is_match(&line) {
                            let old_string = &line;
                            let _ = replace_string_infile(new_string, old_string, anime_file);
                        }
                }
    Ok(())
}


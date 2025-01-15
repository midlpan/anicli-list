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
        "-rconf" => reconfig_anime().expect("An error occurred reconfiguring"),
        "--rconf" => reconfig_anime_nointeractive(),
        "--list" => print_anime_list().expect("An error occurred showing the list"),
        "--show" => print_specific_anime().expect("An error occurred showing a specific anime in the list"),
        "-waifu" => add_waifu(),
        "--waifu" => add_waifu_nointeractive(),
        _ => println!("Argument: {argument} not found")
    }
}


fn replace_characters_to_files(string: &str) -> String {
    let string = string
        .replace("/",    "#")
        .replace("\\",  "##")
        .replace("*",  "###")
        .replace("?", "####");
    string
}

fn set_default_path() -> String {
    // set default path var
    let mut db_conf_file = File::open("path.conf").expect("File path.conf not exists");
    let mut default_path = String::new();

    db_conf_file.read_to_string(&mut default_path)
        .expect("Can not read the file: path.conf");
    let default_path     = default_path.replace("\n", "");
    anime_file_exists(default_path.clone());
    default_path
}

fn add_anime_file(name_arg: usize,new_line: usize,add_string: &str) {
    // args
    let args: Vec<String> = env::args().collect();


    // set anime name
    let anime_name   = replace_characters_to_files(&args[name_arg]);

    let default_path = set_default_path();

        //// anime path
        let default_path = {default_path}.to_owned()+{&anime_name};
        //// redirect the input for a file
        // create the file anime.conf
        let anime_file = {default_path}.to_owned()+"/anime.conf";

        if new_line == 1 {
          write_in_file(anime_file,add_string);
        } else if new_line == 0 {
          write_in_file(anime_file,add_string);
    }
}



fn get_help() {
    // Set Help
    let help = "    Usage:
             anicli-list [ARG]
             anicli-list --add [ANIME NAME] --status [ANIME STATUS] --ep [ANIME EPISODE] --season [ANIME SEASON] # In this exact order]
             anicli-list --rconf [ANIME NAME] [ARG] [TEXT]
             anicli-list --show [ANIME NAME]
             anicli-list --waifu [WAIFU] --anime [ANIME NAME]
    Options:
             --help | -h        Print this message
             -add               Add a anime
             -del               Delete a anime
             -rconf             Reconfigure a anime
             --list             Print all animes in the list
             -waifu             Define your waifu
             --del-waifu        Delete a waifu
             --show             Show the configuration of a specific anime
             --add-rank         Rank an anime
             --del-rank         Delete a rank
             -rank              Show the rank

             --add              Add an anime without interactive questions
             --del              Delete an anime without interactive questions
             --rconf            Reconfigure an anime without interactive questions
             --waifu            Add an anime waifu from no interactive questions
    The --add Options:
             --status           Add an anime status without a interactive questions
             --ep               Add an anime episode without a interactive questions
             --season           Add an anime season without a interactive questions
    The --rconf Options:
             --name             Reconfigure the name of an anime without asking any questions
             --status           Reconfigures an anime's status without intractive questions
             --ep               Reconfigure an anime episode without interactive questions
             --season           Reconfigure an anime season without interactive questions
             --waifu            Reconfigure an anime waifu without interactive questions
    WARNING: Only one argument can be used at a time";
    // Print help
    println!("{help}");
}

fn write_in_file(file: String,write: &str) {
     //// redirect the input for a file
        let mut edit_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(file)
            .unwrap();
            if let Err(file) = writeln!(edit_file, "\n{}", write) {
        eprintln!("Couldn't write to file: {}", file);
    }
}

fn add_anime() {

    // vars
    let mut anime_name        = String::new();
    let mut anime_status      = String::new();
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

        // Set default path
        let default_path = set_default_path();

        //// redirect the input for a file
        //// anime path
        // Create anime name
        let path_anime_name = replace_characters_to_files(&anime_name.clone());
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
        write_in_file(anime_file.clone(),&anime_status);

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
        write_in_file(anime_file.clone(),&anime_episode);

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
        write_in_file(anime_file,&anime_season);
}
// anime season end

fn add_anime_nointeractive() {
    // Set args
    let args: Vec<String> = env::args().collect();
    // Set anime name
    let anime_name = &args[2];
    // Set anime file

        // Set default path
        let default_path = set_default_path();


        //// redirect the input for a file
        // Set path name
        let anime_path_name = replace_characters_to_files(&anime_name);

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
        let anime_status = &args[4];
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
        let anime_episode = &args[6];

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
        let anime_season = &args[8];

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
        let anime_name = replace_characters_to_files(&anime_name);
        //// Set anime file
        // Set default path
        let default_path    = set_default_path();

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

        let question = question.as_str();

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
        let anime_name        = replace_characters_to_files(&anime_name);
        //// Set anime file

        // Set default path
        let default_path    = set_default_path();

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

        let question = question.as_str();

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
        let mut new_anime_waifu    = String::new();

        // Set default path
        let default_path = set_default_path();

        // Print options
        println!("[name]");
        println!("[status]");
        println!("[episode]");
        println!("[season]");
        println!("[waifu]");
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
                        let old_anime_name = replace_characters_to_files(&old_anime_name)
                            .replace("\n", "");
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
                        let new_anime_name = replace_characters_to_files(&new_anime_name)
                            .replace("\n", "");

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
                        let anime_name = replace_characters_to_files(&anime_name)
                            .replace("\n", "");
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

                        reconf_anime("status: ", &new_anime_status, &anime_file);
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
                        let anime_name = replace_characters_to_files(&anime_name)
                            .replace("\n", "");

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

                        reconf_anime("episode: ", &new_anime_episode, &anime_file);

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
                        let anime_name = replace_characters_to_files(&anime_name)
                            .replace("\n", "");

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
                        reconf_anime("season: ", &new_anime_season, &anime_file);
                        Ok(())
    },
        // Reconfig anime waifu UwU
            "waifu\n" => {

                        print!("What is the anime name?:");
                        io::stdout().flush().unwrap();

                        //// user input
                        let _ = io::stdin()
                            .read_line(&mut anime_name)
                            .expect("Failed to read line");
                        let anime_name = replace_characters_to_files(&anime_name)
                            .replace("\n", "");

                        //// Set anime path
                        let anime_path     = {default_path}.to_owned()+{&anime_name};
                        //// Set anime file
                        let anime_file     = {anime_path.clone()}.to_owned()+"/anime.conf";
                        // find anime file
                        anime_file_exists(anime_file.clone());

                        //// user input
                        print!("What is the new anime waifu UwU?:");
                        io::stdout().flush().unwrap();

                        let _ = io::stdin()
                            .read_line(&mut new_anime_waifu)
                            .expect("Failed to read line");
                        let new_anime_waifu = new_anime_waifu.replace("\n", "");
                        reconf_anime("waifu: ", &new_anime_waifu, &anime_file);
                        Ok(())
    },
        _ => {
                let option = option.replace("\n", "");
                println!("Option: '{}' not exists", option);
                Ok(())
    },
  }
}

fn reconf_anime(regex: &str,input: &str,file: &str) {
                 // Set old_anime status
                 //// Set regex
                 let find     = regex;
                 let regex    = Regex::new(regex).unwrap();
                 // Format variables
                 let input    = find.to_owned()+input;
                 let input    = input.as_str();

                 let _ = grep_string_in_file_and_replace(regex, file, input);
}


// Reconfig anime in a no interactive form
fn reconfig_anime_nointeractive() {

        // Set args
        let args: Vec<String> = env::args().collect();
        let new_info          = &args[4].as_str();

        // Set default path
        let default_path = set_default_path();

        // Set anime name
        let anime_name   = replace_characters_to_files(&args[2]);

        // Set anime file
        //// Set anime path
        let anime_path   = {default_path.clone()}.to_owned()+{&anime_name};

        //// Check if anime path exists
        anime_file_exists(anime_path.clone());

        //// Set anime file
        let anime_file   = {anime_path.clone()}.to_owned()+"/anime.conf";



        // Reconfig anime status without interaction
        match args[3].as_str() {
            "--name"    => {
                let old_anime_name = anime_name;
                let old_anime_name = replace_characters_to_files(old_anime_name.as_str());
                let new_anime_name = replace_characters_to_files(&args[4]);

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
                 reconf_anime("status: ", &new_info, &anime_file);
         },
         "--ep" => {
                 // Set new anime seaspm
                 reconf_anime("episode: ", &new_info, &anime_file);
         },
         "--season" => {
                 // Set new anime season
                 reconf_anime("season: ", &new_info, &anime_file);

         },
         "--waifu"  => {
                 // Set new anime waifu
                 reconf_anime("waifu: ", &new_info, &anime_file);
         },
         _ => println!("Argument not exist: {}", &args[3]),
    }
}

// Reconfig anime in a no interactive form end

fn anime_file_exists(file: String) {
//// Check if the file exist
match fs::exists(file) {
        Ok(true) => print!(""),
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




// Print Anime list
fn print_anime_list() -> io::Result<()> {
    //// format default_path
    let default_path    = set_default_path();
    let _ = show_subdirs(&default_path);
    Ok(())
}

fn show_subdirs(default_path: &str) -> io::Result<()>  {
    // Read Directory
    let read_path = fs::read_dir(default_path)?;

    for read_paths in read_path {
        let read_paths  = read_paths?;
        let view_path  = read_paths.path();

        if view_path.is_dir() {
            let _ = show_subdirs(view_path.to_str().unwrap())?;
            let _ = show_files_in_subdirs(&view_path);
        } else if view_path.is_file(){
            let _ = show_files_in_subdirs(&view_path);
        }

    }
    Ok(())
}

fn show_files_in_subdirs(default_path: &std::path::Path) -> io::Result<()> {
    let mut open_path = fs::File::open(default_path)?;
    let mut content   = String::new();
    let _ = open_path.read_to_string(&mut content);
    // A good CLI
    a_good_presentation(content);

    Ok(())
}
// Print anime list end


fn a_good_presentation(content: String) {
    // Make a good CLI
    // Create a border
    let border_lenght = content.len();
    let border_char   = "𝋯";
    // Show top border
    println!("{}", border_char.to_string().repeat(border_lenght));
    // Create a border behind the content
    let content = content.replace("name: ", "𝌆 name: ").replace("status: ", "𝌆 status: ").replace("episode: ", "𝌆 episode: ").replace("season: ", "𝌆 season: ").replace("\n", "").replace("𝌆 status: ", "\n𝌆 status: ").replace("𝌆 episode: ", "\n𝌆 episode: ").replace("𝌆 season: ", "\n𝌆 season: ").replace("waifu: ", "\n𝌆 waifu: ");
    // Print the content
    println!("{}", content);
    // Show ending edge
    println!("{}", border_char.to_string().repeat(border_lenght));


}

// Print a scpecific anime
fn print_specific_anime() -> io::Result<()> {
    // Set args
    let args: Vec<String> = env::args().collect();

    // Set default path
    let default_path    = set_default_path();

    //// Set selected anime
    let selected_anime = &args[2];
    let selected_anime = replace_characters_to_files(&selected_anime);
    let selected_anime = selected_anime.to_owned().to_owned()+"/";
    //// Find anime file
    let anime_path     = default_path.to_owned()+&selected_anime;
    let anime_file     = anime_path.to_owned()+"/anime.conf";
    anime_file_exists(anime_file.clone());

    // Read the file
    let anime_file_open = fs::File::open(anime_file);
    let mut content     = String::new();
    let _ = anime_file_open?.read_to_string(&mut content);
    // A good CLI
    a_good_presentation(content);
    Ok(())
}
// Print a scpecific anime end



// Add an anime waifu
fn add_waifu() {
// Set default vars
    let mut anime_name  = String::new();
    let mut anime_waifu = String::new();
    // anime name
        print!("What is the anime name?:");
        io::stdout().flush().unwrap();

        //// user input
        let _ = io::stdin()
            .read_line(&mut anime_name)
            .expect("Failed to read line");

        anime_name = replace_characters_to_files(&anime_name);
        //// delete newlines
        let len = anime_name.len();
        anime_name.truncate(len - 1);


        // Set default path
        let default_path    = set_default_path();

        //// redirect the input for a file
        //// anime path
        // Create anime name
        let path_anime_name = replace_characters_to_files(&anime_name.clone());
        let anime_path = {default_path}.to_owned()+{&path_anime_name};
        // try to find the anime folder
        anime_file_exists(anime_path.clone());

        // Set anime waifu
        print!("What is the anime waifu UwU?:");
        io::stdout().flush().unwrap();

        //// user input
        let _ = io::stdin()
            .read_line(&mut anime_waifu)
            .expect("Failed to read line");

        //// delete newlines
        let len = anime_waifu.len();
        anime_waifu.truncate(len - 1);


        // Set anime file
        let anime_waifu = "waifu: ".to_owned()+{&anime_waifu};
        let anime_file  = {anime_path}.to_owned()+"/anime.conf";

        //// redirect the input for a file
        write_in_file(anime_file,&anime_waifu);
}
// Add an anime waifu end





// Add an anime waifu from no interactive questions
fn add_waifu_nointeractive() {
// Set args
    let args: Vec<String> = env::args().collect();


    // anime name
    if &args[3] == "--anime" {
        let anime_name      = &args[4];
        let anime_name      = replace_characters_to_files(&anime_name.clone());
        // Set default path
        let default_path    = set_default_path();

        //// redirect the input for a file
        //// anime path
        // Create anime name
        let path_anime_name = replace_characters_to_files(&anime_name.clone());
        let anime_path      = {default_path}.to_owned()+{&path_anime_name};
        // try to find the anime folder
        anime_file_exists(anime_path.clone());

        // Set anime waifu
        let anime_waifu = &args[2];
        // Set anime file
        let anime_waifu = "waifu: ".to_owned()+{&anime_waifu};
        let anime_file  = {anime_path}.to_owned()+"/anime.conf";

        //// redirect the input for a file
        write_in_file(anime_file,&anime_waifu);

    } else {
        println!("Argument: '{}' not found", &args[3]);
        std::process::exit(1);
    }
}

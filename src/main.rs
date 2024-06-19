use std::{
    fs,
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};

use colored::Colorize;
use dialoguer::FuzzySelect;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use serde::Deserialize;

#[derive(Deserialize)]
struct Profiles {
    profiles: Vec<Profile>,
}

#[derive(Deserialize)]
struct Profile {
    profile_name: String,
    paths: Vec<String>,
}

fn main() {
    let config_text = fs::read_to_string("config.toml")
        .expect("config.toml should have been created by the user");
    let profiles: Profiles = toml::from_str(&config_text)
        .expect("should have been able to parse profiles from config.toml");
    let profile_names: Vec<&str> = profiles
        .profiles
        .par_iter()
        .map(|p| p.profile_name.as_str())
        .collect();

    let selection = FuzzySelect::new()
        .with_prompt("Rusty-starter profile")
        .items(&profile_names)
        .interact()
        .expect("should have been able to interact with fuzzy selection");

    profiles.profiles[selection]
        .paths
        .par_iter()
        .for_each(|path| {
            if let Err(e) = Command::new("cmd.exe")
                .args(["/c", &path.to_string()])
                .spawn()
            {
                eprintln!("{} {}: {}", "[-]".red(), path, e);
            } else {
                println!("{} {}", "[+]".green(), path);
            }
        });

    println!("Auto closing in 5s..");
    sleep(Duration::from_secs(5));
    exit(0);
}

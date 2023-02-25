use directories::UserDirs;
use std::{
    fs::{self, OpenOptions},
    io::{stdin, stdout, BufRead, Write},
    process::Command,
};

#[derive(Debug)]
struct Profile {
    name: String,
    email: String,
}

fn main() {
    let config_path = get_config_path().unwrap();
    let profiles = read_profiles_from_file(config_path.as_str());
    let mut input = String::new();

    if let Some(profile) = get_current_profile() {
        println!(
            "User '{}:{}' already added to this repo.",
            profile.name, profile.email
        );
        print!("Configure new user? [Y/n] ");
        stdout().flush().unwrap();
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();

        if !is_input_positive(input.as_str()) {
            return;
        }
    } else {
        println!("No user configured for this repo")
    }

    println!("\nWhich profile to add? (from {config_path})");
    for (i, profile) in profiles.iter().enumerate() {
        println!("{}. {} - {}", i + 1, profile.name, profile.email)
    }

    if profiles.len() == 0 {
        println!("No profiles found in {config_path}");
    }

    println!("{}. Add a new profile", profiles.len() + 1);

    print!("\nOption: ");
    stdout().flush().unwrap();

    input.clear();
    stdin().lock().read_line(&mut input).unwrap();

    let choise: usize = input.trim().parse().unwrap();

    if choise < 1 || choise > profiles.len() + 1 {
        println!("Invalid choise: {choise}");
        return;
    }

    if choise == profiles.len() + 1 {
        print!("Name: ");
        stdout().flush().unwrap();
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();
        let name = input.trim().to_string();

        print!("Email: ");
        stdout().flush().unwrap();
        input.clear();
        stdin().lock().read_line(&mut input).unwrap();
        let email = input.trim().to_string();

        let profile = Profile {
            name: name,
            email: email,
        };

        add_profile_to_config(&profile, &config_path).unwrap();
    } else {
        let profile = &profiles[choise - 1];
        config_git_user(profile.name.as_str(), profile.email.as_str());

        println!(
            "\nUser '{}:{}' successfully configured",
            profile.name, profile.email
        );
    }
}

fn add_profile_to_config(profile: &Profile, file_path: &str) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(file_path)?;
    let mut should_add_newline = false;

    if contents.len() > 0 && contents.as_str().chars().last().unwrap() != '\n' {
        should_add_newline = true;
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_path)?;

    if should_add_newline {
        write!(file, "\n")?;
    }

    writeln!(file, "{}", string_record_from(profile))?;
    return Ok(());
}

fn string_record_from(profile: &Profile) -> String {
    return format!("{}:{}", profile.name, profile.email);
}

fn is_input_positive(input: &str) -> bool {
    if input.len() == 0 {
        return false;
    }

    let input = input.to_lowercase();
    let input = input.trim();

    if input == "y" || input.starts_with("yes") {
        return true;
    }

    return false;
}

// TODO: check if exists
fn read_profiles_from_file(file_path: &str) -> Vec<Profile> {
    let contents = fs::read_to_string(file_path)
        .expect(format!("Couldn't read the file {}", file_path).as_str());

    let mut profiles: Vec<Profile> = Vec::new();

    for line in contents.split("\n") {
        let split: Vec<&str> = line.split(":").collect();
        if split.len() == 2 {
            profiles.push(Profile {
                name: split[0].trim().to_string(),
                email: split[1].trim().to_string(),
            });
        }
    }

    return profiles;
}

fn get_config_path() -> Option<String> {
    let user_dirs = UserDirs::new();

    return match user_dirs {
        None => None,
        Some(user_dirs) => {
            let home = user_dirs.home_dir();
            let config_path = home.join(".git-user.txt");
            return match config_path.to_str() {
                None => None,
                Some(path) => Some(String::from(path)),
            };
        }
    };
}

fn config_git_user(name: &str, email: &str) {
    execute(vec!["git", "config", "user.name", name]);
    execute(vec!["git", "config", "user.email", email]);
}

fn get_current_profile() -> Option<Profile> {
    let name = execute(vec!["git", "config", "user.name"])
        .trim()
        .to_string();

    let email = execute(vec!["git", "config", "user.email"])
        .trim()
        .to_string();

    if name.len() == 0 || email.len() == 0 {
        return None;
    } else {
        return Some(Profile {
            name: name,
            email: email,
        });
    }
}

// TODO: error handling
fn execute(command: Vec<&str>) -> String {
    if command.len() == 0 {
        return String::from("");
    }

    let mut cmd = Command::new(command[0]);
    cmd.args(&command[1..]);

    return match cmd.output() {
        Ok(output) => {
            let s = String::from_utf8_lossy(&output.stdout);
            return s.to_string();
        }
        Err(_) => String::from(""),
    };
}

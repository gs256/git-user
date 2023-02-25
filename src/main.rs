use directories::UserDirs;
use std::{
    fs::{self, File, OpenOptions},
    io::{stdin, stdout, BufRead, Write},
    process::Command,
};

#[derive(Debug)]
struct Profile {
    name: String,
    email: String,
}

fn main() {
    let mut console_input = String::new();

    if let Some(profile) = get_current_profile() {
        println!("User '{}' already added to this repo.", to_string(&profile));
        print("Change user? [Y/n] ");
        let choise = input(&mut console_input);

        if !is_choise_positive(&choise) {
            return;
        }
    } else {
        println!("No user configured for this repo");
    }

    offer_to_configure_profile()
}

fn offer_to_configure_profile() {
    let config_path = get_config_path();

    loop {
        let profiles = read_profiles_from_file(&config_path);

        if !dispatch_options(&profiles) {
            break;
        }
    }
}

fn dispatch_options(profiles: &[Profile]) -> bool {
    let config_path = get_config_path();
    let mut input_buffer = String::new();

    if profiles.len() == 0 {
        println!("No profiles found in {config_path}");
        print("Add a new profile? [Y/n] ");
        let choise = input(&mut input_buffer);

        if is_choise_positive(&choise) {
            create_profile();
            return true;
        }

        return false;
    }

    println!("\nWhich profile to use in this repo? (from {config_path})");

    for (i, profile) in profiles.iter().enumerate() {
        println!("{}. Profile '{}:{}'", i + 1, profile.name, profile.email)
    }

    println!("{}. Add a new profile", profiles.len() + 1);
    print("\nOption: ");

    let choise_raw = input(&mut input_buffer);
    let choise = choise_raw.trim().parse::<usize>();

    if choise.is_err() {
        println!("Invalid number: {choise_raw}");
        return true;
    }

    let choise = choise.unwrap();
    let add_profile_choise = profiles.len() + 1;
    let max_choise_number = add_profile_choise;

    if choise < 1 || choise > max_choise_number {
        println!("Invalid choise: {choise}");
        return true;
    }

    if choise == add_profile_choise {
        create_profile();
        return true;
    } else {
        let profile = &profiles[choise - 1];
        config_git_user(profile.name.as_str(), profile.email.as_str());
        println!(
            "\nUser '{}:{}' successfully configured",
            profile.name, profile.email
        );
        return false;
    }
}

fn create_profile() {
    let mut input_buffer = String::new();

    print("Name: ");
    let name = input(&mut input_buffer).trim().to_string();

    print("Email: ");
    let email = input(&mut input_buffer).trim().to_string();

    let profile = Profile {
        name: name,
        email: email,
    };

    let config_path = get_config_path();
    add_profile_to_config(&profile, &config_path)
        .expect(&format!("Couldn't add profile to {}", config_path));
}

fn print(string: &str) {
    print!("{}", string);
    stdout().flush().unwrap();
}

fn input(buffer: &mut String) -> String {
    buffer.clear();
    stdin().lock().read_line(buffer).unwrap();
    return buffer.clone();
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

    writeln!(file, "{}", to_string(profile))?;
    return Ok(());
}

fn to_string(profile: &Profile) -> String {
    return format!("{}:{}", profile.name, profile.email);
}

fn is_choise_positive(input: &str) -> bool {
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

fn read_profiles_from_file(file_path: &str) -> Vec<Profile> {
    let contents = fs::read_to_string(file_path).unwrap_or_else(|_| {
        File::create(file_path).expect(&format!("Couldn't create {}", file_path));
        return String::new();
    });

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

fn get_config_path() -> String {
    let config_name = ".git-user.txt";

    let user_dirs = UserDirs::new().expect("Couldn't get home directory path");
    let home_dir = user_dirs.home_dir();
    return String::from(
        home_dir
            .join(config_name)
            .to_str()
            .expect("The home path probably contains some weird characters"),
    );
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

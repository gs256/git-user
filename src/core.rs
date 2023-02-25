use directories::UserDirs;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    process::Command,
};

#[derive(Debug)]
pub struct Profile {
    pub name: String,
    pub email: String,
}

impl Profile {
    pub fn to_string(self: &Self) -> String {
        return format!("{}:{}", self.name, self.email);
    }
}

pub fn execute(command: Vec<&str>) -> String {
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

pub fn get_config_path() -> String {
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

pub fn config_git_user(name: &str, email: &str) {
    execute(vec!["git", "config", "user.name", name]);
    execute(vec!["git", "config", "user.email", email]);
}

pub fn add_profile_to_config(profile: &Profile, file_path: &str) -> Result<(), std::io::Error> {
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

    writeln!(file, "{}", profile.to_string())?;
    return Ok(());
}

pub fn read_profiles_from_file(file_path: &str) -> Vec<Profile> {
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

pub fn get_current_profile() -> Option<Profile> {
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

use std::{
    io::{stdin, stdout, BufRead, Write},
    process::exit,
};

mod core;
mod tests;
use crate::core::Profile;

fn main() {
    setup_keyboard_interrupt_handler();

    if let Some(profile) = core::get_current_profile() {
        println!("User '{}' already added to this repo.", profile.to_string());
        print("Change user? [Y/n] ");
        let choise = input();

        if !is_choise_positive(&choise) {
            return;
        }
    } else {
        println!("No user configured for this repo");
    }

    offer_to_configure_profile()
}

fn setup_keyboard_interrupt_handler() {
    ctrlc::set_handler(move || {
        exit(0);
    })
    .expect("Error setting Ctrl-C handler");
}

fn offer_to_configure_profile() {
    let config_path = core::get_config_path();

    loop {
        let profiles = core::read_profiles_from_file(&config_path);

        if !dispatch_options(&profiles) {
            break;
        }
    }
}

fn dispatch_options(profiles: &[Profile]) -> bool {
    let config_path = core::get_config_path();

    if profiles.len() == 0 {
        println!("No profiles found in {config_path}");
        print("Add a new profile? [Y/n] ");
        let choise = input();

        if is_choise_positive(&choise) {
            create_profile();
            return true;
        }

        return false;
    }

    println!("\nWhich profile to use in this repo? (from {config_path})");

    for (i, profile) in profiles.iter().enumerate() {
        println!(
            "{}. Profile '{}:{}' {}",
            i + 1,
            profile.name,
            profile.email,
            profile.description
        )
    }

    println!("{}. Add a new profile", profiles.len() + 1);
    print("\nOption: ");

    let choise_raw = input();
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
        core::config_git_user(profile.name.as_str(), profile.email.as_str());
        println!(
            "\nUser '{}:{}' successfully configured",
            profile.name, profile.email
        );
        return false;
    }
}

fn create_profile() {
    print("Name: ");
    let name = input().trim().to_string();

    print("Email: ");
    let email = input().trim().to_string();

    let profile = Profile {
        name: name,
        email: email,
        description: String::new(),
    };

    let config_path = core::get_config_path();
    core::add_profile_to_config(&profile, &config_path)
        .expect(&format!("Couldn't add profile to {}", config_path));
}

fn print(string: &str) {
    print!("{}", string);
    stdout().flush().unwrap();
}

fn input() -> String {
    let mut buffer = String::new();

    stdin()
        .lock()
        .read_line(&mut buffer)
        .expect("Error reading from stdin");

    return buffer;
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

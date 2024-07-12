mod core;
mod profile;

use crate::profile::Profile;
use std::{
    io::{stdin, stdout, BufRead, Write},
    process::exit,
};

fn main() {
    setup_keyboard_interrupt_handler();

    println!();
    println!("┌──────────┐");
    println!("│ GIT-USER │");
    println!("└──────────┘");
    println!();

    if let Some(profile) = core::get_current_profile() {
        println!(
            "User {} ({}) is already added to this repo",
            profile.name, profile.email
        );
        print("-> Change user? [Y/n] ");
        let choise = input();

        if !is_choise_positive(&choise) {
            return;
        }

        println!();
    }

    offer_to_configure_profile();
    println!();
}

fn setup_keyboard_interrupt_handler() {
    ctrlc::set_handler(move || {
        println!();
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

    if profiles.is_empty() {
        println!("No profiles found in {config_path}");
        print("-> Add a new profile? [Y/n] ");
        let choise = input();

        if is_choise_positive(&choise) {
            create_profile();
            println!();
            return true;
        }

        return false;
    }

    println!("Pick profile for this repo");

    for (i, profile) in profiles.iter().enumerate() {
        if profile.description.is_empty() {
            println!("  [{}] {} ({})", i + 1, profile.name, profile.email,)
        } else {
            println!(
                "  [{}] {} ({}) -> {}",
                i + 1,
                profile.name,
                profile.email,
                profile.description
            )
        }
    }

    println!("  [a] Add a new profile");
    println!("  [q] Quit");
    println!();
    print("-> Option: ");

    let input = input();
    let choise_raw = input.trim();

    if choise_raw.to_lowercase() == "a" {
        create_profile();
        return true;
    } else if choise_raw.to_lowercase() == "q" {
        return false;
    }

    let choise_num = choise_raw.parse::<usize>();

    if choise_num.is_err() {
        println!("Invalid input '{choise_raw}'");
        println!();
        return true;
    }

    let choise_num = choise_num.unwrap();
    let max_choise_number = profiles.len() + 1;

    if choise_num < 1 || choise_num > max_choise_number {
        println!("Invalid choise: {choise_num}");
        return true;
    }

    let profile = &profiles[choise_num - 1];
    core::config_git_user(profile.name.as_str(), profile.email.as_str());
    println!();
    println!(
        "User {} ({}) has been selected for this repo",
        profile.name, profile.email
    );
    false
}

fn create_profile() {
    print("-> Name: ");
    let name = input().trim().to_string();

    print("-> Email: ");
    let email = input().trim().to_string();

    print("-> Description: ");
    let description = input().trim().to_string();

    let profile = Profile {
        name,
        email,
        description,
    };
    println!();

    let config_path = core::get_config_path();
    core::add_profile_to_config(&profile, &config_path)
        .unwrap_or_else(|_| panic!("[ERROR] Couldn't add profile to {}", config_path));
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

    buffer
}

fn is_choise_positive(input: &str) -> bool {
    if input.is_empty() {
        return false;
    }

    let input = input.to_lowercase();
    let input = input.trim();

    if input == "y" || input.starts_with("yes") {
        return true;
    }

    false
}

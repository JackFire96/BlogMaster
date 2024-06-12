use bcrypt::{hash, verify, DEFAULT_COST};
use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};

struct User {
    username: String,
    password_hash: String,
}

fn main() {
    let mut users = HashMap::new();

    loop {
        println!("\n~~~~~~~~~~~~");
        println!("{}", "1. Inscription".cyan());
        println!("{}", "2. Connexion".green());
        println!("{}", "3. Quitter".yellow());
        println!("~~~~~~~~~~~~");

        let choice = get_input("... : ");
        match choice.trim() {
            "1" => register(&mut users),
            "2" => login(&users),
            "3" => break,
            _ => println!("{}", "Option invalide".red()),
        }
    }
}

fn register(users: &mut HashMap<String, String>) {
    let username = get_input("\nNom d'utilisateur : ");
    if users.contains_key(&username) {
      println!("{}", "L'utilisateur est déjà inscrit !".red());
      return;
    }
    let password = get_input("Mot de passe : ");
    let password_hash = hash(password, DEFAULT_COST).expect("Error hashing password");

    users.insert(username, password_hash);
    println!("{}", "Inscription réussie !".green());
}

fn login(users: &HashMap<String, String>) {
    let username = get_input("\nNom d'utilisateur : ");
    let password = get_input("Mot de passe : ");

    match users.get(&username) {
        Some(stored_hash) => {
            if verify(password, stored_hash).expect("Error verifying password") {
                println!("{}", "Connexion réussie !".green());
            } else {
                println!("{}", "Le nom d'utilisateur ou le mot de passe est invalide !".red());
            }
        }
        None => println!("{}", "Le nom d'utilisateur ou le mot de passe est invalide !".red()),
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Error flushing stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input.trim().to_string()
}
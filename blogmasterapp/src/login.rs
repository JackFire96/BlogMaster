use bcrypt::{hash, verify, DEFAULT_COST};
use colored::*;
use std::collections::HashMap;
use std::io::{self, Write};

pub struct User {
  pub username: String,
  pub password_hash: String,
}

pub struct Session {
  pub current_user: Option<String>,
}

pub fn register(users: &mut HashMap<String, String>) {
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

pub fn login(users: &HashMap<String, String>, session: &mut Session) {
  let username = get_input("\nNom d'utilisateur : ");
  let password = get_input("Mot de passe : ");

  match users.get(&username) {
      Some(stored_hash) => {
          if verify(password, stored_hash).expect("Error verifying password") {
              println!("{}", "Connexion réussie !".green());
              session.current_user = Some(username);
          } else {
              println!("{}", "Le nom d'utilisateur ou le mot de passe est invalide !".red());
          }
      }
      None => println!("{}", "Le nom d'utilisateur ou le mot de passe est invalide !".red()),
  }
}

pub fn logout(session: &mut Session) {
  session.current_user = None;
  println!("{}", "Déconnexion réussie !".yellow());
}

pub fn get_input(prompt: &str) -> String {
  print!("{}", prompt);
  io::stdout().flush().expect("Error flushing stdout");

  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Error reading input");
  input.trim().to_string()
}
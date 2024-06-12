mod login;
mod blog;

use std::collections::HashMap;

use colored::Colorize;
use login::get_input;
use mysql::Pool;

fn main() {

  //db connect
  let url = "mysql://root:root@localhost:8888/blog_master";
  let pool = Pool::new(url).unwrap();
  let mut conn = pool.get_conn().unwrap();
  let mut users = HashMap::new();
  let mut session = login::Session { current_user: None };

    loop {
        println!("\n~~~~~~~~~~~~");
        println!("{}", "0. Quitter".yellow());
        if session.current_user.is_none() {
          println!("{}", "1. Inscription".cyan());
          println!("{}", "2. Connexion".green());
        }
        if session.current_user.is_some() {
          println!("3. {}", "Créer un blog simple".blue());
          println!("4. {}", "Logout".blue());
      }
        println!("~~~~~~~~~~~~");

        let choice = get_input("... : ");
        match choice.trim() {
            "0" => break,
            "1" => {
              if session.current_user.is_none() {
                login::register(&mut users);
              }
              else {
                println!("{}", "Vous êtes déjà inscrit !".red());
              }
            }
            "2" => {
              if session.current_user.is_none() {
                login::login(&users, &mut session);
              }
              else {
                println!("{}", "Vous êtes déjà connecté !".red());
              }
            }
            "3" => {
              if session.current_user.is_some() {
                blog::createBlog();
              }
              else {
                println!("{}", "Veuillez vous connecter avant de continuer !".red());
              }
            }
            "4" => {
              if session.current_user.is_some() {
                login::logout(&mut session);
              }
              else {
                println!("{}", "Veuillez vous connecter avant de continuer !".red());
              }
            }
            _ => println!("{}", "Option invalide".red()),
        }
    }
}
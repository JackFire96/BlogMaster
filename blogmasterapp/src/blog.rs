use colored::*;
use super::login::Session;

pub struct Blog {
  pub name: String,
  pub description: String,
  //pub options > liste des options du blog
}

pub fn createBlog() {
  let name = super::login::get_input("Nom du blog : ");
  let description = super::login::get_input("Description du blog : ");

  let blog = Blog {
    name: name.clone(),
    description,
  };

  //Push dans la BDD
  println!("{}", format!("Le blog ['{}'] est créé !", name).green());
}

pub fn listBlog() {
  //Methode pour récupérer les blogs de la BDD
}

pub fn addOptions() {
  
}
use colored::*;

pub struct Channel {
  pub blog: String,
  pub name: String,
  pub description: String,
  //pub messages > liste des messages du channel
}

pub fn createChannel() {
  let blog: String = super::login::get_input("Nom du blog : ");
  let name = super::login::get_input("Nom du canal : ");
  let description = super::login::get_input("Description du canal : ");

  let channel = Channel {
    blog,
    name: name.clone(),
    description,
  };

   //Push dans la BDD
   println!("{}", format!("Le canal ['{}'] est créé !", name).green());
}
use colored::*;

pub struct Channel {
  pub id: i64,
  pub blog: String,
  pub name: String,
  pub description: String,
}

pub fn createChannel() {
  let id: i64 = 0;
  let blog: String = super::login::get_input("Nom du blog : ");
  let name = super::login::get_input("Nom du canal : ");
  let description = super::login::get_input("Description du canal : ");

  let channel = Channel {
    id,
    blog,
    name: name.clone(),
    description,
  };

   //Push dans la BDD
   println!("{}", format!("Le canal ['{}'] est créé !", name).green());
}
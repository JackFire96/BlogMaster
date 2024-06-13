use colored::*;
use mysql::{params, prelude::Queryable, Pool};
use super::login::Session;

pub struct Blog {
  pub id: Option<i64>,
  pub nom: String,
  pub description: String,
  pub galerie: i32,
}

pub fn createBlog(pool: &Pool) {
  let id: i64 = 0;
  let nom = super::login::get_input("Nom du blog : ");
  let description = super::login::get_input("Description du blog : ");
  let galerie: i32 = 0;

  let blog = Blog {
    id: Some(id),
    nom: nom.clone(),
    description: description,
    galerie: galerie,
  };

  //Push dans la BDD
  insert_blog(pool, blog);
  println!("{}", format!("Le blog ['{}'] est créé !", nom).green());
}

pub fn listBlog() {
  //Methode pour récupérer les blogs de la BDD
}

pub fn addOptions() {

}

fn insert_blog(pool: &Pool, blog: Blog) -> std::result::Result<(), Box<dyn std::error::Error>> {
  // Obtenez une connexion
  let mut conn = pool.get_conn()?;
  
  // Préparer et exécuter la requête d'insertion
  conn.exec_drop(
      r"INSERT INTO blog (nom, description, galerie) VALUES (:nom, :description, :galerie)",
      params! {
          "nom" => blog.nom,
          "description" => blog.description,
          "galerie" => blog.galerie,
      },
  )?;

  // Récupérer l'id généré automatiquement (si besoin)
  let id = conn.last_insert_id();
  println!("Inserted blog with ID: {}", id);

  Ok(())
}
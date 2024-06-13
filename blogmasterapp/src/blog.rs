use colored::*;
use mysql::{params, prelude::Queryable, Pool};
use crate::{article::{createArticle, list_article}, login::get_input};

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

pub fn list_blog(pool: &Pool, session: &Session) {
  let blogs = get_blogs(pool).expect("Erreur lors de la récupération des blogs");

  loop {
    println!("~~~~~~~~~~~~");
    println!("{}", "Séléctionnez un blog :".cyan());
    for blog in &blogs {
      println!(
        "{}. {}",
        blog.id.unwrap_or_default(),
        blog.nom,
      );
    }
    println!("{}", "x. Quitter".yellow());
    println!("~~~~~~~~~~~~");
    let choice = get_input("... : ");
    match choice.trim() {
      "x" => break,
      input => {
        match input.parse::<i64>() {
            Ok(blog_id) => {
                if id_exists(&pool, blog_id) {
                    // print blog
                    menu_articles(pool, blog_id, session);
                } else {
                    println!("{}", "Aucun blog trouvé avec ce numéro".red());
                }
            },
            Err(_) => {
                println!("{}", "Option invalide".red());
            }
        }
      }
    }
  }
}

fn get_blogs(pool: &Pool) -> std::result::Result<Vec<Blog>, Box<dyn std::error::Error>> {
  // Obtenez une connexion
  let mut conn = pool.get_conn()?;

  // Exécuter la requête de sélection
  let blogs = conn.query_map(
      "SELECT id, nom, description, galerie FROM blog",
      |(id, nom, description, galerie)| Blog {
          id: Some(id),
          nom,
          description,
          galerie,
      },
  )?;

  Ok(blogs)
}

pub fn id_exists(pool: &Pool, id: i64) -> bool {
  match get_blogs(pool) {
      Ok(blogs) => blogs.iter().any(|blog| blog.id == Some(id)),
      Err(_) => false,
  }
}

pub fn menu_articles(pool: &Pool, blogid: i64, session: &Session) {
  let blogs = get_blogs(pool).expect("Erreur lors de la récupération des blogs");

  loop {
    println!("~~~~~~~~~~~~");
    println!("{}", "1. Créer un article :".cyan());
    println!("{}", "2. Voir un article :".cyan());
    println!("{}", "x. Quitter".yellow());
    println!("~~~~~~~~~~~~");

    let choice = get_input("... : ");
        match choice.trim() {
            "x" => break,
            "1" => {
              createArticle(pool, blogid);
            }
            "2" => {
              list_article(pool, session, blogid);
            }
            _ => println!("{}", "Option invalide".red()),
        }
  }

}
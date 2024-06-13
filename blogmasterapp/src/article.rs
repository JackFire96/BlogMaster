use colored::*;
use mysql::{params, prelude::Queryable, Pool};

use crate::{blog, login::{get_input, Session}, message::{list_message, sendMessage}};

pub struct Article {
  pub id: Option<i64>,
  pub titre: String,
  pub contenu: String,
  pub blog_id: i64,
}

pub fn createArticle(pool: &Pool, blogid: i64) {
  let id: i64 = 0;
  let titre = super::login::get_input("Nom de l'article : ");
  let contenu = super::login::get_input("Contenu de l'article : ");

  let article = Article {
    id: Some(id),
    titre: titre.clone(),
    contenu: contenu,
    blog_id: blogid,
  };

   insert_article(pool, article);
   println!("{}", format!("L'article ['{}'] est créé !", titre).green());
}

fn insert_article(pool: &Pool, article: Article) -> std::result::Result<(), Box<dyn std::error::Error>> {
  // Obtenez une connexion
  let mut conn = pool.get_conn()?;
  
  // Préparer et exécuter la requête d'insertion
  conn.exec_drop(
      r"INSERT INTO article (titre, contenu, blog_id) VALUES (:titre, :contenu, :blog_id)",
      params! {
          "titre" => article.titre,
          "contenu" => article.contenu,
          "blog_id" => article.blog_id,
      },
  )?;

  // Récupérer l'id généré automatiquement (si besoin)
  let id = conn.last_insert_id();
  println!("Inserted blog with ID: {}", id);

  Ok(())
}

pub fn list_article(pool: &Pool, session: &Session, blogid: i64) {
  let articles = get_articles(pool, blogid).expect("Erreur lors de la récupération des articles");

  loop {
    println!("~~~~~~~~~~~~");
    println!("{}", "Séléctionnez un article :".cyan());
    for article in &articles {
      println!(
        "{}. {} : {}",
        article.id.unwrap_or_default(),
        article.titre,
        article.contenu,
      );
    }
    println!("{}", "x. Quitter".yellow());
    println!("~~~~~~~~~~~~");
    let choice = get_input("... : ");
    match choice.trim() {
      "x" => break,
      input => {
        match input.parse::<i64>() {
            Ok(article_id) => {
                if id_exists(&pool, article_id, blogid) {
                    // print blog
                    menu_messages(pool, article_id, session, blogid);
                } else {
                    println!("{}", "Aucun article trouvé avec ce numéro".red());
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

fn get_articles(pool: &Pool, blogid: i64) -> std::result::Result<Vec<Article>, Box<dyn std::error::Error>> {
  // Obtenez une connexion
  let mut conn = pool.get_conn()?;

  // Exécuter la requête de sélection
  let articles = conn.exec_map(
      "SELECT id, titre, contenu, blog_id FROM article WHERE blog_id = :blogid",
      params! {
        "blogid" => blogid,
    },
      |(id, titre, contenu, blog_id)| Article {
          id: Some(id),
          titre,
          contenu,
          blog_id,
      },
  )?;

  Ok(articles)
}

pub fn id_exists(pool: &Pool, id: i64, blogid: i64) -> bool {
  match get_articles(pool, blogid) {
      Ok(articles) => articles.iter().any(|article| article.id == Some(id)),
      Err(_) => false,
  }
}

pub fn menu_messages(pool: &Pool, articleid: i64, mut session: &Session, blogid: i64) {
  let articles = get_articles(pool, blogid).expect("Erreur lors de la récupération des articles");

  loop {
    println!("~~~~~~~~~~~~");
    println!("{}", "1. Envoyer un message :".cyan());
    println!("{}", "2. Voir les messages :".cyan());
    println!("{}", "x. Quitter".yellow());
    println!("~~~~~~~~~~~~");

    let choice = get_input("... : ");
        match choice.trim() {
            "x" => break,
            "1" => {
              sendMessage(pool, articleid, session);
            }
            "2" => {
              list_message(pool, articleid);
            }
            _ => println!("{}", "Option invalide".red()),
        }
  }

}
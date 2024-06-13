use chrono::{DateTime, Local};
use colored::*;
use mysql::{params, prelude::Queryable, Pool};

use crate::{article::Article, login::Session, message};

pub struct Message {
  pub id: Option<i64>,
  pub username: String,
  pub article_id: i64,
  pub time: String,
  pub texte: String,
}

pub fn sendMessage(pool: &Pool, articleid: i64, session: & Session) {
  let id: i64 = 0;
  let text = super::login::get_input("Votre message : ");

  if let Some(ref username) = session.current_user {
    let message = Message {
      id: Some(id),
      username: username.clone(),
      article_id: articleid,
      time: Local::now().to_string(),
      texte: text,
    };
    insert_message(pool, message);
  }
}

fn insert_message(pool: &Pool, message: Message) -> std::result::Result<(), Box<dyn std::error::Error>> {
  // Obtenez une connexion
  let mut conn = pool.get_conn()?;
  
  // Préparer et exécuter la requête d'insertion
  conn.exec_drop(
      r"INSERT INTO message (username, article_id, time, texte) VALUES (:username, :article_id, :time, :texte)",
      params! {
          "username" => message.username,
          "article_id" => message.article_id,
          "time" => message.time,
          "texte" => message.texte,
      },
  )?;

  // Récupérer l'id généré automatiquement (si besoin)
  let id = conn.last_insert_id();
  println!("Inserted blog with ID: {}", id);

  Ok(())
}

pub fn list_message(pool: &Pool, articleid: i64) {
  let messages = get_messages(pool, articleid).expect("Erreur lors de la récupération des messages");
  println!("~~~~~~~~~~~~");
  for message in &messages {
      println!(
      "{} [{}]: {}",
      message.username,
      message.time,
      message.texte,
    );
  }
  println!("{}", "x. Quitter".yellow());
  println!("~~~~~~~~~~~~");
}

fn get_messages(pool: &Pool, articleid: i64) -> std::result::Result<Vec<Message>, Box<dyn std::error::Error>> {
  // Obtenez une connexion
  let mut conn = pool.get_conn()?;

  // Exécuter la requête de sélection
  let articles = conn.exec_map(
      "SELECT id, username, article_id, time, texte FROM message WHERE article_id = :articleid",
      params! {
        "articleid" => articleid,
    },
    |(id, username, article_id, time, texte): (i64, String, i64, String, String)| { 
        Message {
          id: Some(id),
          username,
          article_id,
          time,
          texte,
      }
    },
  )?;

  Ok(articles)
}
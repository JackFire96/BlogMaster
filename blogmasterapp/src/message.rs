use chrono::{DateTime, Local};
use colored::*;

use crate::{channel::Channel, login::Session};

pub struct Message {
  pub channel: String,
  pub username: String,
  pub time: DateTime<Local>,
  pub text: String,
}

pub fn createMessage(channel: &Channel, session: &mut Session) {
  let text = super::login::get_input("Votre message : ");

  let message = Message {
    channel: channel.name.clone(),
    username: session.current_user.clone().unwrap(),
    time: Local::now(),
    text: text.clone(),
  };

   //Push dans la BDD
   println!("{}", format!("Le message ['{}', '{}', '{}'] est créé !", message.username, message.time, text).green());
}
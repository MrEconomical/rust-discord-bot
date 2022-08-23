// Imports

use crate::commands::CommandResult;
use serenity::client::Context;
use serenity::model::channel::Message;

// Ping command

pub async fn main(_: &[String], _: &Context, _: &Message) -> Result<CommandResult, String> {
    Ok(CommandResult {
        content: Some(String::from("pong!")),
        embed: None
    })
}
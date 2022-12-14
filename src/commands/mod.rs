// Imports

mod ping;

use crate::config::PREFIX;
use std::error::Error;
use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::channel::{ Message, MessageType };

// Command parameters

const BLOCK_DELIMITER: [&str; 4] = ["\"", "'", "```", "`"];

// Bot command parameters

struct Command {
    name: String,
    args: Vec<String>
}

pub struct CommandResult {
    content: Option<String>,
    embed: Option<CreateEmbed>
}

impl CommandResult {
    // Create empty command result

    fn empty() -> CommandResult {
        CommandResult {
            content: None,
            embed: None
        }
    }
}

// Handle message event

pub async fn handle_command(ctx: &Context, msg: &Message) -> Result<(), Box<dyn Error>> {
    // Check message details

    if !msg.content.starts_with(PREFIX) && msg.guild_id.is_some() { return Ok(()); }
    if msg.content.is_empty() { return Ok(()); }
    if msg.author.bot { return Ok(()); }
    if msg.kind != MessageType::Regular { return Ok(()); }
    
    if let Some(cmd) = parse_command(&msg.content) {
        // Execute command

        let result = match cmd.name.as_str() {
            "ping" => ping::main(&cmd.args, ctx, msg),
            _ => return Ok(())
        }.await;

        // Send command result message

        match result {
            Ok(result) => {
                msg.channel_id.send_message(&ctx.http, |m| {
                    if let Some(content) = result.content {
                        m.content(content);
                    }
                    if let Some(embed) = result.embed {
                        m.set_embed(embed);
                    }
                    m
                }).await?;
            },
            Err(error) => {
                msg.channel_id.say(&ctx.http, format!("Error: {error}")).await?;
            }
        }
    }

    Ok(())
}

// Parse command from message text

fn parse_command(content: &str) -> Option<Command> {
    // Read command name

    let mut data = if let Some(stripped) = content.strip_prefix(PREFIX) {
        stripped.chars()
    } else {
        content.chars()
    };
    let name = data
        .by_ref()
        .take_while(|char| !char.is_whitespace())
        .collect();

    // Parse arguments

    let data = data
        .skip_while(|char| char.is_whitespace())
        .enumerate()
        .collect::<Vec<_>>();
    let mut args = vec![];
    let mut index = 0;

    while index < data.len() {
        // Check for start delimiter block

        if let Some(del) = BLOCK_DELIMITER.iter().find(|&&del| {
            for (i, char) in del.chars().enumerate() {
                if data[index + i].1 != char {
                    return false;
                }
            }
            true
        }) {
            let len = del.len();

            // Check for end delimiter

            if let Some(end) = data
                .windows(len)
                .skip(index + len)
                .find(|chars| {
                    for (i, char) in del.chars().enumerate() {
                        if chars[i].1 != char {
                            return false;
                        }
                    }
                    true
                })
                .map(|chars| chars[0].0)
            {
                // Collect characters between delimiters

                let mut chars = data.iter().skip(index + len);
                args.push(
                    chars
                        .by_ref()
                        .take(end - index - len)
                        .map(|(_, char)| char)
                        .collect()
                );

                // Find next non-whitespace block start

                index = chars
                    .skip(len)
                    .find(|(_, char)| !char.is_whitespace())
                    .map(|&(index, _)| index)
                    .unwrap_or(data.len());
                continue;
            }
        }

        // Parse regular argument ending with whitespace

        let mut chars = data.iter().skip(index);
        args.push(
            chars
                .by_ref()
                .take_while(|(_, char)| !char.is_whitespace())
                .map(|(_, char)| char)
                .collect()
        );

        // Find next non-whitespace block start

        index = chars
            .find(|(_, char)| !char.is_whitespace())
            .map(|&(index, _)| index)
            .unwrap_or(data.len());
    }

    Some(Command { name, args })
}
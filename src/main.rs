use std::env;

use rand::Rng;
use serenity::{all::Message, async_trait, prelude::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let msg_lc = msg.content.to_ascii_lowercase();
        if msg_lc.contains("sport") || msg_lc.contains("maverick") {
            let mut messages = vec!["Ball", "Soccer", "Sport", "Goal", "Touchdown",
                "Athletics!", "Champion", "Compete", "Dart", "Dive", "Exercise",
                "Fitness", "Football", "Free throw", "Game", "Guard", "Home run",
                "Hole-in-one", "Halftime", "Jump", "Medal", "Loser"];
            if msg.author.id.get() == 302116717184679938 {
                messages.push("That's not right");
                messages.push("Correct");
            }

            let random_idx = rand::thread_rng().gen_range(0..messages.len());
            let message = messages.get(random_idx).unwrap().to_string();
            if let Err(why) = msg.channel_id.say(&ctx.http, message).await {
                println!("Failed to send message: {why:?}");
            }
        }

        let random_chance = rand::thread_rng().gen_range(0..15);
        if random_chance == 5 {
            if let Err(why) = msg.react(&ctx.http, '⚽').await {
                println!("Failed to send message: {why:?}");
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

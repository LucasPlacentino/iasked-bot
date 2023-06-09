//use std::env;

use anyhow::anyhow;
use rand::Rng;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
//use tracing::{error, info};
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::id::GuildId;

//struct Handler;
struct Bot{
    guild_id: String
}

const MESSAGES: [&str; 11] = [
    "Who asked?",
    "Who asked",
    "who asked?",
    "who asked",
    "Who asked?",
    "who asked",
    "Who asked",
    "whoasked",
    "whoasked?",
    "Whoasked",
    "Whoasked?",
];

const REPLIES: [&str; 3] = ["I asked", "I did", "Me"];

#[async_trait]
impl EventHandler for Bot {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_own(&ctx.cache) {
            return;
        }

        if MESSAGES.contains(&msg.content.as_str()) {
            let reply = REPLIES[rand::thread_rng().gen_range(0..REPLIES.len())];
            if let Err(why) = msg.reply(&ctx.http, reply).await {
                println!("> Error sending message: {:?}", why);
            }
            if let Err(why) = msg.react(&ctx.http, '🙋').await {
                println!("> Error reacting to message: {:?}", why);
            }
        }
        /*
        if msg.content == "Who asked?" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.reply(&ctx.http, "I asked").await {
                println!("> Error sending message: {:?}", why);
            }
        }
        */
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("> {} is connected!", ready.user.name);
    
        let guild_id = GuildId(self.guild_id.parse().unwrap());
        // Get the guild_id set in `Secrets.toml`

        let commands =
            GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| {
                        command.name("iasked").description("Say I asked")
                    })
            })
            .await
            .unwrap();

        println!("Registered commands: {:#?}", commands);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("> Received command interaction: {:#?}", command);

            let response_content = match command.data.name.as_str() {
                "iasked" => "I asked".to_owned(),
                command => unreachable!("> Unknown command: {}", command),
            };

            let create_interaction_response =
                command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_content))
                });

            if let Err(why) = create_interaction_response.await {
                println!("> Cannot respond to slash command: {}", why);
            }
        }
    }
}

/*
#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    /*
    let token = env::var("BOT_TOKEN").expect("> Expected a token in the environment");
    */
    let token = if let Some(token) = secret_store.get("BOT_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("> Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("> Client error: {:?}", why);
    }
*/

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("BOT_TOKEN") {
        token
    } else {
        return Err(anyhow!("> 'BOT_TOKEN' was not found").into());
    };
    // Get the guild_id set in `Secrets.toml`
    let guild_id = if let Some(guild_id) = secret_store.get("GUILD_ID") {
        guild_id
    } else {
        return Err(anyhow!("> 'GUILD_ID' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let bot = Bot{
        guild_id
    };

    let client = Client::builder(&token, intents)
        .event_handler(bot)
        .await
        .expect("> Err creating client");

    /*
    if let Err(why) = client.start().await {
        println!("> Client error: {:?}", why);
    }
    */
    Ok(client.into())
}

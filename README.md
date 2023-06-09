# "I Asked" Discord Bot 🤖
A Discord "Who asked?", "I did" bot.  

Make your Discord servers friendlier.  

## 🎾 Features
Replies "I asked", "I did" or "Me" to any message containing variations of "Who asked?"  

## Setup
- **⚠️ Must have the `Message Content` privileged content ⚠️**,  
  as well as `Send Message`, `Read Messages/View Channels`, `Read Message History` and `Add Reactions` permissions!  

  This can be enabled in your [Discord Developer Portal](https://discord.com/developers), in your app and bot's settings and when creating the bot invite link there.

- `BOT_TOKEN` must be put in a `Secrets.toml` file (like [`Secrets_template.toml`](/Secrets_template.toml)), from your bot's settings in the Discord Developer Portal.

- Install [Shuttle](https://docs.shuttle.rs/) and login through the CLI.

- Run `cargo shuttle project start --idle-minutes 0` in the base folder of this repo.  
- Then you can test locally using `cargo shuttle run`.  
- Finally you can deploy the bot to Shuttle using `cargo shuttle deploy`!

## *️⃣ Versions:
- [Python](https://github.com/LucasPlacentino/iasked-bot/tree/python) using [_Disnake_](https://github.com/DisnakeDev/disnake) running on Docker: _TODO_
- [Javascript](https://github.com/LucasPlacentino/iasked-bot/tree/javascript) using [_Discord.js_](https://github.com/discordjs/discord.js) running/hosted on [**Glitch**?](TODO): _TODO_
- [Rust](https://github.com/LucasPlacentino/iasked-bot/tree/rust) using [_Serenity_](https://github.com/serenity-rs/serenity) running/hosted on [**Shuttle**](https://github.com/shuttle-hq/shuttle) (here)

## 👤 Author
[Lucas Placentino](https://github.com/LucasPlacentino)

## 📜 License
This project is licensed under either of the following licenses, at your option:
- Apache License, Version 2.0 ([LICENSE-APACHE](/LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])  
- MIT license ([LICENSE_MIT](/LICENSE-MIT) or [http://opensource.org/licenses/MIT])  

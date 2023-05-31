const { Client, Intents } = require("discord.js");
//const client = new Client({
//  intents: [Intents.FLAGS.GUILDS, Intents.FLAGS.GUILD_MESSAGES]
//});
const client = new Client();

client.on("ready", () => {
  console.log(`> Logged in as ${client.user.tag}!`)
  console.log("> Bot started");
  return
});

client.on("message", (message) => {
  if (message.author.bot) {
    return
  }
  console.log("> Message received");
//  if (message.content === "Who asked?") {
  if (message.content.startsWith("Who asked?")) {
    message.reply("I asked").catch(console.error);
    console.log("> Replying");
    return
  }
});

client.login(process.env.BOT_TOKEN)


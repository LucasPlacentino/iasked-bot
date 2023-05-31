""" # ---------------    old    -------------------
import discord

Bot = commands.Bot(command_prefix="")

@Bot.event
async def on_message(message):
  if "not" in message.content:
      await Bot.send_message(message.channel, "yes")

# ------------------    OR    ---------------------
import discord

client = discord.Client()

@client.event
async def on_message(message):
    if "example" in message:
        message.channel.send("this is an example")

"""

import os
import disnake
from random import randint

replies = ["I asked", "I did", "Me"]
#texts = ["Who asked", "Who Asked", "who asked", "whoasked", "Whoasked"]
reduced_texts = ["whoasked", "whoaskedthough", "whoaskedtho"] # add possible typos ?

class MyClient(disnake.Client):
    async def on_message(self, message: disnake.Message):
        print(f"> Received message")
        
        
        # prevent the bot from replying to itself (even if it shouldn't happen)
        if message.author.id == self.user.id:
            return
        
        #if any(message.content.startswith(x) for x in texts):
        if any(x in ''.join(message.content.split()).lower() for x in reduced_texts): #reduce message content to no spaces and lowercase to check it more easily
        #for text in texts:
        #  if message.content.startswith(text):
          print(f"> Replying to message")
          #await message.reply(replies[randint(0,length(replies)], mention_author=True)
          await message.reply(replies[randint(0,length(replies)])
          return

    async def on_ready(self):
        print(f"> Logged in as {self.user} (ID: {self.user.id})\n------")
        return

intents = disnake.Intents.default()
intents.message_content = True

if __name__ == "__main__":
    client = MyClient(intents=intents)
    client.run(os.getenv("BOT_TOKEN"))
    print(f"> Starting bot")
    print(f"> Replies: ", end="")
    for reply in replies:
        print(reply + f", ")
    print(f"")


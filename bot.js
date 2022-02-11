const fs = require('fs');
const { Client, Collection, Intents } = require('discord.js');
const { Manager } = require("erela.js");
const { token } = require('./config.json');

const client = new Client({ intents: [Intents.FLAGS.GUILDS] });
client.commands = new Collection();


const nodes = [
	{
	  host: "localhost",
	  password: "youshallnotpass",
	  port: 2333,
	}
  ];
  
  // Assign Manager to the client variable
  client.manager = new Manager({
	// The nodes to connect to, optional if using default lavalink options
	nodes,
	// Method to send voice data to Discord
	send: (id, payload) => {
	  const guild = client.guilds.cache.get(id);
	  // NOTE: FOR ERIS YOU NEED JSON.stringify() THE PAYLOAD
	  if (guild) guild.shard.send(payload);
	}
  });

//Event Handler
const eventFiles = fs.readdirSync('./events').filter(file => file.endsWith('.js'));
console.log('-----Loading Events-----')
for (const file of eventFiles) {
	const event = require(`./events/${file}`);
	if (event.once) {
		client.once(event.name, (...args) => event.execute(client, ...args));
		console.log(event.name);
	} else {
		client.on(event.name, (...args) => event.execute(client, ...args));
		console.log(event.name);
	}
}

//Command Handler

const commandDirs = fs.readdirSync('./commands') //todo: make this only look for dirs
console.log('-----Loading Commands-----')
for (const dir of commandDirs) {
	const commandFiles = fs.readdirSync(`./commands/${dir}`).filter(file => file.endsWith('.js'));
	for (const file of commandFiles) {
		const command = require(`./commands/${dir}/${file}`);
		client.commands.set(command.data.name, command);
		console.log(command.data.name);
	}

}


//-----Erela events-----

// Emitted whenever a node connects
client.manager.on("nodeConnect", node => {
    console.log(`Node "${node.options.identifier}" connected.`)
})

// Emitted whenever a node encountered an error
client.manager.on("nodeError", (node, error) => {
    console.log(`Node "${node.options.identifier}" encountered an error: ${error.message}.`)
})

// THIS IS REQUIRED. Send raw events to Erela.js
client.on("raw", d => client.manager.updateVoiceState(d));


client.login(token);
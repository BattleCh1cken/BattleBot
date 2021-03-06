const { SlashCommandBuilder } = require('@discordjs/builders');

module.exports = {
	category: "misc",
	usage: "/ping",
	data:
		new SlashCommandBuilder()
			.setName('ping')
			.setDescription('Replies with Pong!'),
	async execute(interaction) {
		await interaction.reply('Pong!');
	},
};


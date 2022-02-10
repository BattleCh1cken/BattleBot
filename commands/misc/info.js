const { SlashCommandBuilder } = require('@discordjs/builders');
const { MessageActionRow, MessageButton } = require('discord.js');
module.exports = {
	category: "category-here", //current available categories [misc, music, fun]
	usage: "/info",   //use <> to indicate arguments, ie /play <song-name>
	data:
		new SlashCommandBuilder()
			.setName('info')
			.setDescription('Gives information about the server'),
	async execute(interaction) {
		const row = new MessageActionRow()
			.addComponents(
				new MessageButton()
					.setCustomId('ping')
					.setLabel('Ping')
					.setStyle('PRIMARY'),
				new MessageButton()
					.setCustomId("population")
					.setLabel("Population")
					.setStyle("PRIMARY")
			)

		await interaction.reply({ content: "Choose what info you want to see.", components: [row] })
		console.log(interaction.user.id)

		const filter = i => i.user.id === i.user.id;
		const collector = interaction.channel.createMessageComponentCollector({ filter, time: 15000 });

		collector.on('collect', async i => {
			if (i.customId == 'ping') {
				await i.reply({
					content: "pong",
					ephemeral: false
				})
			}else if (i.customId == 'population') {
				await i.reply({
					content: "Population: you",
					ephemeral: false
				})
			}else {console.log("no matching interaction")}
		});

		collector.on('end', collected => {
			console.log(`Collected ${collected.size} items`)
		}
		);
	},
};
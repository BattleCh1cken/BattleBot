const { SlashCommandBuilder } = require('@discordjs/builders');

module.exports = {
	category: "music",
	usage: "/play <song>",
	data:
		new SlashCommandBuilder()
			.setName('play')
			.setDescription('Plays a song')
			.addStringOption(option =>
				option.setName("song")
					.setDescription("The song you want to play")
					.setRequired(true)),
	async execute(interaction) {
		const search = interaction.options.getString("song")
		console.log(search)

		//If I had code it would go here

	},
};
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
	async execute(client, interaction) {
		const search = interaction.options.getString("song")
		let res
		console.log(search)
		try {
			res = await client.manager.search(search, interaction.author);
			if (res.loadType === "LOAD_FAILED") throw res.exception;
			else if (res.loadType === "PLAYLIST_LOADED") throw { message: "Playlists are not supported with this command." };
		} catch (err) {
			interaction.reply("There was an error while searching.")
		}
		const player = client.manager.create({
			guild: interaction.guild.id,
			voiceChannel: interaction.member.voice.channel.id,
			textChannel: interaction.channel.id,
		  });
		  player.connect();
		  player.queue.add(res.tracks[0]);
		  if (!player.playing && !player.paused && !player.queue.size) player.play()
		  return interaction.reply(`queuing ${res.tracks[0].title}.`)

		//If I had code it would go here

	},
};
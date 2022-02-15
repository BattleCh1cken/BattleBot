const { SlashCommandBuilder } = require('@discordjs/builders');
const {MessageActionRow, MessageButton } = require('discord.js');

module.exports = {
	category: "fun",
	usage: "/rps",
	data:
		new SlashCommandBuilder()
			.setName('rps')
			.setDescription('Plays a game of rock paper scissors'),

	async execute(interaction) {
        const row = new MessageActionRow()
			.addComponents(
				new MessageButton()
					.setCustomId('rock')
					.setLabel('Rock')
					.setStyle('PRIMARY'),
				new MessageButton()
					.setCustomId("paper")
					.setLabel("Paper")
					.setStyle("PRIMARY"),
                new MessageButton()
                    .setCustomId("scissors")
                    .setLabel("Scissors")
                    .setStyle("PRIMARY")
			)
            await interaction.reply({ content: "Choose your choice.", components: [row] })


            const filter = i => i.user.id === i.user.id;
            const collector = interaction.channel.createMessageComponentCollector({ filter, time: 15000 });
    
            collector.on('collect', async i => {
                if (i.customId == 'rock') {
                    await i.reply({
                        content: "rock",
                        ephemeral: false
                    })
                }else if (i.customId == 'paper') {
                    await i.reply({
                        content: "paper",
                        ephemeral: false
                    })
                }else if (i.customId == "scissors"){
                    await i.reply({
                        content: "scissors",
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


use crate::bot::Bot;
use crate::util;
use pea_api::get;
use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::application_command::CommandDataOptionValue;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::command::CommandOptionType;
use serenity::prelude::Context;
use serenity::utils::Color;
pub async fn run(bot: &Bot, ctx: &Context, command: &ApplicationCommandInteraction) {
    let option = command
        .data
        .options
        .get(0)
        .expect("Expected int option")
        .resolved
        .as_ref()
        .expect("Expected int object");
    if let CommandDataOptionValue::Integer(height) = option {
        let hash = match get::hash(&bot.api, &(*height as usize)).await {
            Ok(hash) => hash,
            Err(err) => err.to_string(),
        };
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message.embed(|e| {
                            e.color(Color::from_rgb(47, 49, 54)).field(
                                format!("Hash - {}", height),
                                util::markdown_code_block("ini", &format!("[{}]", hash)),
                                false,
                            )
                        })
                    })
            })
            .await
            .unwrap();
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hash").description("Get hash by height").create_option(|option| {
        option
            .name("height")
            .description("A positive integer")
            .kind(CommandOptionType::Integer)
            .min_int_value(0)
            .required(true)
    })
}

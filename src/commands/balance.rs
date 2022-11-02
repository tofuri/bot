use pea_api::get;
use serenity::{
    builder::CreateApplicationCommand,
    model::{
        application::interaction::{application_command::ApplicationCommandInteraction, application_command::CommandDataOptionValue, InteractionResponseType},
        prelude::command::CommandOptionType,
    },
    prelude::Context,
};
const HTTP_API: &str = "http://localhost:8080";
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    if let CommandDataOptionValue::String(address) = command.data.options.get(0).expect("Expected address option").resolved.as_ref().expect("Expected address object") {
        let balance = match get::balance(HTTP_API, address).await {
            Ok(a) => a.to_string(),
            Err(_) => "Unknown".to_string(),
        };
        let balance_staked = match get::balance_staked(HTTP_API, address).await {
            Ok(a) => a.to_string(),
            Err(_) => "Unknown".to_string(),
        };
        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| {
                    message
                        .content(format!(
                            r"```
Balance: {}
Staked: {}
```",
                            balance, balance_staked
                        ))
                        .embed(|e| {
                            e.title("This is a title")
                                .description("This is a description")
                                .image("attachment://ferris_eyes.png")
                                .fields(vec![
                                    ("This is the first field", "This is a field body", true),
                                    ("This is the second field", "Both fields are inline", true),
                                ])
                                .field("This is the third field", "This is not an inline field", false)
                                .footer(|f| f.text("This is a footer"))
                        })
                })
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        }
    }
}
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("balance")
        .description("Get balance of address")
        .create_option(|option| option.name("address").description("An address").kind(CommandOptionType::String).min_int_value(0).required(true))
}

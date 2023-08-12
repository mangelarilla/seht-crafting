mod requests;
mod components;
mod entities;

use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::model::application::command::Command;
use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::model::prelude::application_command::CommandDataOptionValue;
use serenity::model::prelude::command::CommandOptionType;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            command
                .name("menu")
                .description("Menu de solicitudes de crafteo")
                .create_option(|option| option
                    .name("precio")
                    .description("Precio por pieza de la solicitud")
                    .kind(CommandOptionType::Number)
                    .required(true))
                .create_option(|option| option
                    .name("rol")
                    .description("Rol de crafteadores")
                    .kind(CommandOptionType::Role)
                    .required(true))
        }).await;

        match guild_command {
            Ok(command) => info!("Registered global slash command: {}", command.name),
            Err(e) => error!("Error registering global command: {:#?}", e)
        }

    }

    // `interaction_create` runs when the user interacts with the bot
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Ping(ping) => {
                info!("Received ping interaction: {:#?}", ping)
            }
            Interaction::ApplicationCommand(command) => {
                info!("Received command interaction: {}", command.data.name);

                let menu_price = command.data.options.get(0)
                    .expect("Expected price").resolved.as_ref()
                    .expect("Expected price");

                let price = if let CommandDataOptionValue::Number(price) = menu_price {
                    price
                } else { &0f64 };

                let menu_role = command.data.options.get(1)
                    .expect("Expected role").resolved.as_ref()
                    .expect("Expected role");

                let role = if let CommandDataOptionValue::Role(role) = menu_role {
                    role
                } else {unreachable!("Expected role")};

                if let Err(why) = command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|m| m
                            .allowed_mentions(|m| m.roles([role]))
                            .content(components::menu_description(price, &role))
                            .components(|c| c.add_action_row(components::menu_action_row()))
                        )
                })
                    .await
                {
                    info!("Cannot respond to slash command: {}", why);
                }
            }
            Interaction::MessageComponent(component) => {
                info!("Received message component interaction: {}", component.data.custom_id);
                match component.data.custom_id.as_str() {
                    "Gear" => requests::gear::gear(component, &ctx).await,
                    "Consumables" => requests::consumable::consumables(component, &ctx).await,
                    "Enchantment" => requests::enchantment::enchantment(component, &ctx).await,
                    _ => info!("interaction {} not registered", component.data.custom_id)
                }
            }
            Interaction::Autocomplete(autocomplete) => {
                info!("Received autocomplete interaction: {:#?}", autocomplete)
            }
            Interaction::ModalSubmit(modal) => {
                info!("Received modal submit interaction: {}", modal.data.custom_id);
                match modal.data.custom_id.as_str() {
                    "gear_set_modal" => requests::gear::gear_modal(modal, &ctx).await,
                    "enchantment_modal" => requests::enchantment::enchantment_modal(modal, &ctx).await,
                    "consumables_modal" => requests::consumable::consumables_modal(modal, &ctx).await,
                    _ => unreachable!("interaction id not found")
                }
            }
        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Bot)
        .await
        .expect("Error creating client");

    Ok(client.into())
}

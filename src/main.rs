mod requests;
mod components;
mod entities;

use anyhow::anyhow;
use serenity::all::{CreateCommand, CreateCommandOption, CreateInteractionResponse, CreateInteractionResponseMessage};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::prelude::*;
use shuttle_secrets::SecretStore;
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);

        let guild_command = Command::create_global_command(&ctx.http, CreateCommand::new("menu")
            .description("Menu de solicitudes de crafteo")
            .add_option(
                CreateCommandOption::new(
                    CommandOptionType::Number, "precio", "Precio por pieza de la solicitud")
                    .required(true))
            .add_option(
                CreateCommandOption::new(CommandOptionType::Role, "rol", "Rol de crafteadores").required(true)
            )
        ).await;

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
            Interaction::Command(command) => {
                info!("Received command interaction: {}", command.data.name);

                let menu_price = command.data.options.get(0)
                    .expect("Expected price");

                let price = if let CommandDataOptionValue::Number(price) = menu_price.value {
                    price
                } else { 0f64 };

                let menu_role = command.data.options.get(1)
                    .expect("Expected role");

                let role = if let CommandDataOptionValue::Role(role) = menu_role.value {
                    role
                } else {unreachable!("Expected role")};

                if let Err(why) = command.create_response(&ctx.http, CreateInteractionResponse::Message(
                    CreateInteractionResponseMessage::new()
                        .content(components::menu_description(&price, role))
                        .components(vec![components::menu_action_row()])
                )).await {
                    info!("Cannot respond to slash command: {}", why);
                }
            }
            Interaction::Component(component) => {
                info!("Received message component interaction: {}", component.data.custom_id);
                match component.data.custom_id.as_str() {
                    "Gear" => requests::gear::gear(component, &ctx).await,
                    "GearResearch" => requests::gear::gear_research(component, &ctx).await,
                    "Consumables" => requests::consumable::consumables(component, &ctx).await,
                    "Enchantment" => requests::enchantment::enchantment(component, &ctx).await,
                    _ => info!("interaction {} not registered", component.data.custom_id)
                }
            }
            Interaction::Autocomplete(autocomplete) => {
                info!("Received autocomplete interaction: {:#?}", autocomplete)
            }
            Interaction::Modal(modal) => {
                info!("Received modal submit interaction: {}", modal.data.custom_id);
                match modal.data.custom_id.as_str() {
                    "gear_set_modal" => requests::gear::gear_modal(modal, &ctx).await,
                    "enchantment_modal" => requests::enchantment::enchantment_modal(modal, &ctx).await,
                    "consumables_modal" => requests::consumable::consumables_modal(modal, &ctx).await,
                    _ => unreachable!("interaction id not found")
                }
            }
            _ => {}
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

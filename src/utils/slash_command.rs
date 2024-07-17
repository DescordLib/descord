use std::collections::HashMap;

use log::{error, info};
use nanoserde::DeJson;
use reqwest::Method;

use super::*;
use crate::consts::permissions as perms;
use crate::internals::*;

use crate::models::application_command::ApplicationCommand;

fn map_param_type_to_u32(param_type: &ParamType) -> u32 {
    match param_type {
        ParamType::String => 3,
        ParamType::Int => 4,
        ParamType::User => 6,
        ParamType::Channel => 7,
        _ => 3,
    }
}

// TODO for fireplank, add checks for permissions
// TODO fix this stupid looking code

pub async fn register_slash_commands(commands: Vec<SlashCommand>) -> HashMap<String, SlashCommand> {
    let mut slash_commands = HashMap::new();
    let bot_id = get_bot_id().await;
    let registered_commands = fetch_application_commands(&bot_id).await;

    for local_command in &commands {
        let mut permissions: u64 = 0;
        for permission in &local_command.permissions {
            let permission = perms::parse(&permission).expect("unknown permission name");
            permissions |= permission;
        }

        let options = local_command
            .fn_param_names
            .iter()
            .zip(local_command.fn_param_autocomplete.iter())
            .zip(local_command.fn_param_renames.iter())
            .zip(local_command.fn_sig.iter())
            .zip(local_command.fn_param_descriptions.iter())
            .zip(local_command.optional_params.iter())
            .map(
                |(((((name, autocomplete), rename), type_), description), optional)| {
                    let name = rename.as_ref().unwrap_or_else(|| name);
                    json::object! {
                        name: name.clone(),
                        description: description.clone(),
                        type: map_param_type_to_u32(type_),
                        required: !optional,
                        autocomplete: autocomplete.is_some(),
                    }
                },
            )
            .collect::<Vec<_>>();

        // If the command exists in the fetched commands
        if let Some(registered_command) = registered_commands
            .iter()
            .find(|&cmd| cmd.name.as_str() == local_command.name)
        {
            let registered_options = &registered_command.clone().options.unwrap_or_default();
            let registered_types: Vec<u32> =
                registered_options.iter().map(|opt| opt.type_).collect();

            let registered_names: Vec<&str> = registered_options
                .iter()
                .map(|opt| opt.name.as_str())
                .collect();

            let registered_descriptions: Vec<&str> = registered_options
                .iter()
                .map(|opt| opt.description.as_str())
                .collect();

            let registered_optionals = registered_options
                .iter()
                .map(|opt| !opt.required.unwrap_or(false))
                .collect::<Vec<_>>();

            let registered_autocompletes = registered_options
                .iter()
                .map(|opt| opt.autocomplete.unwrap_or(false))
                .collect::<Vec<_>>();

            let fn_param_names = local_command
                .fn_param_names
                .iter()
                .zip(local_command.fn_param_renames.iter())
                .map(|(name, rename)| rename.as_ref().unwrap_or(name))
                .collect::<Vec<_>>();

            let fn_param_autocompletes = local_command
                .fn_param_autocomplete
                .iter()
                .map(|autocomplete| autocomplete.is_some())
                .collect::<Vec<_>>();

            let types = local_command
                .fn_sig
                .iter()
                .map(map_param_type_to_u32)
                .collect::<Vec<_>>();

            if local_command.description != registered_command.description
                || fn_param_names != registered_names
                || local_command.fn_param_descriptions != registered_descriptions
                || types != registered_types
                || fn_param_autocompletes != registered_autocompletes
                || local_command.optional_params != registered_optionals
            {
                let response = request(
                    Method::PATCH,
                    format!("applications/{}/commands/{}", bot_id, registered_command.id).as_str(),
                    Some(json::object! {
                        name: local_command.name.clone(),
                        description: local_command.description.clone(),
                        options: options,
                        default_member_permissions: permissions.to_string(),
                    }),
                )
                .await
                .text()
                .await
                .unwrap();

                info!(
                    "Updated '{}' slash command, command id: {}",
                    local_command.name, registered_command.id,
                );

                slash_commands.insert(registered_command.id.clone(), local_command.clone());
            } else {
                info!(
                    "No changes detected in '{}' slash command, command id: {}",
                    local_command.name, registered_command.id,
                );

                slash_commands.insert(registered_command.id.clone(), local_command.clone());
            }
        } else {
            // If the command does not exist in the fetched commands, register it
            let response = request(
                Method::POST,
                format!("applications/{}/commands", bot_id).as_str(),
                Some(json::object! {
                    name: local_command.name.clone(),
                    description: local_command.description.clone(),
                    options: options,
                    default_member_permissions: permissions.to_string(),
                }),
            )
            .await
            .text()
            .await
            .unwrap();

            println!("umm: {}", json::parse(&response).unwrap().pretty(4));
            let command_id = json::parse(&response).expect("Failed to parse JSON response")["id"]
                .as_str()
                .expect("Failed to get 'id' from JSON response")
                .to_string();

            info!(
                "Registered '{}' slash command, command id: {}",
                local_command.name, command_id
            );

            slash_commands.insert(command_id, local_command.clone());
        }
    }

    for registered_command in registered_commands {
        // If the command does not exist in the local commands, remove it
        if commands
            .iter()
            .find(|&cmd| cmd.name == registered_command.name)
            .is_none()
        {
            request(
                Method::DELETE,
                format!("applications/{}/commands/{}", bot_id, registered_command.id).as_str(),
                None,
            )
            .await;

            info!(
                "Removed slash command '{}', command id: {}",
                registered_command.name, registered_command.id
            );
        }
    }

    slash_commands
}

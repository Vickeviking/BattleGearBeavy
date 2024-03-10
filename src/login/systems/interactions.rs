use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;
use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInputSettings;
use bevy_simple_text_input::TextInputValue;

use crate::login::styles::*;
use crate::login::DebugLogin;
use crate::login::components::*;
use crate::{AppState, CacheKey, NextState};
use crate::API_URL;
use crate::components::Credentials;
use std::io;

use tokio::runtime::Runtime;
use reqwest::StatusCode;
use serde_json::json;
use reqwest::{Error, Response};
use std::io::Error as IoError;



// ==================  Button Interactions  ================== //

pub fn  interact_with_login_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), 
                            (Changed<Interaction>, With<LoginButton>)>,
    username_query: Query<(Entity, &LoginUsernameInputField)>,
    password_query: Query<(Entity, &LoginPasswordInputField)>,
    mut cache_key:  ResMut<CacheKey>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut error_msg_query: Query<&mut LoginErrorMsg>, 
) {
    if let Ok((interaction, mut background_color)) 
        = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_COLOR_CLICKED.into();
                // get the username and password from query
                let (username_entity, username) = username_query.get_single().unwrap();
                let (password_entity, password) = password_query.get_single().unwrap();
                // check if the login form is valid
                let login_error = check_login_form(username, password, &mut cache_key);
                let err_text = login_error_enum_to_string(login_error);
                if let Ok(mut error_msg) = error_msg_query.get_single_mut() {
                    error_msg.0 = err_text;
                }

                // if Ok, the user was login lets change to the next state
                if login_error == LoginErrorMsgEnum::Ok {
                    app_state_next_state.set(AppState::Dashboard);
                }
    

            }
            Interaction::Hovered => {
                *background_color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *background_color = BUTTON_COLOR.into();
            }
            _ => {}
        }
    }
}

pub fn interact_with_go_to_register_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<GoToRegisterButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) 
        = button_query.get_single_mut() {
        print!("{:?}", interaction);
        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_COLOR_CLICKED.into();
                app_state_next_state.set(AppState::Register);
            }
            Interaction::Hovered => {
                *background_color = BUTTON_COLOR_HOVER.into();
            }
            Interaction::None => {
                *background_color = BUTTON_COLOR.into();
            }
            _ => {}
        }
    }
}


// ==================  Text Input Field Interactions  ================== //


// input field listener
pub fn listener(
    mut commands: Commands,
    mut username_input_field_query: Query<(Entity, &LoginUsernameInputField, &TextInputValue)>,
    mut password_input_field_query: Query<(Entity, &LoginPasswordInputField, &TextInputValue)>,
) {
    for (entity, username_input_field, text_input_value) in username_input_field_query.iter() {
        if username_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(LoginUsernameInputField(text_input_value.0.clone()));
        }
    }

    for (entity, password_input_field, text_input_value) in password_input_field_query.iter() {
        if password_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(LoginPasswordInputField(text_input_value.0.clone()));
        }
    }
    
}

pub fn focus(
    query: Query<(Entity, &Interaction), Changed<Interaction>>,
    mut text_input_query: Query<(Entity, &mut TextInputInactive, &mut BorderColor)>,
) {
    for (interaction_entity, interaction) in &query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive, mut border_color) in &mut text_input_query {
                if entity == interaction_entity {
                    inactive.0 = false;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_INACTIVE.into();
                }
            }
        }
    }
}
 
// make all input fields inactive
pub fn setup_text_input(
    mut commands: Commands,
    query: Query<Entity, With<LoginUsernameInputField>>,
    query2: Query<Entity, With<LoginPasswordInputField>>,
) {
    commands.entity(query.get_single().unwrap())
        .insert(TextInputInactive(true))
        .insert(TextInputSettings {
            retain_on_submit: true
        })
        .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));

    commands.entity(query2.get_single().unwrap())
        .insert(TextInputInactive(true))
        .insert(TextInputSettings {
            retain_on_submit: true
        })
        .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));
}

pub fn on_submit(
    mut commands: Commands,
    mut events: EventReader<TextInputSubmitEvent>,
    username_query: Query<(Entity, &LoginUsernameInputField)>,
    password_query: Query<(Entity, &LoginPasswordInputField)>,
) {
    for event in events.read() {
        info!("submitted: {}", event.value);

        // Check if the entity has the UsernameInputField component
        if let Ok((entity, _)) = username_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(LoginUsernameInputField(event.value.clone()));
            }
        }

        // Check if the entity has the PasswordInputField component
        if let Ok((entity, _)) = password_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(LoginPasswordInputField(event.value.clone()));
            }
        }
    }
}

pub fn check_login_form(
    username: &LoginUsernameInputField,
    password: &LoginPasswordInputField,
    cacheResouce: &mut ResMut<CacheKey>,
) -> LoginErrorMsgEnum {
    
    // make username and password into text
    let username = username.0.clone();
    let password = password.0.clone();

    if username.is_empty() || password.is_empty() {
        return LoginErrorMsgEnum::InvalidCredentials;
    }
    //check if the username has empty spaces
    if username.contains(" ") {
        return LoginErrorMsgEnum::InvalidCredentials;
    }

    // create user credentials
    let credentials = Credentials {
        username: username.clone(),
        password: password.clone(),
    };
    // call login function
    let token = login_sync(credentials);
    // check if the login was successful
    if token.is_err() {
        return LoginErrorMsgEnum::InvalidCredentials;
    } 
    // save the token in the cache
    cacheResouce.0 = token.unwrap();
    LoginErrorMsgEnum::Ok
}

// error message system

pub fn update_error_message_system(
    mut text_query: Query<(&mut Text, &LoginErrorMsg)>,
) {
    for (mut text, error_msg) in text_query.iter_mut() {
        if let Some(section) = text.sections.first_mut() {
            section.value = error_msg.0.clone();
        }
    }
}


// login functionality
pub async fn login(credentials: Credentials) -> Result<String, Error> {
    //debug credentials
    if DebugLogin {
        info!("username: {}", credentials.username);
        info!("password: {}", credentials.password);
    }

    let url = format!("{}/login", API_URL);
    let client = reqwest::Client::new();
    let response = client.post(&url)
        .json(&json!({
            "username": credentials.username,
            "password": credentials.password,
        }))
        .send()
        .await?;

    // debug response
    if DebugLogin {
        info!("response: {:?}", response);
    }

    if response.status() != StatusCode::OK {
        // return reqwest error
        return Err(response.error_for_status().unwrap_err());
    }
    

    let json: serde_json::Value = response.json().await?;
    let token = json.get("token").unwrap().as_str().unwrap();

    Ok(token.to_string())
}

pub fn login_sync(credentials: Credentials) -> Result<String, Error> {
    let rt = Runtime::new().unwrap();
    rt.block_on(login(credentials))
}
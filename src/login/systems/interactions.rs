use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;
use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInputSettings;
use bevy_simple_text_input::TextInputValue;

use crate::login::styles::*;
use crate::login::DebugLogin;
use crate::login::components::*;
use crate::AppState;
use crate::API_URL;


use reqwest::Error;



// ==================  Button Interactions  ================== //

pub fn  interact_with_login_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), 
                            (Changed<Interaction>, With<LoginButton>)>
) {
    if let Ok((interaction, mut background_color)) 
        = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_COLOR_CLICKED.into();
                // TODO: Add logic to log in
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
    mut login_query: Query<(&LoginUsernameInputField, &LoginPasswordInputField)>,
    mut login_button_query: Query<(&mut BackgroundColor), With<LoginButton>>,
) -> LoginErrorMsgEnum {
    
    let (LoginUsernameInputField(username), LoginPasswordInputField(password)) = login_query.get_single().unwrap();

    if username.is_empty() || password.is_empty() {
        return LoginErrorMsgEnum::InvalidCredentials;
    }

    //check if the username has empty spaces
    if username.contains(" ") {
        return LoginErrorMsgEnum::InvalidCredentials;
    }

    // TODO: Add logic to check if the user exists in the database

    LoginErrorMsgEnum::Ok
}

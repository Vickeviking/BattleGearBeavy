use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;
use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInputSettings;
use bevy_simple_text_input::TextInputValue;
use tokio::runtime::Runtime;
use reqwest::Error;
use reqwest::StatusCode;
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};

use crate::api::utility::hash_password;
use crate::register::styles::*;
use crate::register::components::*;
use crate::register::DebugRegister;
use crate::components::{NewUser, Credentials, LoginResponse};
use crate::{AppState, CacheKey};
use crate::API_URL;


//TODO: 

/*

Some type of idea, add a component that has a string, 
during the function change this component to the error message

add a system that updates the error message to the component each frame

*/



// ==================  Button Interactions  ================== //

pub fn  interact_with_go_to_login_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<GoToLoginButton>)
        >,
    mut app_state_next_state: ResMut<NextState<AppState>>
    
) {
    if let Ok((interaction, mut background_color)) 
        = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_COLOR_CLICKED.into();
                app_state_next_state.set(AppState::Login);
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

pub fn interact_with_register_button(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<RegisterButton>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    username_query: Query<(Entity, &UsernameInputField)>,
    email_query: Query<(Entity, &EmailInputField)>,
    password_query: Query<(Entity, &PasswordInputField)>,
    full_name_query: Query<(Entity, &FullNameInputField)>,
    country_query: Query<(Entity, &CountryInputField)>,
    date_of_birth_query: Query<(Entity, &DateOfBirthInputField)>,
    mut error_msg_query: Query<&mut RegisterErrorMsg>, 
) {
    if let Ok((interaction, mut background_color)) 
        = button_query.get_single_mut() {
        if DebugRegister {
            print!("{:?}", interaction);
        }

        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_COLOR_CLICKED.into();
                // we should call submit function here to get the values of the input fields
                // into the components 
                let username = username_query.iter().next().map_or_else(|| "".to_string(), |(_, field)| field.0.clone());
                let email = email_query.iter().next().map_or_else(|| "".to_string(), |(_, field)| field.0.clone());
                let password_hash = password_query.iter().next().map_or_else(|| "".to_string(), |(_, field)| field.0.clone());
                let full_name = full_name_query.iter().next().map_or_else(|| "".to_string(), |(_, field)| field.0.clone());
                let country = country_query.iter().next().map_or_else(|| "".to_string(), |(_, field)| field.0.clone());
                let date_of_birth = date_of_birth_query.iter().next().map_or_else(|| "".to_string(), |(_, field)| field.0.clone());
                // convert date_of_birth yyyymmdd to yyyy-mm-dd
                let date_of_birth = format!("{}-{}-{}", &date_of_birth[0..4], &date_of_birth[4..6], &date_of_birth[6..8]);


                let err_enum: ErrorEnum = check_register_form(
                    username_query,
                    email_query,
                    password_query,
                    full_name_query,
                    country_query,
                    date_of_birth_query
                );

                let err_text = error_enum_to_string(err_enum);

                // Change the error message to the component
                if let Ok(mut error_msg) = error_msg_query.get_single_mut() {
                    error_msg.0 = err_text;
                }

                if err_enum == ErrorEnum::Ok {
                    // lets register the user

                    let user = NewUser {
                        username: username.clone(),
                        email,
                        password_hash: password_hash.clone(),
                        full_name,
                        country,
                        date_of_birth,
                    };
                    
                    match create_user_sync(user) {
                        Ok(()) => info!("User created successfully"),
                        Err(e) => eprintln!("Failed to create user: {}", e),
                    }
                    
                    app_state_next_state.set(AppState::Login);
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


// ==================  Text Input Field Interactions  ================== //

// input field listener
pub fn listener(
    mut commands: Commands,
    mut username_input_field_query: Query<(Entity, &UsernameInputField, &TextInputValue)>,
    mut password_input_field_query: Query<(Entity, &PasswordInputField, &TextInputValue)>,
    mut email_input_field_query: Query<(Entity, &EmailInputField, &TextInputValue)>,
    mut full_name_input_field_query: Query<(Entity, &FullNameInputField, &TextInputValue)>,
    mut country_input_field_query: Query<(Entity, &CountryInputField, &TextInputValue)>,
    mut date_of_birth_input_field_query: Query<(Entity, &DateOfBirthInputField, &TextInputValue)>,
) {
    // lets see if the see if the TextInputValue doesnt match the input field
    for (entity, username_input_field, text_input_value) in username_input_field_query.iter() {
        if username_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(UsernameInputField(text_input_value.0.clone()));
        }
    }

    for (entity, password_input_field, text_input_value) in password_input_field_query.iter() {
        if password_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(PasswordInputField(text_input_value.0.clone()));
        }
    }

    for (entity, email_input_field, text_input_value) in email_input_field_query.iter() {
        if email_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(EmailInputField(text_input_value.0.clone()));
        }
    }

    for (entity, full_name_input_field, text_input_value) in full_name_input_field_query.iter() {
        if full_name_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(FullNameInputField(text_input_value.0.clone()));
        }
    }

    for (entity, country_input_field, text_input_value) in country_input_field_query.iter() {
        if country_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(CountryInputField(text_input_value.0.clone()));
        }
    }

    for (entity, date_of_birth_input_field, text_input_value) in date_of_birth_input_field_query.iter() {
        if date_of_birth_input_field.0 != text_input_value.0 {
            commands.entity(entity).insert(DateOfBirthInputField(text_input_value.0.clone()));
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
                    *border_color = BORDER_COLOR_INACTIVE.into();
                } else {
                    inactive.0 = true;
                    *border_color = BORDER_COLOR_ACTIVE.into();
                }
            }
        }
    }
}
 
// make all input fields inactive
pub fn setup_text_input(
    mut commands: Commands,
    query: Query<Entity, With<UsernameInputField>>,
    query2: Query<Entity, With<EmailInputField>>,
    query3: Query<Entity, With<PasswordInputField>>,
    query4: Query<Entity, With<FullNameInputField>>,
    query5: Query<Entity, With<CountryInputField>>,
    query6: Query<Entity, With<DateOfBirthInputField>>
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity)
            .insert(TextInputInactive(true))
            .insert(TextInputSettings {
                retain_on_submit: true
            })
            .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));
    }

    if let Ok(entity) = query2.get_single() {
        commands.entity(entity)
            .insert(TextInputInactive(true))
            .insert(TextInputSettings {
                retain_on_submit: true
            })
            .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));
    }

    if let Ok(entity) = query3.get_single() {
        commands.entity(entity)
            .insert(TextInputInactive(true))
            .insert(TextInputSettings {
                retain_on_submit: true
            })
            .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));
    }

    if let Ok(entity) = query4.get_single() {
        commands.entity(entity)
            .insert(TextInputInactive(true))
            .insert(TextInputSettings {
                retain_on_submit: true
            })
            .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));
    }

    if let Ok(entity) = query5.get_single() {
        commands.entity(entity)
            .insert(TextInputInactive(true))
            .insert(TextInputSettings {
                retain_on_submit: true
            })
            .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));
    }

    if let Ok(entity) = query6.get_single() {
        commands.entity(entity)
            .insert(TextInputInactive(true))
            .insert(TextInputSettings {
                retain_on_submit: true
            })
            .insert(BorderColor(BORDER_COLOR_ACTIVE.into()));
    }
    
}

pub fn on_submit(
    mut commands: Commands,
    mut events: EventReader<TextInputSubmitEvent>,
    username_query: Query<(Entity, &UsernameInputField)>,
    email_query: Query<(Entity, &EmailInputField)>,
    password_query: Query<(Entity, &PasswordInputField)>,
    full_name_query: Query<(Entity, &FullNameInputField)>,
    country_query: Query<(Entity, &CountryInputField)>,
    date_of_birth_query: Query<(Entity, &DateOfBirthInputField)>,

) {
    for event in events.read() {
        if DebugRegister {
            print!("submitted: {}", event.value);
        }

        // Check if the entity has the UsernameInputField component
        if let Ok((entity, _)) = username_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(UsernameInputField(event.value.clone()));
            }
        }

        // check if the entity has the mail entity
        if let Ok((entity, _)) = email_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(EmailInputField(event.value.clone()));
            }
        }

        // Check if the entity has the PasswordInputField component
        if let Ok((entity, _)) = password_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(PasswordInputField(event.value.clone()));
            }
        }

        // check if the entity has the full name entity
        if let Ok((entity, _)) = full_name_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(FullNameInputField(event.value.clone()));
            }
        }

        // check if the entity has the country entity
        if let Ok((entity, _)) = country_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(CountryInputField(event.value.clone()));
            }
        }

        // check if the entity has the date of birth entity
        if let Ok((entity, _)) = date_of_birth_query.get_single() {
            if entity == event.entity {
                commands.entity(entity).insert(DateOfBirthInputField(event.value.clone()));
            }
        }
    }
}


pub fn check_register_form(
    username_query: Query<(Entity, &UsernameInputField)>,
    email_query: Query<(Entity, &EmailInputField)>,
    password_query: Query<(Entity, &PasswordInputField)>,
    full_name_query: Query<(Entity, &FullNameInputField)>,
    country_query: Query<(Entity, &CountryInputField)>,
    date_of_birth_query: Query<(Entity, &DateOfBirthInputField)>,
) -> ErrorEnum {

    // username
    let (_entity, username_input_field) = username_query.get_single().unwrap();
    let UsernameInputField(username) = username_input_field;
    // email
    let (_entity, email_input_field) = email_query.get_single().unwrap();
    let EmailInputField(email) = email_input_field;
    // password
    let (_entity, password_input_field) = password_query.get_single().unwrap();
    let PasswordInputField(password) = password_input_field;
    // full name
    let (_entity, full_name_input_field) = full_name_query.get_single().unwrap();
    let FullNameInputField(full_name) = full_name_input_field;
    // country
    let (_entity, country_input_field) = country_query.get_single().unwrap();
    let CountryInputField(country) = country_input_field;
    // date of birth
    let (_entity, date_of_birth_input_field) = date_of_birth_query.get_single().unwrap();
    let DateOfBirthInputField(date_of_birth) = date_of_birth_input_field;

    // debug , print out what each field contains
    if DebugRegister {
        info!("username: {}", username);
        info!("email: {}", email);
        info!("password: {}", password);
        info!("full name: {}", full_name);
        info!("country: {}", country);
        info!("date of birth: {}", date_of_birth);
    }

    //check that all fields are filled
    if username.is_empty() || email.is_empty() || password.is_empty() || full_name.is_empty() || country.is_empty() || date_of_birth.is_empty() {
        return ErrorEnum::EmptyFields;
    }

    if DebugRegister {
        info!("All fields are filled");
    }

    // check if username has special characters or spaces
    if username.contains(' ') || username.contains(|c: char| !c.is_alphanumeric()) {
        return ErrorEnum::UsernameWithSpecialCharacters;
    }

    if DebugRegister {
        info!("Username has no special characters or spaces");
    }

    // check if username is taken
    if check_if_username_taken_sync(username).unwrap() {
        return ErrorEnum::UsernameTaken;
    }

    if DebugRegister {
        info!("Username is not taken");
    }
    

    // check if email is valid, (no spaces, has @, has .)
    if !email.contains('@') || !email.contains('.') || email.contains(' ') {
        return ErrorEnum::EmailNotValid;
    }

    if DebugRegister {
        info!("Email is valid");
    }

    // check if email is taken
    if check_if_email_taken_sync(email).unwrap() {
        return ErrorEnum::EmailTaken;
    }

    if DebugRegister {
        info!("Email is not taken");
    }
    
    // check if password is too short
    if password.len() < 8 {
        return ErrorEnum::PasswordTooShort;
    }

    if DebugRegister {
        info!("Password is not too short");
    }

    // check if password is too long
    if password.len() > 50 {
        return ErrorEnum::PasswordTooLong;
    }

    if DebugRegister {
        info!("Password is not too long");
    }

    // check if password has at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return ErrorEnum::PasswordNoUpperCase;
    }

    if DebugRegister {
        info!("Password has at least one uppercase letter");
    }

    // check if password has at least one number
    if !password.chars().any(|c| c.is_numeric()) {
        return ErrorEnum::PasswordNoNumber;
    }

    if DebugRegister {
        info!("Password has at least one number");
    }

    // check if full name has a space
    if username.contains(' ') {
        return ErrorEnum::FullNameNoSpace;
    }

    if DebugRegister {
        info!("Full name has a space");
    }

    // check if date of birth is in the correct format (YYYYMMDD)
    if date_of_birth.len() != 8 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    if DebugRegister {
        info!("Date of birth is in the correct format");
    }

    // first 4 characters should be a year 1900-2100
    let year = &date_of_birth[0..4];
    if year.parse::<i32>().unwrap() < 1900 || year.parse::<i32>().unwrap() > 2100 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    if DebugRegister {
        info!("Year is between 1900 and 2100");
    }

    // next 2 characters should be a month 01-12
    let month = &date_of_birth[4..6];
    if month.parse::<i32>().unwrap() < 1 || month.parse::<i32>().unwrap() > 12 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    if DebugRegister {
        info!("Month is between 01 and 12");
    }

    // last 2 characters should be a day 01-31
    let day = &date_of_birth[6..8];
    if day.parse::<i32>().unwrap() < 1 || day.parse::<i32>().unwrap() > 31 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    if DebugRegister {
        info!("Day is between 01 and 31");
    }

    ErrorEnum::Ok
}


//TODO: pub fn update_register_error_msg

pub fn check_if_username_taken_sync(username: &str) -> Result<bool, Error> {
    let rt = Runtime::new().unwrap();
    rt.block_on(check_if_username_taken(username))
}

pub fn check_if_email_taken_sync(email: &str) -> Result<bool, Error> {
    let rt = Runtime::new().unwrap();
    rt.block_on(check_if_email_taken(email))
}

pub async fn check_if_username_taken(username: &str) -> Result<bool, Error> {
    // check if username is taken
    let url = format!("{}/users/username_exists/{}", API_URL, username);
    let response: bool = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

pub async fn check_if_email_taken(email: &str) -> Result<bool, Error> {
    // check if email is taken
    let url = format!("{}/users/email_exists/{}", API_URL, email);
    let response: bool = reqwest::get(&url).await?.json().await?;
    Ok(response)
}

// error message system

pub fn update_error_message_system(
    mut text_query: Query<(&mut Text, &RegisterErrorMsg)>,
) {
    for (mut text, error_msg) in text_query.iter_mut() {
        if let Some(section) = text.sections.first_mut() {
            section.value = error_msg.0.clone();
        }
    }
}

// creating user
pub async fn create_user(mut user: NewUser) -> Result<(), reqwest::Error> {
    let url = format!("{}/users", API_URL);
    let client = reqwest::Client::new();

    //hash password 
    match hash_password(user.password_hash.clone()) {
        Ok(hashed_password) => {
            user.password_hash = hashed_password;
        }
        _ => {
            info!("Could not hash");
        }
    }

    let response = client.post(&url)
        .json(&json!({
            "username": user.username,
            "email": user.email,
            "password_hash": user.password_hash,
            "full_name": user.full_name,
            "country": user.country,
            "date_of_birth": user.date_of_birth,
        }))
        .send()
        .await?;

    assert!(response.status().is_success());
    
    Ok(())
}

pub fn create_user_sync(user: NewUser) -> Result<(), Error> {
    let rt = Runtime::new().unwrap();
    rt.block_on(create_user(user))
}

// login user

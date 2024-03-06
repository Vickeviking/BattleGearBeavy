use bevy::prelude::*;
use bevy_simple_text_input::TextInputSubmitEvent;
use bevy_simple_text_input::TextInputInactive;
use bevy_simple_text_input::TextInputSettings;

use crate::register::styles::*;
use crate::register::components::*;
use crate::AppState;
use crate::API_URL;





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

pub fn interact_with_go_to_register_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor), 
        (Changed<Interaction>, With<RegisterButton>)
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>
) {
    if let Ok((interaction, mut background_color)) 
        = button_query.get_single_mut() {
        print!("{:?}", interaction);
        match *interaction {
            Interaction::Pressed => {
                *background_color = BUTTON_COLOR_CLICKED.into();
                //TODO: handle logic to register user
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
pub fn listener(mut events: EventReader<TextInputSubmitEvent>) {
    for event in events.read() {
        info!("{:?} submitted: {}", event.entity, event.value);
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
        info!("submitted: {}", event.value);

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

    /*
         UsernameTaken,
        UsernameWithSpecialCharacters,
        EmailTaken,
        EmailNotValid,
        PasswordTooShort,
        PasswordTooLong,
        PasswordNoUpperCase,
        PasswordNoNumber,
        FullNameNoSpace,
        DateOfBirthNotValidFormat,
        Ok,
     */

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

    // check if username has special characters or spaces
    if username.contains(' ') || username.contains(|c: char| !c.is_alphanumeric()) {
        return ErrorEnum::UsernameWithSpecialCharacters;
    }
    // check if username is taken
    if check_if_username_taken(username) {
        return ErrorEnum::UsernameTaken;
    }
    // check if email is valid, (no spaces, has @, has .)
    if !email.contains('@') || !email.contains('.') || email.contains(' ') {
        return ErrorEnum::EmailNotValid;
    }

    // check if email is taken
    if check_if_email_taken(email) {
        return ErrorEnum::EmailTaken;
    }

    // check if password is too short
    if password.len() < 8 {
        return ErrorEnum::PasswordTooShort;
    }

    // check if password is too long
    if password.len() > 50 {
        return ErrorEnum::PasswordTooLong;
    }

    // check if password has at least one uppercase letter
    if !password.chars().any(|c| c.is_uppercase()) {
        return ErrorEnum::PasswordNoUpperCase;
    }

    // check if password has at least one number
    if !password.chars().any(|c| c.is_numeric()) {
        return ErrorEnum::PasswordNoNumber;
    }

    // check if full name has a space
    if username.contains(' ') {
        return ErrorEnum::FullNameNoSpace;
    }

    // check if date of birth is in the correct format (YYYYMMDD)
    if date_of_birth.len() != 8 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    // first 4 characters should be a year 1900-2100
    let year = &date_of_birth[0..4];
    if year.parse::<i32>().unwrap() < 1900 || year.parse::<i32>().unwrap() > 2100 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    // next 2 characters should be a month 01-12
    let month = &date_of_birth[4..6];
    if month.parse::<i32>().unwrap() < 1 || month.parse::<i32>().unwrap() > 12 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    // last 2 characters should be a day 01-31
    let day = &date_of_birth[6..8];
    if day.parse::<i32>().unwrap() < 1 || day.parse::<i32>().unwrap() > 31 {
        return ErrorEnum::DateOfBirthNotValidFormat;
    }

    ErrorEnum::Ok
}


pub fn check_if_username_taken(username: &str) -> bool {
    // check if username is taken
    let url = format!("{}/users/username_exists/{}", API_URL, username);

    // TODO: make a request to the server to check if the username is taken

    /* let client = reqwest::blocking::Client::new();
    let response = client.get(&url).send().unwrap(); */
    
    false
}

pub fn check_if_email_taken(email: &str) -> bool {
    // check if email is taken
    let url = format!("{}/users/email_exists/{}", API_URL, email);

    // TODO: make a request to the server to check if the email is taken

    false
}



use bevy::prelude::*;

#[derive(Component)]
pub struct Register {}



#[derive(Component)]
pub struct RegisterErrorMsg {}

#[derive(Component)]
pub struct GoToLoginButton {}

#[derive(Component)]
pub struct RegisterButton {}



#[derive(Component)]
pub struct UsernameInputField(pub String);

#[derive(Component)]
pub struct EmailInputField(pub String);

#[derive(Component)]
pub struct PasswordInputField(pub String);

#[derive(Component)]
pub struct FullNameInputField(pub String);

#[derive(Component)]
pub struct CountryInputField(pub String);

#[derive(Component)]
pub struct DateOfBirthInputField(pub String);



#[derive(Component)]
pub struct RegisterImageMarker;


#[derive(Component)]
pub enum ErrorEnum {
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
    Ok
}

pub fn error_enum_to_string(error_enum: ErrorEnum) {
    match error_enum {
        ErrorEnum::UsernameTaken => {
            info!("Username is taken");
        }
        ErrorEnum::UsernameWithSpecialCharacters => {
            info!("Username has special characters or spaces");
        }
        ErrorEnum::EmailTaken => {
            info!("Email is taken");
        }
        ErrorEnum::EmailNotValid => {
            info!("Email is not valid");
        }
        ErrorEnum::PasswordTooShort => {
            info!("Password must be at least 8 characters long");
        }
        ErrorEnum::PasswordTooLong => {
            info!("Password can be at most 50 characters long");
        }
        ErrorEnum::PasswordNoUpperCase => {
            info!("Password has no uppercase letter");
        }
        ErrorEnum::PasswordNoNumber => {
            info!("Password has no number");
        }
        ErrorEnum::FullNameNoSpace => {
            info!("separate first name and last name with a space");
        }
        ErrorEnum::DateOfBirthNotValidFormat => {
            info!("Date of birth should be in the format YYYYMMDD");
        }
        ErrorEnum::Ok => {
            info!("Ok");
        }
    }
}
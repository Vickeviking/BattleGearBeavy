use bevy::prelude::*;

#[derive(Component)]
pub struct Register {}



#[derive(Component)]
pub struct RegisterErrorMsg(pub String);

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
    NetworkError,
    EmptyFields,
    Ok
}



pub fn error_enum_to_string(error_enum: ErrorEnum) -> String {
    match error_enum {
        ErrorEnum::UsernameTaken => {
            "Username is taken".to_string()
        }
        ErrorEnum::UsernameWithSpecialCharacters => {
            "Username has special characters or spaces".to_string()
        }
        ErrorEnum::EmailTaken => {
            "Email is taken".to_string()
        }
        ErrorEnum::EmailNotValid => {
            "Email is not valid".to_string()
        }
        ErrorEnum::PasswordTooShort => {
            "Password must be at least 8 characters long".to_string()
        }
        ErrorEnum::PasswordTooLong => {
            "Password can be at most 50 characters long".to_string()
        }
        ErrorEnum::PasswordNoUpperCase => {
            "Password has no uppercase letter".to_string()
        }
        ErrorEnum::PasswordNoNumber => {
            "Password has no number".to_string()
        }
        ErrorEnum::FullNameNoSpace => {
            "separate first name and last name with a space".to_string()
        }
        ErrorEnum::DateOfBirthNotValidFormat => {
            "Date of birth should be in the format YYYYMMDD".to_string()
        }
        ErrorEnum::NetworkError => {
            "Network error".to_string()
        }
        ErrorEnum::EmptyFields => {
            "Empty fields".to_string()
        }
        ErrorEnum::Ok => {
            "Ok".to_string()
        }
    }
}
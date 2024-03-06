use bevy::prelude::*;

#[derive(Component)]
pub struct Login {}

#[derive(Component)]
pub struct LoginButton {}

#[derive(Component)]
pub struct GoToRegisterButton {}



#[derive(Component)]
pub struct LoginUsernameInputField(pub String);

#[derive(Component)]
pub struct LoginPasswordInputField(pub String);

#[derive(Component)] 
pub struct LoginErrorMsg {}

#[derive(Component)]
pub enum LoginErrorMsgEnum {
    InvalidCredentials,
    NetworkError,
    ServerError,
    UnknownError,
    Ok
}

pub fn login_error_enum_to_string(error_enum: LoginErrorMsgEnum) -> String {
    match error_enum {
        LoginErrorMsgEnum::InvalidCredentials => "Invalid credentials".to_string(),
        LoginErrorMsgEnum::NetworkError => "Network error".to_string(),
        LoginErrorMsgEnum::ServerError => "Server error".to_string(),
        LoginErrorMsgEnum::UnknownError => "Unknown error".to_string(),
        LoginErrorMsgEnum::Ok => "Ok".to_string()
    }
}
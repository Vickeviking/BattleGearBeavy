use bevy::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Resource)]
pub struct MyAssets {
    pub hero_image: Handle<Image>,
    pub fira_sans_bold: Handle<Font>,
    pub irish_grover: Handle<Font>,
    pub anton: Handle<Font>,


}

#[derive(Serialize)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub full_name: String,
    pub country: String,
    pub date_of_birth: String,
}

#[derive(Serialize, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub token: String,
}


#[derive(Component)]
pub struct Camera{}
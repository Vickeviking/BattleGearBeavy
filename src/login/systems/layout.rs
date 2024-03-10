use bevy::a11y::accesskit::Node;
use bevy::ui::widget::UiImageSize;
use bevy::{prelude::*};
use bevy_simple_text_input::TextInputBundle;
use bevy_round_ui::prelude::*;
use bevy::ui::ContentSize;

use crate::components::MyAssets;
use crate::login::styles::*;


use crate::login::components::{LoginErrorMsg, Login, LoginButton, LoginPasswordInputField, GoToRegisterButton, LoginUsernameInputField};

pub fn spawn_login(mut commands: Commands, assets: Res<MyAssets>, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    let login_entity = build_login(&mut commands, &assets, materials);
}

pub fn despawn_login(mut commands: Commands, login_query: Query<Entity, With<Login>>) {
    if let Ok(login_entity) = login_query.get_single() {
        commands.entity(login_entity).despawn_recursive(); // recursive to despawn all child ui nodes
    }
}


// not a system, a rust function that 
pub fn build_login(commands: &mut Commands, assets: &Res<MyAssets>, mut materials: ResMut<Assets<RoundUiMaterial>>) -> Entity {
    let login_entity = commands.spawn(
(
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    row_gap: Val::Px(80.0),
                    ..default()
                },
                background_color: BACK_GROUND_COLOR.into(),
                ..default()
            },
            Login {}, // mark this entity as a login entity
        ))
        .with_children(|parent| {

            // ======== Left side image =========
            
            parent.spawn( 
                
                ImageBundle {
                    style: Style {
                        max_height: Val::Percent(100.0),
                        min_height: Val::Percent(100.0),
                        max_width: Val::Percent(40.0),
                        ..default()
                    },
                    image: assets.hero_image.clone().into(),
                    ..default()
                }
            );

            // ======== Right side login panel =========

            parent.spawn( 
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        row_gap: Val::Px(20.0),
                        ..default()
                    },
                    ..default()
                }
            ).with_children(|parent| {
                // ==== Title ====
                parent.spawn(TextBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: "Log in".to_string(),
                                style: TextStyle {
                                    font: assets.anton.clone().into(),
                                    font_size: TITLE_FONT_SIZE_XLARGE,
                                    color: TITLE_COLOR.into(),
                                },
                            },
                        ],
                        ..default()
                    },
                    ..default()
                });

                // ==== User login Box ====
                parent.spawn(
                        NodeBundle {
                            style: Style {
                                width: Val::Px(400.0),
                                height: Val::Px(250.0),
                                flex_direction: FlexDirection::Column,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                row_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                    }
                ).with_children(|parent| {

                    // username and username input beside each other
                    parent.spawn(
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                column_gap: Val::Px(16.0),
                                ..default()
                            },
                            ..default()
                        }
                    ).with_children(|parent| {
                        // ==== USERNAME TEXT ====
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "Username".to_string(),
                                        style: TextStyle {
                                            font: assets.fira_sans_bold.clone().into(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: INPUT_FIELD_TITLE_COLOR.into(),
                                        },
                                    },
                                ],
                                ..default()
                            },
                            ..default()
                        });

                        // ==== USERNAME INPUT ====
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(250.0),
                                    height: Val::Px(40.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    padding: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                border_color: BORDER_COLOR_INACTIVE.into(),
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            TextInputBundle::default().with_text_style(
                                TextStyle {
                                    font: assets.fira_sans_bold.clone().into(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            LoginUsernameInputField("".to_string()) //marker
                        ));

                    });

                     // password and password input beside each other
                     parent.spawn(
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::FlexStart,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                column_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        }
                    ).with_children(|parent| {

                        // ==== PASSWORD TEXT ====
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "Password".to_string(),
                                        style: TextStyle {
                                            font: assets.fira_sans_bold.clone().into(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: INPUT_FIELD_TITLE_COLOR.into(),
                                        },
                                    },
                                ],
                                ..default()
                            },
                            ..default()
                        });

                        // ==== PASSWORD INPUT ====
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(250.0),
                                    height: Val::Px(40.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    padding: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                border_color: BORDER_COLOR_INACTIVE.into(),
                                background_color: Color::WHITE.into(),
                                ..default()
                            },
                            TextInputBundle::default().with_text_style(
                                TextStyle {
                                    font: assets.fira_sans_bold.clone().into(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            LoginPasswordInputField("".to_string()) //marker
                        ));
                    });

                    // ==== Error Message ====
                    parent.spawn(
                        (TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection {
                                            value: "FEEEL placeholder".to_string(),
                                            style: TextStyle {
                                                font: assets.fira_sans_bold.clone().into(),
                                                font_size: TITLE_FONT_SIZE_XSMALL,
                                                color: ERROR_COLOR.into(),
                                            },
                                        },
                                    ],
                                    ..default()
                                },
                                ..default()
                            },
                            LoginErrorMsg("Correct".to_string()), // mark this entity as an error message
                        )
                    );


                    // ==== BUTTONS
                parent.spawn(
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::End,
                            column_gap: Val::Px(20.0),
                            padding: UiRect::top(Val::Px(20.0)),
                            ..default()
                        },
                        ..default()
                    }
                ).with_children(|parent| {
                    parent.spawn(
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::End,
                                width: Val::Px(100.0),
                                ..default()
                            },
                            ..default()
                        }
                    );
                    parent.spawn(
                        (
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Px(100.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                background_color: BUTTON_COLOR.into(),
                                ..default()
                            },
                            LoginButton {}, // mark this entity as a login button
                        )
                    ).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Login", 
                                        TextStyle {
                                            font: assets.fira_sans_bold.clone().into(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: BUTTON_TEXT_COLOR.into(),
                                        }    
                                    )
                                ],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        });
                    
                    });

                    parent.spawn(
                        (
                            ButtonBundle {
                                style: Style {
                                    justify_content: JustifyContent::Center,
                                    align_items: AlignItems::Center,
                                    width: Val::Px(120.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                background_color: BUTTON_COLOR.into(),
                                ..default()
                            },
                            GoToRegisterButton {}, // mark this entity as a register button
                        )
                    ).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Register", 
                                        TextStyle {
                                            font: assets.fira_sans_bold.clone().into(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: BUTTON_TEXT_COLOR.into(),
                                        }    
                                    )
                                ],
                                justify: JustifyText::Center,
                                ..default()
                            },
                            ..default()
                        });
                    
                    });
                });

                    
                });


            });
        
        }
    ).id();
    login_entity
}


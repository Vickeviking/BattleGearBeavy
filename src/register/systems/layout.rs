use bevy::prelude::*;
use bevy_simple_text_input::TextInputBundle;
use bevy_round_ui::prelude::*;

use crate::components::MyAssets;
use crate::register::components::*;
use crate::register::styles::*;

pub fn spawn_register(mut commands: Commands, assets: Res<MyAssets>, mut materials: ResMut<Assets<RoundUiMaterial>>) {
    let register_entity = build_register(&mut commands, &assets, materials);
    info!("Register entity: {:?}", register_entity);
}

pub fn despawn_register(mut commands: Commands, register_query: Query<Entity, With<Register>>) {
    if let Ok(register_entity) = register_query.get_single() {
        commands.entity(register_entity).despawn_recursive(); // recursive to despawn all child ui nodes
    }
}



// not a system, a rust function that 
pub fn build_register(commands: &mut Commands, assets: &Res<MyAssets>, mut materials: ResMut<Assets<RoundUiMaterial>>) -> Entity {
    let register_entity = commands.spawn(
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
            Register {}, // mark this entity as a login entity
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
            
            ).insert(RegisterImageMarker);

            // ======== Right side login panel =========

            parent.spawn( 
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        row_gap: Val::Px(40.0),
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
                                value: "Register".to_string(),
                                style: TextStyle {
                                    font: assets.anton.clone(),
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
                                height: Val::Px(450.0),
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
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Stretch,
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
                                            font: assets.fira_sans_bold.clone(),
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
                                    font: assets.fira_sans_bold.clone(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            UsernameInputField("".to_string()) //marker
                        ));

                    });

                    // User email and email input beside each other
                    parent.spawn(
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                column_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        }
                    ).with_children(|parent| {
                            
                        // ==== EMAIL TEXT ====
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "Email".to_string(),
                                        style: TextStyle {
                                            font: assets.fira_sans_bold.clone(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: INPUT_FIELD_TITLE_COLOR.into(),
                                        },
                                    },
                                ],
                                ..default()
                            },
                            ..default()
                        });

                        // ==== EMAIL INPUT ====
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
                                    font: assets.fira_sans_bold.clone(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            EmailInputField("".to_string()) //marker
                        ));
                    });

                     // password and password input beside each other
                     parent.spawn(
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
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
                                            font: assets.fira_sans_bold.clone(),
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
                                    font: assets.fira_sans_bold.clone(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            PasswordInputField("".to_string()) //marker
                        ));
                    });

                    // ====  FullNameInputField ====

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                column_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        }
                    )).with_children(|parent| {
                        // ==== FULL NAME TEXT ====
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "Full Name".to_string(),
                                        style: TextStyle {
                                            font: assets.fira_sans_bold.clone(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: INPUT_FIELD_TITLE_COLOR.into(),
                                        },
                                    },
                                ],
                                ..default()
                            },
                            ..default()
                        });

                        // ==== FULL NAME INPUT ====
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
                                    font: assets.fira_sans_bold.clone(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            FullNameInputField("".to_string()) //marker
                        ));
                    });

                    // ====  DateOfBirthInputField ====

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                column_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        }
                    )).with_children(|parent| {
                        // ==== DATE OF BIRTH TEXT ====
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "Date of Birth".to_string(),
                                        style: TextStyle {
                                            font: assets.fira_sans_bold.clone(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: INPUT_FIELD_TITLE_COLOR.into(),
                                        },
                                    },
                                ],
                                ..default()
                            },
                            ..default()
                        });

                        // ==== DATE OF BIRTH INPUT ====
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(295.0),
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
                                    font: assets.fira_sans_bold.clone(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            DateOfBirthInputField("".to_string()) //marker
                        ));
                    });

                    // ====  CountryInputField ====

                    parent.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Row,
                                justify_content: JustifyContent::SpaceBetween,
                                align_items: AlignItems::Center,
                                width: Val::Percent(100.0),
                                height: Val::Px(50.0),
                                column_gap: Val::Px(20.0),
                                ..default()
                            },
                            ..default()
                        }
                    )).with_children(|parent| {
                        // ==== COUNTRY TEXT ====
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection {
                                        value: "Country".to_string(),
                                        style: TextStyle {
                                            font: assets.fira_sans_bold.clone(),
                                            font_size: TITLE_FONT_SIZE_MEDIUM,
                                            color: INPUT_FIELD_TITLE_COLOR.into(),
                                        },
                                    },
                                ],
                                ..default()
                            },
                            ..default()
                        });

                        // ==== COUNTRY INPUT ====
                        parent.spawn((
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(150.0),
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
                                    font: assets.fira_sans_bold.clone(),
                                    font_size: TITLE_FONT_SIZE_MEDIUM,
                                    color: INPUT_TEXT_COLOR.into(),
                                    ..default()
                                }
                            ),
                            CountryInputField("".to_string()) //marker
                        ));

                        parent.spawn(
                            NodeBundle {
                                style: Style {
                                    width: Val::Px(50.0),
                                    height: Val::Px(40.0),
                                    border: UiRect::all(Val::Px(2.0)),
                                    padding: UiRect::all(Val::Px(2.0)),
                                    ..default()
                                },
                                border_color: BORDER_COLOR_INACTIVE.into(),
                                background_color: BACK_GROUND_COLOR.into(),
                                ..default()
                            }
                        );
                    });

                    

                    // ==== Error Message ====
                    parent.spawn(
                        (TextBundle {
                                text: Text {
                                    sections: vec![
                                        TextSection {
                                            value: "FEEEL placeholder".to_string(),
                                            style: TextStyle {
                                                font: assets.fira_sans_bold.clone(),
                                                font_size: TITLE_FONT_SIZE_XSMALL,
                                                color: ERROR_COLOR.into(),
                                            },
                                        },
                                    ],
                                    ..default()
                                },
                                ..default()
                            },
                            RegisterErrorMsg("Correct placeholder".to_string()), // mark this entity as an error message
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
                                    width: Val::Px(120.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                background_color: BUTTON_COLOR.into(),
                                ..default()
                            },
                            RegisterButton {}, // mark this entity as a register button
                        )
                    ).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Register", 
                                        TextStyle {
                                            font: assets.fira_sans_bold.clone(),
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
                                    width: Val::Px(100.0),
                                    height: Val::Px(50.0),
                                    ..default()
                                },
                                background_color: BUTTON_COLOR.into(),
                                ..default()
                            },
                            GoToLoginButton {}, // mark this entity as a login button
                        )
                    ).with_children(|parent| {
                        parent.spawn(TextBundle {
                            text: Text {
                                sections: vec![
                                    TextSection::new(
                                        "Login", 
                                        TextStyle {
                                            font: assets.fira_sans_bold.clone(),
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
    register_entity
}


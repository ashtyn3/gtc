use bevy::prelude::*;
use bevy_mod_picking::prelude::{Click, On, Pickable, PickingInteraction, Pointer};

use crate::{
    components::{despawn_screen, GameState, MenuEntity},
    resources::Board,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), create_menu)
            .add_systems(
                Update,
                update_button_colors.run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnExit(GameState::Menu), despawn_screen::<MenuEntity>);
    }
}

pub fn new_game_btn(mut commands: Commands, mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Game);
}

pub fn load_game_btn(mut commands: Commands) {}
fn update_button_colors(
    mut buttons: Query<(Option<&PickingInteraction>, &mut BackgroundColor), With<Button>>,
) {
    for (interaction, mut button_color) in &mut buttons {
        *button_color = match interaction {
            Some(PickingInteraction::Pressed) => Color::rgb(0.35, 0.75, 0.35),
            Some(PickingInteraction::Hovered) => Color::rgb(0.25, 0.25, 0.25),
            Some(PickingInteraction::None) | None => Color::WHITE,
        }
        .into();
    }
}

pub fn create_menu(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            MenuEntity,
        ))
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: Color::OLIVE.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "Goats and Tigers",
                    TextStyle {
                        font_size: 80.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ));

                p.spawn((
                    ButtonBundle {
                        style: Style {
                            margin: UiRect::top(Val::Percent(5.)),
                            padding: UiRect::all(Val::Percent(1.0)),
                            ..default()
                        },
                        ..default()
                    },
                    On::<Pointer<Click>>::run(new_game_btn),
                ))
                .with_children(|p| {
                    p.spawn((
                        TextBundle::from_section(
                            "New Game",
                            TextStyle {
                                color: Color::BLACK,
                                font_size: 20.,
                                ..default()
                            },
                        ),
                        Pickable::IGNORE,
                    ));
                });
            });
        });
}

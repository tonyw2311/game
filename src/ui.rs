use bevy::prelude::*;

use crate::Money;
use crate::player::Player;
use crate::main_menu::GameState;

pub struct GameUI;

#[derive(Component)]
pub struct MoneyText;

#[derive(Component)]
pub struct PlayerText;

impl Plugin for GameUI {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_game_ui)
            .add_systems(Update, (update_money_ui,update_health_ui).run_if(in_state(GameState::Game)));
    }
}

fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(10.0),
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                //background_color: Color::BLUE.into(),
                ..default()
            },
            Name::new("UI Root"),
        ))
        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Money!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                MoneyText,
            ));
        })        .with_children(|commands| {
            commands.spawn((
                TextBundle {
                    text: Text::from_section(
                        "Health!",
                        TextStyle {
                            font_size: 32.0,
                            ..default()
                        },
                    ),
                    ..default()
                },
                PlayerText,
            ));
        });
}

fn update_money_ui(mut texts: Query<&mut Text, With<MoneyText>>, money: Res<Money>) {
    for mut text in &mut texts {
        text.sections[0].value = format!("Money: ${:?}\n", money.0);
    }
}

fn update_health_ui(mut texts: Query<&mut Text, With<PlayerText>>, player:Query<&Player>){
    let player = player.single();
    for mut text in &mut texts {
        text.sections[0].value = format!("Health: {:?} HP",player.health );
    }

}
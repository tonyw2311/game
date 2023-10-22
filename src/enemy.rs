use bevy::prelude::*;

use crate::{Money, Player};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy_parent)
            .add_systems(Update, (spawn_enemy, enemy_lifetime))
            .register_type::<Enemy>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
}

#[derive(Component)]
pub struct EnemyParent;

fn spawn_enemy_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        EnemyParent,
        Name::new("Enemy Parent"),
    ));
}

fn spawn_enemy(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<EnemyParent>>,
) {
    if !input.just_pressed(KeyCode::L) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    let texture = asset_server.load("triangle.png");

    commands.entity(parent).with_children(|commands| {
        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Enemy {
                health: 100.0,
                speed: 50.0,
            },
            Name::new("Enemy"),
        ));
    });
}

fn enemy_lifetime(
    mut commands: Commands,
    mut enemies: Query<(Entity, &mut Enemy)>,
    parent: Query<Entity, With<EnemyParent>>,
    mut money: ResMut<Money>,
) {
    let parent = parent.single();

    for (enemy_entity, enemy) in &mut enemies {

        if enemy.health <= 0.0 {
            money.0 += 15.0;

            commands.entity(parent).remove_children(&[enemy_entity]);
            commands.entity(enemy_entity).despawn();

            info!("enemy killed for $15! Current Money: ${:?}", money.0);
        }
    }
}

use bevy::prelude::*;
use rand::Rng;

use crate::{Money, Player};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy_parent)
            .add_systems(Update, enemy_lifetime)
            .register_type::<Enemy>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub collision_damage: f32,
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

/* fn spawn_enemy(
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
                speed: 10.0,
            },
            Name::new("Enemy"),
        ));
    });
} */

fn enemy_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut enemies: Query<(Entity, &mut Transform, &mut Enemy), Without<Player>>,
    player_transform: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<EnemyParent>>,
    mut money: ResMut<Money>,
    asset_server: Res<AssetServer>,
) {
    let parent = parent.single();
    let player_transform = player_transform.single();
    let mut rng = rand::thread_rng();

    for (enemy_entity, mut enemy_transform, enemy) in &mut enemies {
        if enemy.health <= 0.0 {
            money.0 += 15.0;

            let texture = asset_server.load("heart.png");
            let heart_transform = &mut enemy_transform.clone();
            heart_transform.scale = Vec3::splat(1.);

            if rng.gen_bool(0.5){
                commands.spawn(
                    SpriteBundle {
                        texture:asset_server.load("coin.png"),
                        transform: *enemy_transform,
                        ..default()
                    },
                );
            }
            else{
                commands.spawn(
                    SpriteBundle {
                        texture,
                        transform: *enemy_transform,
                        ..default()
                    },
                );
            }


            commands.entity(parent).remove_children(&[enemy_entity]);
            commands.entity(enemy_entity).despawn();
        }
        let movement_amount = enemy.speed
            * Vec3::normalize(player_transform.translation - enemy_transform.translation)
            * time.delta_seconds();
        enemy_transform.translation += movement_amount
    }
}

use bevy::prelude::*;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::enemy::Enemy;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (character_movement,character_collision))
            .register_type::<Player>();
    }
}

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
    pub health: f32,
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::W) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            transform.translation.x -= movement_amount;
        }
    }
}

fn character_collision(
    mut players: Query<(&mut Transform, &mut Player),Without<Enemy>>,
    mut enemies: Query<(&mut Transform, &mut Enemy),Without<Player>>,
) {
    for (player_transform, mut player) in players.iter_mut() {
        for (enemy_transform, mut enemy) in enemies.iter_mut() {
            let distance = enemy_transform
                .translation
                .distance(player_transform.translation);
            if distance < 10. {
                enemy.health -= 1.0;
                player.health -= enemy.collision_damage;
            }
        }
    }
}

/*
fn projectile_movement(
    mut projectiles: Query<(&mut Transform, &Projectile)>,
    input: Res<Input<KeyCode>>,
    time:Res<Time>,
){
    for(mut transform, projectile) in &mut projectiles{

    }
} */

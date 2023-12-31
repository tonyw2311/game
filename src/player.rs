use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::tilemap::{TileCollider, TILE_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, character_movement)
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
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();
        let mut y_del = 0.;
        let mut x_del = 0.;
        if input.pressed(KeyCode::W) {
            y_del += movement_amount;
        }
        if input.pressed(KeyCode::S) {
            y_del -= movement_amount;
        }
        if input.pressed(KeyCode::D) {
            x_del += movement_amount;
        }
        if input.pressed(KeyCode::A) {
            x_del -= movement_amount;
        }
        let target = transform.translation + Vec3::new(x_del, 0.0, 0.0);
        if wall_collision_check(target, &wall_query) {
            transform.translation = target;
        }
        let target = transform.translation + Vec3::new(0.0, y_del, 0.0);
        if wall_collision_check(target, &wall_query) {
            transform.translation = target;
        }
    }
}




pub fn wall_collision_check(
    target_player_pos: Vec3,
    wall_query: &Query<&Transform, (With<TileCollider>, Without<Player>)>,
) -> bool {
    for wall_transform in wall_query.iter() {
        let collision = collide(
            target_player_pos,
            Vec2::splat(TILE_SIZE * 0.9),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collision.is_some() {
            return false;
        }
    }
    true
}

use crate::main_menu::GameState;
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_inspector_egui::InspectorOptions;

use crate::map_gen::RoomTag;
use crate::tilemap::{TileCollider, TILE_SIZE};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (character_movement, room_enter).run_if(in_state(GameState::Game)),
        )
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
    mut players: Query<(&mut Transform, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>, Without<TileCollider>)>,
) {
    let (mut player_transform, player) = players.single_mut();
    let mut camera_transform = camera.single_mut();

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
    let target = player_transform.translation + Vec3::new(x_del, 0.0, 0.0);
    if wall_collision_check(target, &wall_query) {
        player_transform.translation = target;
        camera_transform.translation = target;
    }
    let target = player_transform.translation + Vec3::new(0.0, y_del, 0.0);
    if wall_collision_check(target, &wall_query) {
        player_transform.translation = target;
        camera_transform.translation = target;
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

pub fn room_enter(
    mut commands: Commands,
    players: Query<&Transform, With<Player>>,
    rooms: Query<(Entity, &Transform, &RoomTag), Without<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    let player = players.single();
    for (room_entity, room_transform, room) in rooms.iter() {
        
        if is_inside_room(
            player.translation,
            room_transform.translation,
            room.width,
            room.height,
        ) {
            commands.entity(room_entity).insert(materials.add(ColorMaterial::from(Color::LIME_GREEN)));
            println!("Entered the room! Transform: {:?}", room_transform);
        }
        else{
            commands.entity(room_entity).insert(materials.add(ColorMaterial::from(Color::BLACK)));
        }
    }
}

fn is_inside_room(
    player_location: Vec3,
    room_location: Vec3,
    room_width: f32,
    room_height: f32,
) -> bool {
    // Implement logic to check if the player is inside the room
    // For simplicity, let's assume a rectangular room and check if player_location is within its bounds
    let room_size = Vec2::new(room_width, room_height); // Adjust the size of the room as needed

    let room_min = room_location.xy() - room_size / 2.0;
    let room_max = room_location.xy() + room_size / 2.0;

    player_location.x > room_min.x
        && player_location.x < room_max.x
        && player_location.y > room_min.y
        && player_location.y < room_max.y
}

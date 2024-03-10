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
    mut rooms: Query<(Entity, &mut Transform, &RoomTag), Without<Player>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<AssetServer>,
    mut prev_room_pos: Local<Option<Vec3>>,
) {
    let player = players.single();
    for (room_entity, mut room_transform, room) in rooms.iter_mut() {
        if is_inside_room(
            player.translation,
            room_transform.translation,
            room.width,
            room.height,
        ) {
            if let Some(prev_pos) = prev_room_pos.as_ref() {
                if *prev_pos != room_transform.translation {
                    create_walls(&mut commands, &assets, room.width, room.height,room_transform.translation);
                    create_doorway(&mut commands, &assets, room.width, room.height, room_transform.translation);
                    room_transform.translation.z = -30.;
                    println!(
                        "Entered the room! Transform: {:?}",
                        room_transform.translation
                    );
                } 
            } else {
                // This is the first room the player enters
                create_walls(&mut commands, &assets, room.width, room.height, room_transform.translation);
                create_doorway(&mut commands, &assets, room.width, room.height, room_transform.translation);
                room_transform.translation.z = -30.;
                println!(
                    "Entered the room for the first time! Transform: {:?}",
                    room_transform.translation
                );
            }
            *prev_room_pos = Some(room_transform.translation);
        }
        else {
            room_transform.translation.z = 5.;
            commands
                .entity(room_entity)
                .insert(materials.add(ColorMaterial::from(Color::BLACK)));
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

fn create_walls(commands: &mut Commands, assets: &Res<AssetServer>, width: f32, height: f32, room_location: Vec3) {
    let room_size = Vec2::new(width, height);
    let room_min = room_location.xy() - room_size / 2.0;
    let room_max = room_location.xy() + room_size / 2.0;
    let mut x_wall = room_min.x;
    let mut y_wall = room_min.y;

    while x_wall < room_max.x {
        let texture = assets.load("wall.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(x_wall, room_max.y, 10.0),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            TileCollider,
        ));

        let texture = assets.load("wall.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(x_wall, room_min.y, 10.0),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            TileCollider,
        ));

        x_wall += TILE_SIZE
    }
    while y_wall < room_max.y {
        let texture = assets.load("wall.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(room_max.x , y_wall, 10.0),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            TileCollider,
        ));

        let texture = assets.load("wall.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(room_min.x, y_wall, 10.0),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            TileCollider,
        ));

        y_wall += TILE_SIZE
    }
}

fn create_doorway(commands: &mut Commands, assets: &Res<AssetServer>, width: f32, height: f32, room_location: Vec3) {
    let texture = assets.load("sand.png");
    let room_size = Vec2::new(width, height);
    let room_min = room_location.xy() - room_size / 2.0;
    let room_max = room_location.xy() + room_size / 2.0;
    
    // Determine the position of the doorway
    let doorway_position = Vec3::new(room_max.x, room_location.y, 10.0); // Adjust the position as needed
    
    // Spawn the doorway sprite
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            translation: doorway_position,
            scale: Vec3::splat(1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

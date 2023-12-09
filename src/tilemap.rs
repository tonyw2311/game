

use bevy::{prelude::*, window::PrimaryWindow};
use crate::main_menu::GameState;


pub const TILE_SIZE: f32 = 8.;
pub struct TileMapPlugin;

#[derive(Component)]
pub struct TileCollider;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), create_simple_map);
    }
}

fn create_simple_map(
    mut commands: Commands,
    assets: Res<AssetServer>,
    primary_query: Query<&Window, With<PrimaryWindow>>,
) {
    let primary = primary_query.single();
    println!("{}", primary.resolution.width() / 2.);
    println!("{}", primary.resolution.height());

    let mut x = -primary.resolution.width() / 2.;
    let mut y = -primary.resolution.height() / 2.;

    while x < primary.resolution.width() / 2. {
        y += 32.;

        while y < primary.resolution.height() / 2. {
            let texture = assets.load("brick.png");
            commands.spawn(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(x, y, -100.0),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                },
                ..Default::default()
            });
            y += 32.
        }
        y = -primary.resolution.height() / 2.;
        x += 32.
    }

    let mut x_wall = -primary.width() / 2.;
    let mut y_wall = -primary.height() / 2.;

    while x_wall < primary.width() / 2. {
        let texture = assets.load("wall.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(x_wall, primary.resolution.height() / 10. - 4., 10.0),
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
                    translation: Vec3::new(x_wall, -primary.resolution.height() / 10. + 4., 10.0),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                },
                ..Default::default()
            },
            TileCollider,
        ));

        x_wall += TILE_SIZE
    }
    while y_wall < primary.resolution.height() / 2. {
        let texture = assets.load("wall.png");
        commands.spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(
                        primary.resolution.width() / 10. - TILE_SIZE / 2.,
                        y_wall,
                        10.0,
                    ),
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
                    translation: Vec3::new(
                        -primary.resolution.width() / 10. + TILE_SIZE / 2.,
                        y_wall,
                        10.0,
                    ),
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

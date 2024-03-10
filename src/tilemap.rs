

use bevy::prelude::*;
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
) {
    let size_x = 3000.;
    let size_y= 3000.;
    let mut x = -size_x / 2.;
    let mut y = -size_y/ 2.;

    while x < size_x / 2. {
        y += 32.;

        while y < size_y / 2. {
            let texture = assets.load("brick.png");
            commands.spawn(SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(x, y, -20.0),
                    scale: Vec3::splat(1.),
                    ..Default::default()
                },
                ..Default::default()
            });
            y += 32.
        }
        y = -size_y / 2.;
        x += 32.
    }}
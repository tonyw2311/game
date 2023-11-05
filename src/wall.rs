use bevy::prelude::*;


pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<Wall>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Wall {
}


pub fn spawn_wall(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    translation: Vec3,
) -> Entity {

    let texture = assets.load("wall.png");
    commands.spawn(SpriteBundle {
        texture,
        transform: Transform {
            translation,
            ..Default::default()
        },
        ..Default::default()
    }).id()
}

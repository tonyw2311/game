use crate::player::Player;
use bevy::{prelude::*, render::camera::ScalingMode, sprite::MaterialMesh2dBundle};
use drops::DropsPlugin;
use enemy::EnemyPlugin;
use enemy_spawner::EnemySpawnerPlugin;
use pig::PigPlugin;
use player::PlayerPlugin;
use projectile::ProjectilePlugin;
use tilemap::TileMapPlugin;

use ui::GameUI;

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

mod drops;
mod enemy;
mod enemy_spawner;
mod pig;
mod player;
mod projectile;
mod tilemap;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Shape Battle".into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        /*         .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        ) */
        .insert_resource(ClearColor(Color::rgb(0.9, 0.3, 0.6)))
        .insert_resource(Money(100.0))
        .register_type::<Money>()
        .add_plugins((
            PigPlugin,
            GameUI,
            ProjectilePlugin,
            EnemyPlugin,
            PlayerPlugin,
            EnemySpawnerPlugin,
            DropsPlugin,
            TileMapPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(4.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::DARK_GRAY)),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Player {
            speed: 50.0,
            health: 200.0,
        },
        Name::new("Player"),
    ));

    commands.spawn(enemy_spawner::EnemySpawner {
        cooldown: 1.,
        timer: 1.,
    });
}



use bevy::{prelude::*, sprite::MaterialMesh2dBundle, window::PrimaryWindow};
use rand::Rng;

use crate::enemy::{Enemy,EnemyParent};
use crate::main_menu::GameState;


pub struct EnemySpawnerPlugin;

impl Plugin for EnemySpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_spawning.run_if(in_state(GameState::Game))).
            register_type::<EnemySpawner>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct EnemySpawner {
    pub cooldown: f32,
    pub timer: f32,
}

pub fn update_spawning(
    primary_query: Query<&Window, With<PrimaryWindow>>,
    mut spawner_query: Query<&mut EnemySpawner>,
    time: Res<Time>,
    mut commands: Commands,
    parent: Query<Entity, With<EnemyParent>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let parent = parent.single();

    for mut spawner in spawner_query.iter_mut() {
        spawner.timer -= time.delta_seconds();
        if spawner.timer <= 0. {
            let Ok(primary) = primary_query.get_single() else {
                return;
            };
            spawner.timer = spawner.cooldown;

            let mut spawn_transform = Transform::from_scale(Vec3::splat(5.));

            let mut rng = rand::thread_rng();
/*             let texture;
            if rng.gen_bool(0.5){
                texture = asset_server.load("triangle.png");
            }
            else{
                texture = asset_server.load("square.png");
                
            } */
            let side = rng.gen_range(3..8);
            let mult = side as f32;

            spawn_transform.translation = Vec3::new(rng.gen_range(-primary.height()/2.0..primary.height()/2.),rng.gen_range(-primary.width()/2.0..primary.width()/2.0),0.);
            let radius = rng.gen_range(1.0..3.0);
            spawn_transform.scale = Vec3::splat(radius);

            commands.entity(parent).with_children(|commands| {
                commands.spawn((
                    MaterialMesh2dBundle {
                        mesh: meshes.add(shape::RegularPolygon::new(3.,side).into()).into(),
                        material: materials.add(ColorMaterial::from(Color::PURPLE)),
                        transform: spawn_transform,
                        ..default()
                    },
                    Enemy {
                        health: 25.*mult,
                        speed: 20.0,//+ 20.*(time.elapsed_seconds()/60.).floor(),
                        collision_damage:1.,
                        radius:radius*5.       
                    },
                    Name::new("Enemy"),
                ));
            });
        }
    }
}

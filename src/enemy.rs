use rand::seq::SliceRandom;
use rand::Rng;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::main_menu::GameState;
use crate::{
    drops::{Drops, DropsParent},
    Player,
};

use crate::player::wall_collision_check;
use crate::tilemap::TileCollider;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_enemy_parent)
            .add_systems(Update, enemy_lifetime.run_if(in_state(GameState::Game)))
            .register_type::<Enemy>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Enemy {
    pub health: f32,
    pub speed: f32,
    pub collision_damage: f32,
    pub radius: f32,
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
    mut enemies: Query<(Entity, &mut Transform, &mut Enemy),( Without<Player>,Without<TileCollider>)>,
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>,Without<TileCollider>)>,
    parent: Query<Entity, With<EnemyParent>>,
    drops_parent: Query<Entity, With<DropsParent>>,
    asset_server: Res<AssetServer>,
    wall_query: Query<&Transform, (With<TileCollider>, Without<Player>)>,
) {
    let parent = parent.single();
    let drops_parent = drops_parent.single();
    let (mut player_transform, mut player) = player_query.single_mut();
    let mut rng = rand::thread_rng();

    for (enemy_entity, mut enemy_transform, mut enemy) in &mut enemies {
        if enemy.health <= 0.0 {
            let transform = &mut enemy_transform.clone();
            transform.translation.z = -1.0;
            transform.scale = Vec3::splat(1.);
            if rng.gen_bool(1.) {
                let drop_arr = ["health", "coin", "damage_up"];
                let drop = drop_arr.choose(&mut rand::thread_rng()).unwrap();
                let sprite = Sprite {
                    custom_size: Some(Vec2::splat(7.)),
                    ..Default::default()
                };

                commands.entity(drops_parent).with_children(|commands| {
                    commands.spawn((
                        SpriteBundle {
                            sprite,
                            texture: asset_server.load(format!("{}.png", drop)),
                            transform: *transform,

                            ..default()
                        },
                        Drops {
                            drop_type: format!("{}", drop),
                        },
                        Name::new(format!("{}", drop)),
                    ));
                });
            }
            commands.entity(parent).remove_children(&[enemy_entity]);
            commands.entity(enemy_entity).despawn();
        }

        let mut movement_amount = enemy.speed
            * Vec3::normalize(player_transform.translation - enemy_transform.translation)
            * time.delta_seconds();

        if player_collision(
            player_transform.translation,
            enemy_transform.translation + movement_amount,
            enemy.radius,
        ) {
            movement_amount = movement_amount/enemy.speed *5.;
            let movement_x = Vec3::new(movement_amount.x, 0., 0.);
            let movement_y = Vec3::new(0., movement_amount.y, 0.);
            if wall_collision_check(movement_x + player_transform.translation, &wall_query) {

                player_transform.translation += movement_x;
            }
            if wall_collision_check(movement_y + player_transform.translation, &wall_query){
                player_transform.translation += movement_y;
            }

            enemy.health -= 1.0;
            player.health -= enemy.collision_damage;
        } else {
            enemy_transform.translation += movement_amount
        }
    }
}

fn player_collision(target_player: Vec3, target_enemy: Vec3, enemy_radius: f32) -> bool {
    let collision = collide(
        target_player,
        Vec2::splat(enemy_radius),
        target_enemy,
        Vec2::splat(5.),
    );
    if collision.is_some() {
        return true;
    }
    false
}


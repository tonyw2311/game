use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::Player;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_projectile_parent)
            .add_systems(
                Update,
                (create_projectile, projectile_lifetime, projectile_collision),
            )
            .register_type::<Projectile>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]

pub struct Projectile {
    pub lifetime: Timer,
    pub speed: f32,
    pub direction: Vec2,
}

#[derive(Component)]
pub struct ProjectileParent;

fn create_projectile_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        ProjectileParent,
        Name::new("Projectile Parent"),
    ));
}

fn create_projectile(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<ProjectileParent>>,
) {
    if !input.any_just_pressed([KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right]) {
        return;
    }
    if !input.any_just_pressed([KeyCode::Up, KeyCode::Down, KeyCode::Left, KeyCode::Right]) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();
    let texture = asset_server.load("bullet2.png");
    let mut dir = Vec2::ZERO;
    if input.pressed(KeyCode::Up) {
        dir = Vec2::Y;
    }
    if input.pressed(KeyCode::Down) {
        dir = Vec2::NEG_Y;
    }
    if input.pressed(KeyCode::Left) {
        dir = Vec2::NEG_X;
    }
    if input.pressed(KeyCode::Right) {
        dir = Vec2::X;
    }

    commands.entity(parent).with_children(|commands| {
        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..default()
            },
            Projectile {
                lifetime: Timer::from_seconds(4.0, TimerMode::Once),
                speed: 200.0,
                direction: dir,
            },
            Name::new("Bullet"),
        ));
    });
}

fn projectile_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut projectiles: Query<(&mut Transform, Entity, &mut Projectile)>,
    parent: Query<Entity, With<ProjectileParent>>,
) {
    let parent = parent.single();

    for (mut projectile_transform, projectile_entity, mut projectile) in &mut projectiles {
        projectile.lifetime.tick(time.delta());

        let movement_amount = projectile.speed * projectile.direction * time.delta_seconds();
        projectile_transform.translation += Vec3::new(movement_amount.x, movement_amount.y, 0.);

        /*         let mut movement_amount = projectile.speed * projectile.lifetime.tick(time.delta());
        if input.pressed(KeyCode::Up) {
            projectile_entity.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::Down) {
            projectile_entity.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::Left) {
            projectile_entity.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::Right) {
            projectile_entity.translation.x -= movement_amount;
        } */


        if projectile.lifetime.finished() {
            commands
                .entity(parent)
                .remove_children(&[projectile_entity]);
            commands.entity(projectile_entity).despawn();
        }
    }
}

fn projectile_collision(
    mut commands: Commands,
    mut enemies: Query<(&mut Transform, &mut Enemy), Without<Projectile>>,
    projectiles: Query<(&mut Transform, Entity), With<Projectile>>,
    parent: Query<Entity, With<ProjectileParent>>,
) {
    let parent = parent.single();
    for (enemy_transform, mut enemy) in &mut enemies {
        for (projectile_transform, projectile_entity) in projectiles.iter() {
            let distance = enemy_transform
                .translation
                .distance(projectile_transform.translation);
            if distance < 10. {
                println!("Enemy hit player! Game Over!");
                enemy.health -= 25.0;
                commands
                    .entity(parent)
                    .remove_children(&[projectile_entity]);
                commands.entity(projectile_entity).despawn();
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

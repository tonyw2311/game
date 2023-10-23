use bevy::prelude::*;
use rand::Rng;

use crate::{Money, Player};

pub struct DropsPlugin;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy_parent)
            .add_systems(Update, enemy_lifetime)
            .register_type::<Drops>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Drops {
    pub drop_type: String,
}

#[derive(Component)]
pub struct DropsParent;

fn spawn_enemy_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        DropsParent,
        Name::new("Drops Parent"),
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
    mut enemies: Query<(Entity, &mut Transform, &mut Drops), Without<Player>>,
    player_transform: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<DropsParent>>,
    mut money: ResMut<Money>,
    asset_server: Res<AssetServer>,
) {
    let parent = parent.single();
    let player_transform = player_transform.single();
    let mut rng = rand::thread_rng();

  
    }


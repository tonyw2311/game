use bevy::prelude::*;

use crate::{Money, Player};


pub struct DropsPlugin;
use crate::main_menu::GameState;

impl Plugin for DropsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_drops_parent)
            .add_systems(Update, drops_lifetime.run_if(in_state(GameState::Game)))
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

fn spawn_drops_parent(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        DropsParent,
        Name::new("Drops Parent"),
    ));
}

fn drops_lifetime(
    mut commands: Commands,
    mut drops: Query<(Entity, &mut Transform, &mut Drops), Without<Player>>,
    mut player_transform: Query<(&mut Transform, &mut Player), With<Player>>,
    parent: Query<Entity, With<DropsParent>>,
    mut money: ResMut<Money>,
) {
    let parent = parent.single();
    let (player_transform,  mut player) = player_transform.single_mut();

    for (drop_entity, drop_transform, drop) in &mut drops {
        let distance = drop_transform
            .translation
            .distance(player_transform.translation);
        if distance < 10. {
            if drop.drop_type== "COIN"{
                money.0 += 10.;
            }
            else if drop.drop_type=="health" {
                player.health += 10.;
            }
            commands.entity(parent).remove_children(&[drop_entity]);
            commands.entity(drop_entity).despawn();

        }

    }
}

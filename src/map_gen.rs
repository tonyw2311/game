use bevy::{
    prelude::*, render::render_resource::encase::ArrayLength, sprite::MaterialMesh2dBundle,
};
use rand::Rng;
pub const MIN_LEAF_SIZE: f32 = 200.;
pub const MAX_LEAF_SIZE: f32 = 600.;
use crate::tilemap::{TileCollider, TILE_SIZE};

pub struct MapGenPlugin;
impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_level)
            .register_type::<Leaf>();
    }
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Leaf {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    child_split: bool,
}

#[derive(Component)]
pub struct RoomTag {
    pub width: f32,
    pub height: f32,
    pub is_left_edge: bool,
    pub is_right_edge: bool,
    pub is_top_edge: bool,
    pub is_bottom_edge: bool,
    pub doors: Vec<Vec2>,
}

impl Leaf {
    fn split(&mut self) -> Option<(Leaf, Leaf)> {
        let mut rng = rand::thread_rng();

        if self.child_split {
            return None;
        }

        let max: f32;
        let mut split_b: bool = rng.gen_bool(0.5);
        if (self.width > self.height) && (self.width / self.height >= 1.25) {
            max = self.width - MIN_LEAF_SIZE;
            split_b = false;
        } else if (self.height > self.width) && (self.height / self.width >= 1.25) {
            max = self.height - MIN_LEAF_SIZE;
            split_b = true;
        } else {
            max = if split_b {
                self.height - MIN_LEAF_SIZE
            } else {
                self.width - MIN_LEAF_SIZE
            }
        }
        if max <= MIN_LEAF_SIZE {
            return None;
        }

        let split = rng.gen_range(MIN_LEAF_SIZE..max);
        let left_child;
        let right_child;
        /*
        divide by height
        */
        if split_b {
            left_child = Leaf {
                x: self.x,
                y: self.y,
                width: self.width,
                height: split,
                child_split: false,
            };
            right_child = Leaf {
                x: self.x,
                y: self.y + split,
                width: self.width,
                height: self.height - split,
                child_split: false,
            };

        /*
        divide by width
        */
        } else {
            left_child = Leaf {
                x: self.x,
                y: self.y,
                width: split,
                height: self.height,
                child_split: false,
            };
            right_child = Leaf {
                x: self.x + split,
                y: self.y,
                width: self.width - split,
                height: self.height,
                child_split: false,
            };
        }
        self.child_split = true;
        return Some((left_child, right_child));
    }
}

fn start_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut _leaf: Vec<Leaf> = vec![];
    let weight = 2.;
    let root = Leaf {
        x: -640. * weight,
        y: -360. * weight,
        width: 1280. * weight,
        height: 720. * weight,
        child_split: false,
    };
    _leaf.push(root);
    let mut fully_split = false;
    while !fully_split {
        fully_split = true;
        let mut temp_leaf: Vec<Leaf> = vec![];
        for l in _leaf.iter_mut() {
            if l.child_split {
                continue;
            }
            if l.width > MAX_LEAF_SIZE || l.height > MAX_LEAF_SIZE {
                let leaves = l.split();
                match leaves {
                    Some(leaf) => {
                        temp_leaf.push(leaf.0);
                        temp_leaf.push(leaf.1);
                        fully_split = false;
                    }
                    None => println!("Not a thing"),
                }
            }
        }
        _leaf.append(&mut temp_leaf)
    }

    let mut rng = rand::thread_rng();

    // Inside your main loop or function where you iterate over leaves
    for (i, l) in _leaf.iter().enumerate() {
        if !l.child_split {
            let room_min = Vec2::new(l.x, l.y);
            let room_max = Vec2::new(l.x + l.width, l.y + l.height);
            let is_left_edge = room_min.x == l.x;
            let is_right_edge = room_max.x == -l.x;
            let is_bottom_edge = room_min.y == l.y;
            let is_top_edge = room_max.y == -l.y;
            /*             println!("Room Min: {}", room_min);
            println!("Room Max: {}", room_max);
            println!(); */
            let spawn_transform = Transform {
                translation: Vec3::new(l.x + l.width / 2., l.y + l.height / 2., -10.),
                ..Default::default()
            };
            let mut doors = Vec::new();

            // Check if this leaf shares an edge with any other leaf
            for other_leaf in _leaf.iter().skip(i + 1) {
                // Check if the two leaves share an edge and calculate door positions if needed
                doors.extend(calculate_door_positions(&l, &other_leaf));
            }

            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: meshes
                        .add(shape::Quad::new(Vec2::new(l.width, l.height)).into())
                        .into(),
                    material: materials.add(ColorMaterial::from(Color::rgb(
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                        rng.gen_range(0.0..1.0),
                    ))),
                    transform: spawn_transform,
                    ..default()
                },
                RoomTag {
                    width: l.width,
                    height: l.height,
                    is_left_edge,
                    is_right_edge,
                    is_top_edge,
                    is_bottom_edge,
                    doors, // Add the calculated doors to the RoomTag
                },
            ));
        }
    }
}

// Define a function to calculate door positions for leaves sharing the same edge
fn calculate_door_positions(leaf: &Leaf, other_leaf: &Leaf) -> Vec<Vec2> {
    let mut doors = Vec::new();
    // Calculate the boundaries of both leaves
    let leaf_left_x = leaf.x;
    let leaf_right_x = leaf.x + leaf.width;
    let other_leaf_left_x = other_leaf.x;
    let other_leaf_right_x = other_leaf.x + other_leaf.width;

    let leaf_bottom_y = leaf.y;
    let leaf_top_y = leaf.y + leaf.height;
    let other_leaf_bottom_y = other_leaf.y;
    let other_leaf_top_y = other_leaf.y + leaf.height;

// Tolerance for overlap
let tolerance = 1.0;

// Check for overlap and calculate door position if the rooms share the top edge
if leaf_top_y >= other_leaf_bottom_y - tolerance
    && leaf_top_y <= other_leaf_top_y + tolerance
    && (leaf_left_x..=leaf_right_x).contains(&other_leaf_left_x)
{
    let door_position = Vec2::new((leaf_right_x + other_leaf_left_x) / 2.0, leaf_top_y);
    doors.push(door_position);
}

// Check for overlap and calculate door position if the rooms share the right edge
if leaf_right_x >= other_leaf_left_x - tolerance
    && leaf_right_x <= other_leaf_right_x + tolerance
    && (leaf_bottom_y..=leaf_top_y).contains(&other_leaf_bottom_y)
{
    let door_position = Vec2::new(leaf_right_x, (leaf_top_y + other_leaf_bottom_y) / 2.0);
    doors.push(door_position);
}

// Check for overlap and calculate door position if the rooms share the bottom edge
if leaf_bottom_y >= other_leaf_top_y - tolerance
    && leaf_bottom_y <= other_leaf_bottom_y + tolerance
    && (leaf_left_x..=leaf_right_x).contains(&other_leaf_left_x)
{
    let door_position = Vec2::new((leaf_right_x + other_leaf_left_x) / 2.0, leaf_bottom_y);
    doors.push(door_position);
}

// Check for overlap and calculate door position if the rooms share the left edge
if leaf_left_x >= other_leaf_right_x - tolerance
    && leaf_left_x <= other_leaf_left_x + tolerance
    && (leaf_bottom_y..=leaf_top_y).contains(&other_leaf_bottom_y)
{
    let door_position = Vec2::new(leaf_left_x, (leaf_top_y + other_leaf_bottom_y) / 2.0);
    doors.push(door_position);
}


    // Repeat similar checks for other edges if needed
    doors
}


fn create_walls_and_doorway(
    commands: &mut Commands,
    assets: &Res<AssetServer>,
    width: f32,
    height: f32,
    room_location: Vec3,
    door_locations: &Vec<Vec2>,
) {
    let room_size = Vec2::new(width, height);
    let room_min = room_location.xy() - room_size / 2.0;
    let room_max = room_location.xy() + room_size / 2.0;
    let mut x_wall = room_min.x;
    let mut y_wall = room_min.y;

    while x_wall < room_max.x - TILE_SIZE {
        let mut skip_wall = false;
        for door_pos in door_locations.iter() {
            if (x_wall + TILE_SIZE > door_pos.x) && (x_wall < door_pos.x + TILE_SIZE) {
                skip_wall = true;
                break;
            }
        }

        if skip_wall {
            x_wall += TILE_SIZE;
            continue; // Skip spawning wall segments under the doorway
        }

        let texture = assets.load("wall.png");
        commands.spawn(SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(x_wall, room_max.y, 10.0),
                scale: Vec3::splat(1.),
                ..Default::default()
            },
            ..Default::default()
        });

        let texture = assets.load("wall.png");
        commands.spawn(SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(x_wall, room_min.y, 10.0),
                scale: Vec3::splat(1.),
                ..Default::default()
            },
            ..Default::default()
        });

        x_wall += TILE_SIZE;
    }

    while y_wall < room_max.y {
        let mut skip_wall = false;
        for door_pos in door_locations.iter() {
            if (y_wall + TILE_SIZE > door_pos.y) && (y_wall < door_pos.y + TILE_SIZE) {
                skip_wall = true;
                break;
            }
        }

        if skip_wall {
            y_wall += TILE_SIZE;
            continue; // Skip spawning wall segments under the doorway
        }

        let texture = assets.load("wall.png");
        commands.spawn(SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(room_max.x, y_wall, 10.0),
                scale: Vec3::splat(1.),
                ..Default::default()
            },
            ..Default::default()
        });

        let texture = assets.load("wall.png");
        commands.spawn(SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(room_min.x, y_wall, 10.0),
                scale: Vec3::splat(1.),
                ..Default::default()
            },
            ..Default::default()
        });

        y_wall += TILE_SIZE;
    }

    // Spawn the doorway sprite
    for door_location in door_locations {
        let doorway_texture = assets.load("sand.png");

        commands.spawn(SpriteBundle {
            texture: doorway_texture,
            transform: Transform {
                translation:  Vec3::new(door_location.x,door_location.y,10.),
                scale: Vec3::splat(1.),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

use bevy::prelude::*;
use rand::Rng;
pub const MIN_LEAF_SIZE: f32 = 10.;
pub const MAX_LEAF_SIZE: f32 = 35.;
pub struct MapGenPlugin;
impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_level)
            .add_systems(Update, system)
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

impl Leaf {
    fn split(&mut self) -> Option<(Leaf, Leaf)> {
        let mut rng = rand::thread_rng();

        if self.child_split {
            return None;
        }

        let mut max: f32 = self.height - MIN_LEAF_SIZE;
        let mut split_b: bool = false;
        if (self.width > self.height) && (self.width / self.height >= 1.25) {
            max = self.width - MIN_LEAF_SIZE;
        } else if (self.height > self.width) && (self.height / self.width >= 1.25) {
            max = self.height - MIN_LEAF_SIZE;
            split_b = true;
        }

        if max <= MIN_LEAF_SIZE {
            return None;
        }

        let split = rng.gen_range(MIN_LEAF_SIZE..max);
        let left_child;
        let right_child;
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

fn start_level(mut gizmos: Gizmos, mut commands: Commands, assets: Res<AssetServer>) {
    let mut _leaf: Vec<Leaf> = vec![];
    let root = Leaf {
        x: 0.,
        y: 0.,
        width: 100.,
        height: 100.,
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

    for l in _leaf.iter() {
        let mut i = l.x;
        let mut j = l.y;

        while i < (l.x + l.width) {
            i += 3.;
            let texture = assets.load("square.png");
            commands.spawn((SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(i, l.y, 10.0),
                    scale: Vec3::splat(0.5),
                    ..Default::default()
                },
                ..Default::default()
            },));
            let texture = assets.load("square.png");
            commands.spawn((SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(i, l.y+l.height, 10.0),
                    scale: Vec3::splat(0.5),
                    ..Default::default()
                },
                ..Default::default()
            },));

        }
        while j < (l.y + l.height) {
            j += 3.;
            let texture = assets.load("square.png");
            commands.spawn((SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(l.x, j, 10.0),
                    scale: Vec3::splat(0.5),
                    ..Default::default()
                },
                ..Default::default()
            },));
            let texture = assets.load("square.png");
            commands.spawn((SpriteBundle {
                texture,
                transform: Transform {
                    translation: Vec3::new(l.x+l.width, j, 10.0),
                    scale: Vec3::splat(0.5),
                    ..Default::default()
                },
                ..Default::default()
            },));

        }

        gizmos.rect_2d(
            Vec2::new(l.x, l.y),
            0.,
            Vec2::new(l.width, l.height),
            Color::BLACK,
        );
        print!("{} ,", l.x);
        println!("{}", l.y);
    }
}

fn system(mut gizmos: Gizmos) {
    gizmos.rect_2d(Vec2::ZERO, 2., Vec2::splat(10.), Color::RED);

}

fn create_rooms(){
    
}

/* fn split_leaf(leaf: Leaf) -> bool {
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.5) {}

    let mut max: f32 = 0.;
    let mut split_b: bool = false;
    if (leaf.width > leaf.height) && (leaf.width / leaf.height >= 1.25) {
        max = leaf.width - MIN_LEAF_SIZE
    } else if (leaf.height > leaf.width) && (leaf.height / leaf.width >= 1.25) {
        max = leaf.height - MIN_LEAF_SIZE;
        split_b = true
    }
    if max <= MIN_LEAF_SIZE {
        return false;
    }
    let split = rng.gen_range(MIN_LEAF_SIZE..max);
    if split_b {
        let leftChild = Leaf {
            x: leaf.x,
            y: leaf.y,
            width: leaf.width,
            height: split,
            child_split: false,
            right_child_split: false,
        };
        let rightChild = Leaf {
            x: leaf.x,
            y: leaf.y + split,
            width: leaf.width,
            height: leaf.height - split,
            child_split: false,
            right_child_split: false,
        };
    } else {
        let leftChild = Leaf {
            x: leaf.x,
            y: leaf.y,
            width: split,
            height: leaf.height,
            child_split: false,
            right_child_split: false,
        };
        let rightChild = Leaf {
            x: leaf.x + split,
            y: leaf.y,
            width: leaf.width - split,
            height: leaf.height,
            child_split: false,
            right_child_split: false,
        };
    }
    return true;
}
 */

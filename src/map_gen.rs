use bevy::prelude::*;
use rand::Rng;
pub const MIN_LEAF_SIZE: f32 = 32.;
pub const MAX_LEAF_SIZE: f32 = 32.;
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
    left_child_split: bool,
    right_child_split: bool,
}

impl Leaf {
    fn split(&self) -> bool {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.5) {}

        let mut max: f32 = 0.;
        let mut split_b: bool = false;
        if (self.width > self.height) && (self.width / self.height >= 1.25) {
            max = self.width - MIN_LEAF_SIZE;
        } else if (self.height > self.width) && (self.height / self.width >= 1.25) {
            max = self.height - MIN_LEAF_SIZE;
            split_b = true;
        }
        if max <= MIN_LEAF_SIZE {
            return false;
        }
        let split = rng.gen_range(MIN_LEAF_SIZE..max);
        if split_b {
            let leftChild = Leaf {
                x: self.x,
                y: self.y,
                width: self.width,
                height: split,
                left_child_split: false,
                right_child_split: false,
            };
            let rightChild = Leaf {
                x: self.x,
                y: self.y + split,
                width: self.width,
                height: self.height - split,
                left_child_split: false,
                right_child_split: false,
            };
        } else {
            let leftChild = Leaf {
                x: self.x,
                y: self.y,
                width: split,
                height: self.height,
                left_child_split: false,
                right_child_split: false,
            };
            let rightChild = Leaf {
                x: self.x + split,
                y: self.y,
                width: self.width - split,
                height: self.height,
                left_child_split: false,
                right_child_split: false,
            };
        }
        return true;
    }
}

fn start_level() {
    let mut _leaf: Vec<Leaf> = vec![];
    let root = Leaf {
        x: 0.,
        y: 0.,
        width: 100.,
        height: 100.,
        left_child_split: false,
        right_child_split: false,
    };
    _leaf.push(root);
    let mut fully_split = false;
    while !fully_split {
        fully_split = true;
        for l in _leaf.iter() {
            if l.width > MAX_LEAF_SIZE || l.height > MAX_LEAF_SIZE || 1. > 0.25 {
                if l.split() {
                    
                }
            }
        }
    }
}

fn split_leaf(leaf: Leaf) -> bool {
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
            left_child_split: false,
            right_child_split: false,
        };
        let rightChild = Leaf {
            x: leaf.x,
            y: leaf.y + split,
            width: leaf.width,
            height: leaf.height - split,
            left_child_split: false,
            right_child_split: false,
        };
    } else {
        let leftChild = Leaf {
            x: leaf.x,
            y: leaf.y,
            width: split,
            height: leaf.height,
            left_child_split: false,
            right_child_split: false,
        };
        let rightChild = Leaf {
            x: leaf.x + split,
            y: leaf.y,
            width: leaf.width - split,
            height: leaf.height,
            left_child_split: false,
            right_child_split: false,
        };
    }
    return true;
}

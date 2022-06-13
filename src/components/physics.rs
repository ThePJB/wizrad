use crate::kmath::*;
use crate::entity::*;
use crate::wave_game::*;
use itertools::Itertools;

// Position based dynamics
#[derive(Debug)]
pub struct CollisionEvent {
    pub subject: u32,
    pub object: u32,
    pub penetration: Vec2,
}

#[derive(Clone)]
pub struct Physics {
    pub mass: f32,
    pub velocity: Vec2,
    pub old_pos: Vec2,
}

// 5 cases: both a in b, both b in a, a left in b, b left in a, no overlap
fn overlap_amount(a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
    let a1_in_b = a1 >= b1 && a1 <= b2;
    let a2_in_b = a2 >= b1 && a2 <= b2;
    let b1_in_a = b1 >= a1 && b1 <= a2;
    let b2_in_a = b2 >= a1 && b2 <= a2;

    let a_left = (a1 + a2) < (b1 + b2);

    if !a1_in_b && !a2_in_b && !b1_in_a && !b2_in_a {return 0.0;} // no overlap
    if a1_in_b && a2_in_b && a_left {return a2 - b1;} // a fully within b // maybe better to do distance to outside still
    if a1_in_b && a2_in_b && !a_left {return b2 - a1;} // a fully within b // maybe better to do distance to outside still
    if b1_in_a && b2_in_a && a_left {return a2 - b1;} // b fully within a
    if b1_in_a && b2_in_a && !a_left {return b2 - a1;} // b fully within a
    if a1_in_b {return b2 - a1;} // a to right of b
    if b1_in_a {return -(a2 - b1);} // b to right of a
    panic!("unreachable overlap");
}
// enum: subject_within, subject_touching, subject_outside
// if touching the pen vector is overlap amount in dir
// if within maybe need distance to outside as well
// distance to outside = 

// if theres a collision return axis and amount of least penetration
fn collide_rects(a: Rect, b: Rect) -> Option<Vec2> {
    let x_overlap = overlap_amount(a.left(), a.right(), b.left(), b.right());
    let y_overlap = overlap_amount(a.top(), a.bot(), b.top(), b.bot());

    if x_overlap == 0.0 || y_overlap == 0.0 {return None};

    if x_overlap.abs() < y_overlap.abs() {
        return Some(Vec2::new(x_overlap, 0.0));
    } 
    return Some(Vec2::new(0.0, y_overlap));
}

impl WaveGame {
    pub fn move_entities(&mut self, dt: f32) {
        for (id, phys) in self.physics.iter_mut() {
            let rect = self.rect.get_mut(id).unwrap();
            phys.old_pos = rect.centroid();
            *rect = rect.translate(phys.velocity * dt);
        }
    }

    pub fn collisions(&self) -> Vec<CollisionEvent> {
        self.physics.iter().cartesian_product(self.physics.iter())
            .filter(|((sid, sphys), (oid, ophys))| sid != oid)
            .filter_map(|((&sid, sphys), (&oid, ophys))| collide_rects(*self.rect.get(&sid).unwrap(), *self.rect.get(&oid).unwrap()).map(|pen| CollisionEvent {subject: sid, object: oid, penetration: pen}))
            .collect()
    }

    pub fn fix_overlaps(&mut self, cols: &[CollisionEvent], proportion: f32) {
        for col in cols {
            let omass = self.physics.get(&col.object).unwrap().mass;
            let sphys = self.physics.get(&col.subject).unwrap();
            let sw = sphys.mass / (sphys.mass + omass);
            // let ow = ophys.mass / denom;
            // what way is penetration
            let mut rect = self.rect.get_mut(&col.subject).unwrap();
            *rect = rect.translate((1.0 - sw) * col.penetration * proportion);
        }
    }

    pub fn fix_velocities(&mut self, dt: f32) {
        for (id, val) in self.physics.iter_mut() {
            let rect = self.rect.get_mut(&id).unwrap();

            val.velocity = (rect.centroid() - val.old_pos) / dt;
        }
    }

}


#[test]
fn test_overlap_amount() {
    assert_eq!(overlap_amount(-0.4, 0.6, -1.0, 1.0), 1.4);
    assert_eq!(overlap_amount(-0.5, 0.5, -1.0, 1.0), 1.5);
    assert_eq!(overlap_amount(-0.6, 0.4, -1.0, 1.0), 1.4);
    
    assert_eq!(overlap_amount(-1.0, 1.0, -0.4, 0.6), 1.4);// oh shit not reflective symmetry
    assert_eq!(overlap_amount(-1.0, 1.0, -0.5, 0.5), 1.5);
    assert_eq!(overlap_amount(-1.0, 1.0, -0.6, 0.4), 1.4); 
}

// still the case where they exactly overlap is problematic

// i hope the axis of penetration is opposite directions. yes it is
// exact overlap: maybe it would be good to handle and make it opposite.
// maybe substeps, if i move a smaller amount of the total distance to move
// other physics improvements; canonical overlaps
// i thought my noise wqould fiox it, would be good to check 
#[test]
fn test_axis_pen() {
    let mut wg = WaveGame::new();

    wg.add_entity(&Entity::new()
        .with_rect(Rect::new_centered(0.0, 0.0, 2.0, 2.0))
        .with_physics(1.0, Vec2::new(0.0, 0.0))
    );

    wg.add_entity(&Entity::new()
        .with_rect(Rect::new_centered(0.1, 0.05, 1.0, 1.0))
        .with_physics(1.0, Vec2::new(0.0, 0.0))
    );

    let cols = wg.collisions();
    println!("cols: {:?}", cols);
}
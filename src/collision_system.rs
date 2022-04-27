// use std::collections::HashMap;
// use crate::components::entity_common::Common;
// use crate::wave_game::*;
// use crate::kmath::*;

// #[derive(Debug)]
// pub struct CollisionEvent {
//     pub subject: u32,
//     pub object: u32,
//     pub penetration: Vec2,
// }

// // 5 cases: both a in b, both b in a, a left in b, b left in a, no overlap
// fn overlap_amount(a1: f32, a2: f32, b1: f32, b2: f32) -> f32 {
//     let a1_in_b = a1 >= b1 && a1 <= b2;
//     let a2_in_b = a2 >= b1 && a2 <= b2;
//     let b1_in_a = b1 >= a1 && b1 <= a2;
//     let b2_in_a = b2 >= a1 && b2 <= a2;

//     if !a1_in_b && !a2_in_b && !b1_in_a && !b2_in_a {return 0.0;} // no overlap
//     if a1_in_b && a2_in_b {return a2 - a1;} // a fully within b // maybe better to do distance to outside still
//     if b1_in_a && b2_in_a {return b2 - b1;} // b fully within a
//     if a1_in_b {return b2 - a1;} // a to right of b
//     if b1_in_a {return -(a2 - b1);} // b to right of a
//     panic!("unreachable overlap");
// }

// // if theres a collision return axis and amount of least penetration
// fn collide_rects(a: Rect, b: Rect) -> Option<Vec2> {
//     let x_overlap = overlap_amount(a.left(), a.right(), b.left(), b.right());
//     let y_overlap = overlap_amount(a.top(), a.bot(), b.top(), b.bot());

//     if x_overlap == 0.0 || y_overlap == 0.0 {return None};

//     if x_overlap.abs() < y_overlap.abs() {
//         return Some(Vec2::new(x_overlap, 0.0));
//     } 
//     return Some(Vec2::new(0.0, y_overlap));
// }

// pub fn collide_entity_entity(
//         entities: &HashMap<u32, Common>, 
//         collisions: &mut Vec<CollisionEvent>, 
//         dt: f32) {

//     for (subject_key, subject) in entities {

//         let subject_rect_desired = subject.rect.translate(subject.velocity * dt);
        
//         for (object_key, object) in entities {
//             if subject_key == object_key {continue};
            
//             if let Some(penetration) = collide_rects(subject_rect_desired, object.rect) {

//                 collisions.push(CollisionEvent {
//                     subject: *subject_key,
//                     object: *object_key,
//                     penetration,
//                 })
//             }
//         }
//     }
// }

// fn movement_bounds(subject_key: u32, collisions: &Vec<CollisionEvent>) -> (f32, f32, f32, f32) {
//     let max_dx = collisions.iter().filter(|col| col.subject == subject_key)
//         .filter(|col| col.penetration.x < 0.0)
//         .map(|col| col.penetration.x)
//         .fold(f32::INFINITY, |a, b| a.min(b));

//     let max_dy = collisions.iter().filter(|col| col.subject == subject_key)
//         .filter(|col| col.penetration.y < 0.0)
//         .map(|col| col.penetration.y)
//         .fold(f32::INFINITY, |a, b| a.min(b));
        
//     let min_dx = collisions.iter().filter(|col| col.subject == subject_key)
//         .filter(|col| col.penetration.x > 0.0)
//         .map(|col| col.penetration.x)
//         .fold(-f32::INFINITY, |a, b| a.max(b));

//     let min_dy = collisions.iter().filter(|col| col.subject == subject_key)
//         .filter(|col| col.penetration.y > 0.0)
//         .map(|col| col.penetration.y)
//         .fold(-f32::INFINITY, |a, b| a.max(b));

//     return (min_dx, max_dx, min_dy, max_dy);
// }

// fn clamp(val: f32, min: f32, max: f32) -> f32 {
//     match val {
//         val if val <= min => min,
//         val if val >= max => max,
//         _ => val
//     }
// }

// pub fn apply_movement2(entities: &mut HashMap<u32, Common>, collisions: &Vec<CollisionEvent>, dt: f32) {
//     for (entity_key, entity) in entities.iter_mut() {
//         entity.rect.x += entity.velocity.x * dt;
//         entity.rect.y += entity.velocity.y * dt;

//         for col in collisions {
//             if col.subject == *entity_key {
//                 entity.rect.translate(-col.penetration);
//             }
//             if col.object == *entity_key {
//                 entity.rect.translate(-col.penetration);
//             }
//         }
//     }
// }


// pub fn apply_movement(entities: &mut HashMap<u32, Common>, collisions: &Vec<CollisionEvent>, dt: f32) {
//     for (entity_key, entity) in entities.iter_mut() {
//         let (min_dx, max_dx, min_dy, max_dy) = movement_bounds(*entity_key, collisions);

//         let x_movt = entity.velocity.x * dt + clamp(0.0, min_dx, max_dx);
//         let y_movt = entity.velocity.y * dt + clamp(0.0, min_dy, max_dy);

//         entity.rect.x += x_movt;
//         entity.rect.y += y_movt;
//     }
// }

// #[test]
// fn test_collide_rects() {
//     {
//         let r1 = Rect::new(0.0, 0.0, 1.0, 1.0);
//         let r2 = Rect::new(0.9, 0.0, 1.0, 1.0);
//         println!("rect intersection: {:?}", collide_rects(r1, r2));
//     }
//     {
//         let r1 = Rect::new(0.0, 0.0, 1.0, 1.0);
//         let r2 = Rect::new(0.9, 0.1, 1.0, 1.0);
//         println!("rect intersection: {:?}", collide_rects(r1, r2));
//     }
// }

// // #[test]
// // fn test_collision() {
// //     // bruh every time i touch this delta vs absolutes

// //     let mut entities = HashMap::new();
// //     let mut collisions = Vec::new();
// //     entities.insert(0, Entity::new(EntityKind::Player, Vec2::new(0.0+0.025, 0.0+0.025)).with_velocity(Vec2::new(1.0, 0.0)));
// //     entities.insert(1, Entity::new(EntityKind::Player, Vec2::new(0.05+0.025, 0.0+0.025)));
// //     println!("entities before: {:?}", entities);
// //     collide_entity_entity(&entities, &mut collisions, 0.01);
// //     println!("collisions: {:?}", collisions);
// //     let bounds = movement_bounds(0, &collisions);
// //     println!("bounds: {:?}", bounds); // looks correct to me, actually bound seems wrong
// //     apply_movement(&mut entities, &collisions, 0.01);
// //     println!("entities now: {:?}", entities);
// // }
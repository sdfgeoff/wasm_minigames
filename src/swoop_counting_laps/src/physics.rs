use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

use itertools::Itertools;
use std::cell::RefCell;
use std::rc::Rc;

use super::map::Map;
use super::ship::Ship;
use super::transform::{length, normalize, vect_between, Vec2};

const SHIP_RADIUS: f32 = 0.05;
const GROUND_FRICTION: f32 = 5.0;

/// Moves the ships when they are close together, propagates velocity/motion
/// and all other physics of the ships.
pub fn calc_ship_physics(all_ships: &mut Vec<Ship>, map: &Map, dt: f32) {
    // Motion
    for ship in all_ships.iter_mut() {
        ship.update(dt as f32);
    }

    // Collisions between ships
    let ship_refs = all_ships.iter_mut().map(|x| Rc::new(RefCell::new(x)));
    let all_pairs = ship_refs.permutations(2);
    let collisions = all_pairs.filter_map(|ships: Vec<Rc<RefCell<&mut Ship>>>| {
        let ship1 = ships[0].clone();
        let ship2 = ships[1].clone();

        check_collision(ship1, ship2)
    });

    collisions.for_each(|pair| resolve_collision(pair));

    // Collisions with map
    for ship in all_ships.iter_mut() {
        let map_sdf = map.distance_field((ship.position.x, ship.position.y));
        if map_sdf > -SHIP_RADIUS {
            let normal = map.calc_normal((ship.position.x, ship.position.y));
            let overlap = map_sdf + SHIP_RADIUS;

            // Place ship back on the map
            ship.position.x -= normal.0 * overlap;
            ship.position.y -= normal.1 * overlap;

            // Slow the ship down
            ship.velocity.x -= ship.velocity.x * dt * GROUND_FRICTION;
            ship.velocity.y -= ship.velocity.y * dt * GROUND_FRICTION;
        }
    }
}

/// Returns the details of a collision between two ships.
fn check_collision<'a>(
    ship1: Rc<RefCell<&'a mut Ship>>,
    ship2: Rc<RefCell<&'a mut Ship>>,
) -> Option<CollisionEvent<'a>> {
    let normal = vect_between(&ship1.borrow().position, &ship2.borrow().position);
    let len = length(&normal);
    if len < SHIP_RADIUS * 2.0 {
        Some(CollisionEvent {
            obj1: ship1,
            obj2: ship2,
            normal: normalize(normal),
            overlap: len - SHIP_RADIUS * 2.0,
        })
    } else {
        None
    }
}

/// Use the details of a CollisionEvent to move the ships apart so they are
/// no longer colliding
fn resolve_collision(pair: CollisionEvent) {
    let mut ship1 = pair.obj1.borrow_mut();
    let mut ship2 = pair.obj2.borrow_mut();

    ship1.position.x -= pair.normal.0 * pair.overlap * 0.5;
    ship1.position.y -= pair.normal.1 * pair.overlap * 0.5;
    ship2.position.x += pair.normal.0 * pair.overlap * 0.5;
    ship2.position.y += pair.normal.1 * pair.overlap * 0.5;
}

#[derive(Debug)]
struct CollisionEvent<'a> {
    obj1: Rc<RefCell<&'a mut Ship>>,
    obj2: Rc<RefCell<&'a mut Ship>>,
    normal: Vec2,
    overlap: f32,
}

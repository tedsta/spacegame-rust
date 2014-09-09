use ship::Ship;
use module::{Engine, EngineModule};

pub fn generate_ship() -> Ship {
    let mut ship = Ship::new();
    ship.add_module(EngineModule::new());
    ship
}
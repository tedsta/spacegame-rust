use std::cmp;
use rand::Rng;
use rand;

use ship::{Ship, ShipId};
use module::{
    ModuleShape,

    EngineModule,
    ProjectileWeaponModule,
    ShieldModule,
    SolarModule,
    CommandModule,
    BeamWeaponModule
};

pub fn generate_ship(id: ShipId, name: String, level: u8) -> Ship {
    if level == 0 {
        panic!("Can't generate ship with level 0");
    }

    // Random number generater
    let mut rng = rand::thread_rng();

    // Brand new ship!!
    let mut ship = Ship::new(id, name, level);
    
    // Generate ship height
    let height = cmp::min(8, rng.gen::<u8>()%(cmp::max(level, 2)) + cmp::max(1, level/2));

    // Generate some random module counts
    let mut num_power = cmp::max(height, rng.gen::<u8>()%(level + 1) + 1);
    let num_engines = cmp::min(height, (rng.gen::<u8>()%(level + 1))/2 + 1);
    let num_shields = rng.gen::<u8>()%(level + 1);
    let num_weapons = rng.gen::<u8>()%(level + 1) + 1;
    let num_beams = rng.gen::<u8>()%(level/2) + 1;
    
    // Add top half engines
    for i in 0 .. num_engines/2 + num_engines%2 {
        let mut engine = EngineModule::new();
        engine.x = 0;
        engine.y = i;
        ship.add_module(engine);
    }
    
    // Add bottom half engines
    for i in 0 .. num_engines/2 {
        let mut engine = EngineModule::new();
        engine.x = 0;
        engine.y = height - 1 - i;
        ship.add_module(engine);
    }
    
    // Fill in any remaining space between engines with power modules
    for i in 0 .. height - num_engines {
        let mut solar = SolarModule::new();
        solar.x = 1;
        solar.y = num_engines/2 + num_engines%2 + i;
        ship.add_module(solar);
        num_power -= 1;
    }
    
    // Now, randomly fill up rest of ship with remaining modules
    let mut x = 2;
    let mut y = 0;
    
    let mut module_counts = [num_power, num_shields, num_weapons, num_beams];
    
    // While there's still modules to be placed...
    while module_counts.iter().filter(|x| **x > 0).count() > 0 {
        // Choose a module type
        let mut choice = rng.gen::<u8>()%(module_counts.len() as u8);
        
        // Make sure there are modules left to place of that type
        while module_counts[choice as usize] == 0 {
            choice += 1;
            if choice as usize >= module_counts.len() {
                choice = 0;
            }
        }
        
        // Power module
        if choice == 0 {
            let mut solar = SolarModule::new();
            solar.x = x;
            solar.y = y;
            ship.add_module(solar);
        } else if choice == 1 {
            let mut shield = ShieldModule::new();
            shield.x = x;
            shield.y = y;
            ship.add_module(shield);
        } else if choice == 2 {
            let mut weapon = ProjectileWeaponModule::new();
            weapon.x = x;
            weapon.y = y;
            ship.add_module(weapon);
        } else if choice == 3 {
            let mut beam = BeamWeaponModule::new();
            beam.x = x;
            beam.y = y;
            ship.add_module(beam);
        }
        
        // Decrement the chosen module's pool
        module_counts[choice as usize] -= 1;
    
        // Move cursor
        y += 1;
        if y >= height {
            y = 0;
            x += 1;
        }
    }
    
    // Figure out where to put command module
    let mut command_x = ship.get_width();
    let command_y = cmp::min(height - 1, rng.gen::<u8>()%(height + 1));
    
    while ship.is_space_free(command_x - 1, command_y, &ModuleShape::new(vec![vec![1, 0], vec![1, 0]])) {
        command_x -= 1;
    }
    
    // Finally, add the command module
    let mut command = CommandModule::new();
    command.x = command_x;
    command.y = command_y;
    ship.add_module(command);
    
    ship
}

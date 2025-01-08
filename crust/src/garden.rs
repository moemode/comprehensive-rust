//! This module implements the garden, including a highly performant germination
//! implementation.

pub struct Garden {
    // The garden is a 2D grid of plants.
    pub plants: Vec<Vec<Plant>>,
}

pub struct Plant {
    // The plant's age in days.
    pub age: u32,
    // The plant's height in centimeters.
    pub height: u32,
    // The plant's health, as a percentage.
    pub health: u32,
}

/// Harvest the produce in the garden that is ready.
pub fn harvest(garden: &mut Garden) {
    for row in &mut garden.plants {
        for plant in row {
            if plant.health >= 100 {
                println!("Harvesting a plant!");
                plant.health = 0;
            }
        }
    }
}
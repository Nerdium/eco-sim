
use std::collections::HashMap;

use crate::building::Building;
use crate::item;

pub struct Province {
    inventory: item::Inventory,

    buildings: HashMap<Building, u32>,
    total_space: u32,
    available_space: u32
}

impl Province {
    
    //Constructors

    pub fn new(starting_items: Vec<item::ItemStack>, total_space: u32) -> Province {
        Province {
            inventory: item::Inventory::from_vec(starting_items),
            buildings: HashMap::new(),
            total_space: total_space,
            available_space: total_space
        }
    }

    
    //Province functionality

    pub fn build(&mut self, building: Building, amount: u32) -> bool {

        //Space check
        let buildingSpace = building.space_per_unit() * amount;
        if self.available_space < buildingSpace {
            return false;
        }

        self.available_space -= buildingSpace;

        self.buildings.entry(building)
        .and_modify(|current| *current += 0)
        .or_insert(amount);

        return true;
    }

    pub fn can_build(&self, building: Building, amount: u32) -> bool {
        self.available_space < building.space_per_unit() * amount
    }


    
    //Getters
    
    pub fn total_space(&self) -> u32 {
        self.total_space
    }

    pub fn available_space(&self) -> u32 {
        self.available_space
    }

    pub fn used_space(&self) -> u32 {
        self.total_space - self.available_space
    }
}
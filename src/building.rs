
use std::fmt;

#[derive(PartialEq, Eq, Hash)]
pub enum Building {
    FARM,
    MINE,
    LUMBERYARD,
    FISHERY,
    STORE,
    FACTORY,
    HOUSING,
    INFRASTRUCTURE,
}

impl Building {
    pub fn space_per_unit(&self) -> u32 {
        match self {
            Building::FARM => 50,
            Building::MINE => 50,
            Building::LUMBERYARD => 50,
            Building::FISHERY => 50,
            Building::STORE => 50,
            Building::FACTORY => 50,
            Building::HOUSING => 10,
            Building::INFRASTRUCTURE => 1
        }
    }
}

impl fmt::Display for Building {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 
            match self {
                Building::FARM => "Farm",
                Building::MINE => "Mine",
                Building::LUMBERYARD => "Lumberyard",
                Building::FISHERY => "Fishery",
                Building::STORE => "Store",
                Building::FACTORY => "Factory",
                Building::HOUSING => "Housing",
                Building::INFRASTRUCTURE => "Infrastructure"
            }
        )
    } 
}
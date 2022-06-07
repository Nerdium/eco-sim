
mod building;
mod item;
mod province;

use crate::building::Building;
use crate::item::Item;
use crate::item::ItemStack;
use crate::province::Province;


fn main() {
    
    let starting_items: Vec<ItemStack> = vec![
        ItemStack { item: Item::FEED, amount: 20 },
        ItemStack { item: Item::GRAIN, amount: 10 },
        ItemStack { item: Item::FISH, amount: 30 },
    ];

    let mut province = Province::new(starting_items, 100);

    province.build(Building::FARM, 5);

}

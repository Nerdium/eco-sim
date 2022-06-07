
use std::fmt;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Item {
    //Natural
    GRAIN,
    MEAT,
    FISH,
    WOOD,
    COAL,
    ORE,
    
    //Manufactured
    FEED,
    MEAL,
    METAL
}

impl Item {
    pub fn is_natural(&self) -> bool {
        match self {
            Item::GRAIN => true,
            Item::MEAT => true,
            Item::FISH => true,
            Item::WOOD => true,
            Item::COAL => true,
            Item:: ORE => true,

            _ => false
        }
    }

    pub fn is_manufactured(&self) -> bool {
        !self.is_natural()
    }
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 
            match self {
                Item::GRAIN => "Grain",
                Item::MEAT => "Meat",
                Item::FISH => "Fish",
                Item::WOOD => "Wood",
                Item::COAL => "Coal",
                Item::ORE => "Ore",
                Item::FEED => "Feed",
                Item::MEAL => "Meal",
                Item::METAL => "Metal"
            }
        )
    }
}




pub struct ItemStack {
    pub item: Item,
    pub amount: u32
}




pub struct Inventory {
    item_map: HashMap<Item, u32>
}

impl Inventory {

    //Constructors

    pub fn new() -> Inventory {
        Inventory { item_map: HashMap::new() }
    }

    pub fn from_vec(stack_vec: Vec<ItemStack>) -> Inventory {
        let mut inventory = Inventory::new();

        for(stack) in stack_vec {
            inventory.add_stack(stack);
        }

        return inventory;
    }

    //Adding items

    pub fn add(&mut self, item: Item, amount: u32) {
        self.item_map.entry(item)
            .and_modify(|current| *current += amount)
            .or_insert(amount);
    }

    pub fn add_stack(&mut self, stack: ItemStack) {
        self.item_map.entry(stack.item)
            .and_modify(|current| *current += stack.amount)
            .or_insert(stack.amount);
    }

    //Put all items from another inventory into this one
    pub fn drain_from(&mut self, other: &mut Inventory) {
        for(item, amount) in other.item_map.drain() {
            self.add(item, amount);
        }
    }


    //Removing items

    //Tries to take a certain amount and returns none if not enough
    pub fn take(&mut self, item: Item, amount: u32) -> Option<ItemStack> {

        let mut taken = None;
        let mut emptied = false;

        self.item_map.entry(item)
            .and_modify(|current| {
                if *current >= amount {

                    if *current == amount {
                        emptied = true;
                    }

                    taken = Some(ItemStack { item: item, amount: amount });
                    *current -= amount
                }
            });

        //Remove empty entry
        if emptied {
            self.item_map.remove_entry(&item);
        }

        return taken;
    }

    //Takes all of an item
    pub fn take_all(&mut self, item: Item) -> Option<ItemStack> {
        match self.item_map.remove(&item) {
            Some(amount) => Some(ItemStack { item: item, amount: amount }),
            None => None
        }
    }

    //Takes as many of item as possible without going over amount
    pub fn take_at_most(&mut self, item: Item, amount: u32) -> Option<ItemStack> {

        let mut taken = None;
        let mut emptied = false;

        self.item_map.entry(item)
            .and_modify(|current| {
                if *current > amount {
                    taken = Some(ItemStack { item: item, amount: amount });
                    *current -= amount;
                } else {
                    taken = Some(ItemStack { item: item, amount: *current });
                    emptied = true;
                }
            });

        //Remove empty entry
        if emptied {
            self.item_map.remove_entry(&item);
        }

        return taken;

    }


    //Checking values

    pub fn contains(&self, item: Item) -> bool {
        self.item_map.contains_key(&item)
    }

    pub fn check_amount(&self, item: Item) -> u32 {
        match self.item_map.get(&item) {
            Some(val) => *val,
            None => 0
        }
    }
}
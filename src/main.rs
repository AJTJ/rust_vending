use std::collections::HashMap;

use vending_machine::{CoinDatabase, InventoryDatabase, VendingCreator};

fn main() {
    let coins: CoinDatabase = HashMap::new();
    let inventory: InventoryDatabase = HashMap::new();

    let my_machine = VendingCreator::create_vending_machine(coins, inventory);

    // Use to create game loop here to interact with the vending machine
}

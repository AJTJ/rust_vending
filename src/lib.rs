// 1. How do you design the Vending Machine in Java? (solution)

// You need to design a Vending Machine which
//  Accepts coins of 1,5,10,25 Cents i.e. penny, nickel, dime, and quarter.
//  Allow user to select products Coke(25), Pepsi(35), Soda(45)
//  Allow user to take refund by canceling the request.
//  Return the selected product and remaining change if any
//  Allow reset operation for vending machine supplier.

// https://javarevisited.blogspot.com/2016/06/design-vending-machine-in-java.html#axzz4sZVwtCgs

// NOTES
// single-threaded, for the moment

use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Coin {
    Cent,
    Nickle,
    Dime,
    Quarter,
    Dollar,
}

pub type CoinDatabase = HashMap<Coin, i32>;

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub enum Product {
    Coke(i32),
    Pepsi(i32),
    Soda(i32),
}

pub type InventoryDatabase = HashMap<Product, i32>;

#[derive(Clone)]
pub struct VendingMachine {
    coins: CoinDatabase,
    inventory: InventoryDatabase,
    inserted_coins_value: i32,
    total_sales: i32,
}

impl VendingMachine {
    // NOTE: It is important to separate the differences between public and private endpoints.
    // Here I am making it clea
    pub fn select_item_get_price() {
        unimplemented!()
    }

    // TRANSACTIONS
    pub fn insert_coin(self: &mut Self, c: Coin) {
        let mut coin_type = self.coins.get(&c);
        let coin_val = coin_type.get_or_insert(&0);
        self.coins.insert(c, 1 + *coin_val);
    }

    pub fn refund() {
        // system to spit out coins
        unimplemented!()
    }

    pub fn insufficent_product(&self, p: Product) {
        unimplemented!()
    }

    pub fn make_purchase_get_change(&mut self, p_choice: Product) {
        let inv = &mut self.inventory;
        let item = inv.get(&p_choice);

        // check current balance

        // check inventory
        if let Some(p) = item {
            if p > &0 {
                inv.insert(p_choice, p - 1);
            } else {
                let _ = &self.insufficent_product(p_choice);
            }
        }

        // TODO: create other public facing API endpoints
        // decrement balance
        // send product
        // return change

        self.private_function_name();

        unimplemented!()
    }
}

impl VendingMachine {
    fn private_function_name(&self) {
        unimplemented!()
    }
}

// A factory pattern for initializing a vending machine
pub struct VendingCreator;

impl VendingCreator {
    pub fn create_vending_machine(
        coins: CoinDatabase,
        inventory: InventoryDatabase,
    ) -> VendingMachine {
        VendingMachine {
            coins,
            inventory,
            inserted_coins_value: 0,
            total_sales: 0,
        }
    }
}

use std::collections::HashMap;

struct Store {
    name: String,
    prices: HashMap<String, f32>,
}

impl Store {
    fn new(name: String) -> Store {
        Store {
            name: name,
            prices: HashMap::new(),
        }
    }

    fn add_item(&mut self, name: String, price: f32) {
        self.prices.insert(name, price);
    }

    fn price(&self, item_name: &str) -> f32 {
        self.prices[item_name]
    }
}

fn build_stores() -> Vec<Store> {
    let mut stores = vec![];

    let mut store = Store::new(format!("R-mart"));
    store.add_item(format!("chocolate"), 5.0);
    store.add_item(format!("doll"), 22.0);
    store.add_item(format!("bike"), 150.0);
    stores.push(store);

    let mut store = Store::new(format!("Bullseye"));
    store.add_item(format!("chocolate"), 2.0);
    store.add_item(format!("doll"), 23.0);
    store.add_item(format!("bike"), 145.0);
    stores.push(store);

    let mut store = Store::new(format!("Woolmart"));
    store.add_item(format!("chocolate"), 2.0);
    store.add_item(format!("doll"), 23.0);
    store.add_item(format!("bike"), 146.0);
    stores.push(store);

    stores
}

fn find_best_store<'a>(stores: &'a Vec<Store>, shopping_list: &Vec<String>) -> Option<&'a Store> {
    stores.iter().min_by(|store_1, store_2| {
        let sum_1 = compute_sum(store_1, shopping_list);
        let sum_2 = compute_sum(store_2, shopping_list);
        sum_1.partial_cmp(&sum_2).unwrap()
    })
}

fn compute_sum(store: &Store, shopping_list: &Vec<String>) -> f32 {
    shopping_list.iter()
                 .map(|item_name| store.price(item_name))
                 .sum()
}

pub fn main() {
    let shopping_list = vec![format!("chocolate"),
                             format!("doll"),
                             format!("bike")];
    let stores = build_stores();
    if let Some(&Store { ref name, .. }) = find_best_store(&stores, &shopping_list) {
        println!("Best store: {}", name);
    } else {
        println!("No stores present");
    }
}

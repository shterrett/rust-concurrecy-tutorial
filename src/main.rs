use std::f32::INFINITY;
use std::collections::HashMap;
use std::thread;
use std::sync::{ Arc, Mutex };

#[derive(Clone)]
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

fn find_best_store(stores: Vec<Store>, shopping_list: Arc<Vec<String>>) -> Option<Store> {
    let best: Arc<Mutex<(f32, Option<Store>)>> = Arc::new(Mutex::new((INFINITY, None)));

    let _ = stores.into_iter()
                  .map(|store| {
                      let local_list = shopping_list.clone();
                      let local_best = best.clone();
                      thread::spawn(move || {
                          let mut data = local_best.lock().unwrap();
                          let price = compute_sum(&store, local_list);
                          if price < data.0  {
                              *data = (price, Some(store));
                          }
                      })
                  })
                  .map(|handle| handle.join().unwrap())
                  .count();
    let answer = best.lock().unwrap();
    answer.1.clone()
}

fn compute_sum(store: &Store, shopping_list: Arc<Vec<String>>) -> f32 {
    shopping_list.iter()
                 .map(|item_name| store.price(item_name))
                 .sum()
}

pub fn main() {
    let shopping_list = Arc::new(vec![format!("chocolate"),
                                      format!("doll"),
                                      format!("bike")]);
    let stores = build_stores();
    if let Some(Store { name, .. }) = find_best_store(stores, shopping_list.clone()) {
        println!("Best store: {}", name);
    } else {
        println!("No stores present");
    }
}

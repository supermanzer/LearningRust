use std::collections::HashMap;

pub fn create_map() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert(String::from("a"), 10);
    map.insert(String::from("b"), 20);
    map.insert(String::from("c"), 30);
    return map;
}

pub fn access_map(map: &HashMap<String, i32>, key: &str) -> i32 {
    let val = map.get(key).copied().unwrap_or(0);
    return val;
}

pub fn interate_map(map: &HashMap<String, i32>) {
    for (key, value) in map {
        println!("Key: {}, Value: {}", key, value);
    }
}

pub fn update_map() {
    let mut map = create_map();
    map.insert(String::from("a"), 100);
    map.entry("b".to_string()).or_insert(200);
    println!("Map: {:?}", map);
    map.entry("d".to_string()).or_insert(400);
    println!("Map: {:?}", map);
    let keys = "a b c d";
    for key in keys.split_whitespace() {
        let count = map.entry(key.to_string()).or_insert(0);
        *count += 1;
    }
    println!("Map: {:?}", map);
}

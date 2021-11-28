// By default rust uses SipHash for hashmaps
use std::collections::HashMap;

pub fn run() {
    // Create a Hashmap of strings
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // This returns an optional because the key may not have a value in the map
    let score = scores.get(&team_name); 
    
    // Iterating key-vals
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // The entry.or_insert API only inserts if the key doesnt exist
    scores.entry(String::from("Blue")).or_insert(10000);
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);

    // Another way to create the map is to zip 2 iters
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores2: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores2);

    // Note that once we insert into a map we lose ownership of those vars
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
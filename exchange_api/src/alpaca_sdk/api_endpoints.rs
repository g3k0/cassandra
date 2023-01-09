use std::collections::HashMap;

struct Endpoint {
    path: String,
    method: String,
}

impl Endpoint {
    fn new(path: &str, method: &str) -> Endpoint {
        Endpoint {
            path: path.to_string(),
            method: method.to_string(),
        }
    }
}

fn get_endpoints() {
    // Define the type of the Hashmap keys as `String` and the type of the values as `Vec<Endpoint>`
    let mut endpoints: HashMap<String, Vec<Endpoint>> = HashMap::new();

    // Add a list of endpoints for the key "home"
    endpoints.insert(
        "home".to_string(),
        vec![Endpoint::new("/", "GET"), Endpoint::new("/home", "POST")],
    );

    // Add a list of endpoints for the key "users"
    endpoints.insert(
        "users".to_string(),
        vec![
            Endpoint::new("/users", "GET"),
            Endpoint::new("/users/:id", "GET"),
            Endpoint::new("/users", "POST"),
        ],
    );
}
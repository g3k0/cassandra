use std::collections::HashMap;


// TO DO - insert body parameters for post and query strings
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

    /**
     * Add a list of endpoints for the key "account"
     */
    endpoints.insert(
        "account".to_string(),
        vec![Endpoint::new("/v2/account", "GET")],
    );

    /**
     * Add a list of endpoints for the key "orders"
     */ 
    endpoints.insert(
        "orders".to_string(),
        vec![
            Endpoint::new("/v2/orders", "GET"),
            Endpoint::new("/v2/orders/:order_id", "GET"),
            Endpoint::new("/v2/orders", "POST"),
            Endpoint::new("/v2/orders/:order_id", "PATCH"),
            Endpoint::new("/v2/orders", "DELETE"),
            Endpoint::new("/v2/orders/:order_id", "DELETE"),
        ],
    );

    /**
     * Add a list of endpoints for the key "positions"
     */ 
    endpoints.insert(
        "positions".to_string(),
        vec![
            Endpoint::new("/v2/positions", "GET"),
            Endpoint::new("/v2/positions/:symbol_or_asset_id", "GET"),
            Endpoint::new("/v2/positions", "DELETE"),
            Endpoint::new("/v2/positions/:symbol_or_asset_id", "DELETE"),
        ],
    );

    /**
     * Add a list of endpoints for the key "assets"
     */ 
    endpoints.insert(
        "assets".to_string(),
        vec![
            Endpoint::new("/v2/assets", "GET"),
            Endpoint::new("/v2/assets/:asset_id", "GET"),
            Endpoint::new("/v2/assets/:symbol", "GET"),
        ],
    );

    /**
     * Add a list of endpoints for the key "watchlists"
     */ 
    endpoints.insert(
        "watchlists".to_string(),
        vec![
            Endpoint::new("/v2/watchlists", "GET"),
            Endpoint::new("/v2/watchlists", "POST"),
            Endpoint::new("/v2/watchlists/:watchlist_id", "GET"),
            Endpoint::new("/v2/watchlists/:by_name", "GET"),
            Endpoint::new("/v2/watchlists/:watchlist_id", "POST"),
            Endpoint::new("/v2/watchlists/:by_name", "POST"),
            Endpoint::new("/v2/watchlists/:watchlist_id", "PUT"),
            Endpoint::new("/v2/watchlists/:by_name", "PUT"),
            Endpoint::new("/v2/watchlists/:watchlist_id", "DELETE"),
            Endpoint::new("/v2/watchlists/:by_name", "DELETE"),
            Endpoint::new("/v2/watchlists/:watchlist_id/:symbol", "DELETE"),
        ],
    );

    /**
     * Add a list of endpoints for the key "calendar"
     */ 
    endpoints.insert(
        "calendar".to_string(),
        vec![
            Endpoint::new("/v2/calendar", "GET"),
        ],
    );

    /**
     * Add a list of endpoints for the key "clock"
     */ 
    endpoints.insert(
        "clock".to_string(),
        vec![
            Endpoint::new("/v2/clock", "GET"),
        ],
    );

    /**
     * Add a list of endpoints for the key "corporate_actions_announcements"
     */ 
    endpoints.insert(
        "corporate_actions_announcements".to_string(),
        vec![
            Endpoint::new("v2/corporate_actions/announcements", "GET"),
            Endpoint::new("v2/corporate_actions/announcements/:announcement_id", "GET"),
        ],
    );

    /**
     * Add a list of endpoints for the key "account_configurations"
     */ 
    endpoints.insert(
        "account_configurations".to_string(),
        vec![
            Endpoint::new("/v2/account/configurations", "GET"),
        ],
    );

    /**
     * Add a list of endpoints for the key "account_activities"
     */ 
    endpoints.insert(
        "account_activities".to_string(),
        vec![
            Endpoint::new("/v2/account/activities", "GET"),
            Endpoint::new("/v2/account/activities/:activity_type", "GET"),
        ],
    );

    /**
     * Add a list of endpoints for the key "portfolio_history"
     */ 
    endpoints.insert(
        "portfolio_hisotry".to_string(),
        vec![
            Endpoint::new("/v2/account/portfolio/history", "GET"),
        ],
    );
}
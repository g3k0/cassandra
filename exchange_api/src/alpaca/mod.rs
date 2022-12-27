/**
 * In order to use the Alpaca API, set the following env variables at system level:
 * - APCA_API_BASE_URL
 * - APCA_API_KEY_ID
 * - APCA_API_SECRET_KEY
 */
use apca::ApiInfo;
use apca::Client;
use apca::api::v2::order;

use apca::api::v2::order::Order;
use apca::api::v2::order::OrderReq;
use num_decimal::Num;
struct ClientSingleton;

impl ClientSingleton {
    pub fn new() -> Client {
        let api_info: ApiInfo = ApiInfo::from_env().unwrap();
        let client: Client = Client::new(api_info);
        client
    }
}

lazy_static! {
    static ref CLIENT: Client = ClientSingleton::new();
}

pub async fn buy(asset: &str, amount: i16, limit_price: i32) {
    let request: OrderReq = order::OrderReqInit {
        type_: order::Type::Limit,
        limit_price: Some(Num::from(limit_price)),
        ..Default::default()
    }
    .init(asset, order::Side::Buy, order::Amount::quantity(amount));
    
    let order: Order = CLIENT.issue::<order::Post>(&request).await.unwrap();
    println!("Created order {}", order.id.as_hyphenated());
}
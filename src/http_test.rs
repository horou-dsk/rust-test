use std::collections::HashMap;
use serde_json::{Value, Map};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Param {
    address: String,
    binary: bool,
    foward: bool,
    ledger_index_max: i8,
    ledger_index_min: i8,
    limit: u32
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
    id: u32
}

#[tokio::main]
async fn run() -> Result<(), Box<dyn std::error::Error>> {
    /*let mut map = HashMap::new();
    /*binary: false,
        foward: false,
        ledger_index_max: -1,
        ledfer_index_min: -1,
        limit: 1000,*/
    map.insert("account", "rPALAcRgn6BFdtJJtSDaCaHhF8X8eNSLyi");
    map.insert("binary", "false");
    map.insert("foward", "false");
    map.insert("ledger_index_max", "-1");
    map.insert("ledfer_index_min", "-1");
    map.insert("limit", "100");*/
    let param = Param {
        address: "rPALAcRgn6BFdtJJtSDaCaHhF8X8eNSLyi".to_string(),
        binary: false,
        foward: false,
        ledger_index_max: -1,
        ledger_index_min: -1,
        limit: 1000
    };
    let user = User {
        id: 11,
        name: "Nagisa".to_string()
    };
    // let encoded = json::encode(&param).unwrap();
    let client = reqwest::Client::new();
    let resp = client.post("http://127.0.0.1:8089/login/mongoPost")
        .header("Content-Type", "application/json;charset=UTF-8")
        .json(&user)
        .send()
        .await?
        .json::<Map<String, Value>>()
        .await?;

    match resp.get("msg") {
        Option::Some(msg) => {
            println!("{}", msg)
        }
        Option::None => {
            println!("我操，这是空的额")
        }
    }
    // println!("{}", resp.get("code"));
    Ok(())
}

#[test]
fn my_main() {
    run();
}
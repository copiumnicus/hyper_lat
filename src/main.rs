use std::io::Write;
use std::time::Duration;

use hyper_latency::{Socket, LatErr};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

fn f64_to_string<S>(x: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&x.to_string())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "channel", content = "data")]
pub enum Message {
    #[serde(rename = "subscriptionResponse")]
    SubRes(MethodData),
    #[serde(rename = "l2Book")]
    L2Book(WsBook),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MethodData {
    pub method: String,
    pub subscription: Subscription,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Subscription {
    /// Message type (e.g., "l2Book")
    #[serde(rename = "type")]
    pub type_: String,
    /// Coin symbol
    pub coin: String,
    /// Number of significant figures
    #[serde(rename = "nSigFigs")]
    pub n_sig_figs: Option<u32>,
    /// Mantissa value (can be null)
    pub mantissa: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsBook {
    pub coin: String,
    pub time: u64,
    // [BID,ASK]
    pub levels: [Vec<WsLevel>; 2],
}
impl WsBook {
    pub fn time(&self) -> Duration {
        Duration::from_millis(self.time)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WsLevel {
    /// Price
    #[serde(deserialize_with = "string_to_f64", serialize_with = "f64_to_string")]
    pub px: f64,
    /// Size
    #[serde(deserialize_with = "string_to_f64", serialize_with = "f64_to_string")]
    pub sz: f64,
    /// Number of orders
    pub n: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Method {
    pub method: String,
    pub subscription: Sub,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sub {
    /// "l2Book" as message type
    pub r#type: String,
    /// coin symbol
    pub coin: String,
}
impl Sub {
    pub fn l2book(coin: impl ToString) -> Method {
        Method {
            method: "subscribe".into(),
            subscription: Self {
                r#type: "l2Book".into(),
                coin: coin.to_string(),
            },
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct HyperRawPriceConfig {
    quote: String,
    assets: Vec<String>,
}

impl HyperRawPriceConfig {
    fn _run(self) -> Result<(), LatErr> {
        println!("START HYPER RAW PRICE");
        let url = "wss://api.hyperliquid.xyz/ws";
        let s = Socket::new(url)?;
        let mut subs = Vec::new();
        for a in self.assets {
            subs.push(serde_json::to_string(&Sub::l2book(a))?);
        }
        s.send_multi(subs)?;

        // Open file in append mode
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open("timestamps.txt")?;

        loop {
            let st = s.read()?;
            let m: Message = serde_json::from_str(&st)?;
            match m {
                Message::L2Book(b) => {
                    // Write to file instead of printing
                    writeln!(
                        file,
                        "EX_TS_MS={} LOCAL_TS_MS={}",
                        b.time,
                        timed::now().as_millis()
                    )?;
                }
                Message::SubRes(r) => {
                    println!("HYPER_BOOK_ACK {:?}", r);
                }
            }
        }
    }
}

pub fn main() {
    let c = HyperRawPriceConfig {
        quote: "USDC".into(),
        assets: vec!["ETH".into()],
    };
    c._run().unwrap();
}

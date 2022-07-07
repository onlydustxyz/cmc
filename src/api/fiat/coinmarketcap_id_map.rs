use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fmt::Write;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CmcIdMapFiat {
    pub status: Status,
    pub data: Vec<Currency>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Status {
    pub timestamp: String,
    pub error_code: i64,
    pub error_message: Value,
    pub elapsed: i64,
    pub credit_count: i64,
    pub notice: Value,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    pub id: i64,
    pub name: String,
    pub sign: String,
    pub symbol: String,
}

impl CmcIdMapFiat {
    pub fn display(&self) -> String {
        let mut s = String::new();
        for f in &self.data {
            let _ = write!(
                s,
                "Id: {}\nName: {}\nSign: {}\nSymbol: {}\n---------------",
                f.id, f.name, f.sign, f.symbol
            );
        }
        s
    }
}

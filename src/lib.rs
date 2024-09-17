pub mod builder;
pub mod search;

use log::kv::{ToValue, Value};
use search::*;

// For logging with log::kv feature
macro_rules! impl_to_value {
    ($($t:ty),+) => {
        $(
            impl ToValue for $t {
                fn to_value(&self) -> Value {
                    Value::from_serde(self)
                }
            }
        )+
    };
}

impl_to_value! {
    SearchGMPRequest,
    SearchGMPResponse,
    GMPTransaction
}

use serde_derive::Deserialize;
use serde_derive::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub market: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub base_volume: String,
    pub quote_volume: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub fees: String,
}

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityProviders {
    pub position_id: String,
    pub liquidity : Liquidity,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Liquidity {
    pub available_funds: String,
    pub stark_key: String,
    pub quote:Option<Quote>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub credit_asset: String,
    pub credit_amount: String,
    pub debit_amount: String,
}


impl LiquidityProviders {
    pub fn default() -> LiquidityProviders {
        LiquidityProviders {
            position_id: "".to_string(),
            liquidity: Liquidity::default(),
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub product_name: String,
    pub supplier_id: i32,
    pub unit_price: bigdecimal::BigDecimal,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub package: String,
    pub is_discontinued: bool,
}

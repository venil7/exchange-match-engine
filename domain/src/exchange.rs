#[derive(serde::Deserialize, serde::Serialize)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OrderRequest {
    #[prost(enumeration="Direction", tag="1")]
    pub direction: i32,
    #[prost(enumeration="OrderType", tag="2")]
    pub r#type: i32,
    #[prost(double, tag="3")]
    pub amount: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Direction {
    Buy = 0,
    Sell = 1,
}
impl Direction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Direction::Buy => "Buy",
            Direction::Sell => "Sell",
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OrderType {
    Market = 0,
    Limit = 1,
}
impl OrderType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            OrderType::Market => "Market",
            OrderType::Limit => "Limit",
        }
    }
}

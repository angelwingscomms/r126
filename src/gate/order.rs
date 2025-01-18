use serde::{Serialize, Deserialize, Deserializer};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct Order {
    pub id: Option<String>,
    pub text: Option<String>,
    pub amend_text: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
    pub create_time_ms: Option<i64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number_i64")]
    pub update_time_ms: Option<i64>,
    pub status: Option<OrderStatus>,
    pub currency_pair: Option<String>,
    #[serde(rename = "type")]
    pub order_type: Option<OrderType>,
    pub account: Option<String>,
    pub side: Option<OrderSide>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub amount: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub price: Option<f64>,
    pub time_in_force: Option<TimeInForce>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub iceberg: Option<f64>,
    pub auto_borrow: Option<bool>,
    pub auto_repay: Option<bool>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub left: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub filled_amount: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub fill_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub filled_total: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub avg_deal_price: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub fee: Option<f64>,
    pub fee_currency: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub point_fee: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub gt_fee: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub gt_maker_fee: Option<f64>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub gt_taker_fee: Option<f64>,
    pub gt_discount: Option<bool>,
    #[serde(deserialize_with = "deserialize_option_string_or_number")]
    pub rebated_fee: Option<f64>,
    pub rebated_fee_currency: Option<String>,
    #[serde(deserialize_with = "deserialize_option_string_or_number_i64")]
    pub stp_id: Option<i64>,
    pub stp_act: Option<StpAction>,
    pub finish_as: Option<FinishStatus>,
    pub action_mode: Option<ActionMode>,
}

fn deserialize_option_string_or_number<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = Option<f64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or number representing an Option<f64>")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(StringOrNumberVisitor)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse().map(Some).map_err(serde::de::Error::custom)
        }

        fn visit_f64<E>(self, value: f64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value))
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value as f64))
        }
    }

    deserializer.deserialize_option(StringOrNumberVisitor)
}

fn deserialize_option_string_or_number_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrNumberVisitor;

    impl<'de> serde::de::Visitor<'de> for StringOrNumberVisitor {
        type Value = Option<i64>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or number representing an Option<i64>")
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(None)
        }

        fn visit_some<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_any(StringOrNumberVisitor)
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            value.parse().map(Some).map_err(serde::de::Error::custom)
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value))
        }

        fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Some(value as i64))
        }
    }

    deserializer.deserialize_option(StringOrNumberVisitor)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Open,
    Closed,
    Cancelled,
}

impl fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Limit,
    Market,
}

impl fmt::Display for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    Buy,
    Sell,
}

impl fmt::Display for OrderSide {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TimeInForce {
    GTC,
    IOC,
    POC,
    FOK,
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StpAction {
    CN,
    CO,
    CB,
    #[serde(rename = "-")]
    Dash,
}

impl fmt::Display for StpAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum FinishStatus {
    Open,
    Filled,
    Cancelled,
    LiquidateCancelled,
    DepthNotEnough,
    TraderNotEnough,
    Small,
    IOC,
    POC,
    FOK,
    STP,
    Unknown,
}

impl fmt::Display for FinishStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ActionMode {
    ACK,
    RESULT,
    FULL,
}

impl fmt::Display for ActionMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

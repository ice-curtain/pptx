use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlIterateIntervalTime;

use crate::schemas::presentation::main::CtTlIterateIntervalPercentage;

/**
 * @author : zhilong.zhou
 * @description : CT_TLIterateData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlIterateData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@backwards")]
    pub backwards_attr: Option<String>,

    #[serde(rename(serialize = "p:tmAbs", deserialize = "tmAbs"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm_abs: Option<CtTlIterateIntervalTime>,

    #[serde(rename(serialize = "p:tmPct", deserialize = "tmPct"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tm_pct: Option<CtTlIterateIntervalPercentage>,
}

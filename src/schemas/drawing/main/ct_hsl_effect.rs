use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_HSLEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHslEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hue")]
    pub hue_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sat")]
    pub sat_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lum")]
    pub lum_attr: Option<String>,
}

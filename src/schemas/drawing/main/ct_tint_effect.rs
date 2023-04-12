use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TintEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTintEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hue")]
    pub hue_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@amt")]
    pub amt_attr: Option<String>,
}

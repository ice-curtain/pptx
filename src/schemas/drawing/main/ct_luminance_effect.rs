use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_LuminanceEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLuminanceEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bright")]
    pub bright_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@contrast")]
    pub contrast_attr: Option<String>,
}

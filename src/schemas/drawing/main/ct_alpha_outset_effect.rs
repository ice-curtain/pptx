use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AlphaOutsetEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAlphaOutsetEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rad")]
    pub rad_attr: Option<String>,
}

use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AlphaModulateFixedEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAlphaModulateFixedEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@amt")]
    pub amt_attr: Option<String>,
}

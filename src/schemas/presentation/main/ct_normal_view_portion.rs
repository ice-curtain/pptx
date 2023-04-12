use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_NormalViewPortion
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNormalViewPortion {
    #[serde(rename = "@sz")]
    pub sz_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@autoAdjust")]
    pub auto_adjust_attr: Option<String>,
}

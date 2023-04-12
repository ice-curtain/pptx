use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_InOutTransition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtInOutTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dir")]
    pub dir_attr: Option<String>,
}

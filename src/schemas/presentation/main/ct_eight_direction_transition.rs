use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_EightDirectionTransition
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEightDirectionTransition {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dir")]
    pub dir_attr: Option<String>,
}

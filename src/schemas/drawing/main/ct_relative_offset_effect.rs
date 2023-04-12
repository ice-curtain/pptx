use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_RelativeOffsetEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtRelativeOffsetEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tx")]
    pub tx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ty")]
    pub ty_attr: Option<String>,
}

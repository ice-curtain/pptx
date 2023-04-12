use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Guide
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGuide {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@orient")]
    pub orient_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@pos")]
    pub pos_attr: Option<String>,
}

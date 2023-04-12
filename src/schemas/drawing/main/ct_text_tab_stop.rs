use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextTabStop
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextTabStop {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@pos")]
    pub pos_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@algn")]
    pub algn_attr: Option<String>,
}

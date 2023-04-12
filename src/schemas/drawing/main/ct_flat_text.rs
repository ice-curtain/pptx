use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_FlatText
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtFlatText {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@z")]
    pub z_attr: Option<String>,
}

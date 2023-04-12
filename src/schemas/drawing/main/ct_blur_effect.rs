use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_BlurEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBlurEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rad")]
    pub rad_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@grow")]
    pub grow_attr: Option<String>,
}

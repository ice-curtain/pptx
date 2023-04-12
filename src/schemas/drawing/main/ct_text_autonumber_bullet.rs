use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TextAutonumberBullet
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextAutonumberBullet {
    #[serde(rename = "@type")]
    pub r#type_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@startAt")]
    pub start_at_attr: Option<String>,
}

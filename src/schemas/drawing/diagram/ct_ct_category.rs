use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_CTCategory
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCtCategory {
    #[serde(rename = "@type")]
    pub r#type_attr: String,

    #[serde(rename = "@pri")]
    pub pri_attr: String,
}

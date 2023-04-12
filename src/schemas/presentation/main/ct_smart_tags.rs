use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SmartTags
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSmartTags {
    #[serde(rename = "@r:id")]
    pub r_id_attr: String,
}

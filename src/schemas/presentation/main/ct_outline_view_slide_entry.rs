use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_OutlineViewSlideEntry
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOutlineViewSlideEntry {
    #[serde(rename = "@r:id")]
    pub r_id_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@collapse")]
    pub collapse_attr: Option<String>,
}

use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TagsData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTagsData {
    #[serde(rename = "@r:id")]
    pub r_id_attr: String,
}

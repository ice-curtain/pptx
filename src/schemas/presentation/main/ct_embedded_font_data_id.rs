use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_EmbeddedFontDataId
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEmbeddedFontDataId {
    #[serde(rename = "@r:id")]
    pub r_id_attr: String,
}

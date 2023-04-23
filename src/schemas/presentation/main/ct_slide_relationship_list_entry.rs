use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SlideRelationshipListEntry
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideRelationshipListEntry {
    #[serde(rename = "@r:id")]

    pub r_id_attr: String,
}

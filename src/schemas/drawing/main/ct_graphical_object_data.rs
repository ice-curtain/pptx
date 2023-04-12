use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_GraphicalObjectData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGraphicalObjectData {
    #[serde(rename = "@uri")]
    pub uri_attr: String,
}

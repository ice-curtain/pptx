use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLSubShapeId
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlSubShapeId {
    #[serde(rename = "@spid")]
    pub spid_attr: String,
}

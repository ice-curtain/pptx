use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_CustomShowId
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustomShowId {
    #[serde(rename = "@id")]
    pub id_attr: String,
}

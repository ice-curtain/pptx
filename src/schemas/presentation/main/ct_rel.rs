use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Rel
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtRel {
    #[serde(rename = "@r:id")]
    pub r_id_attr: String,
}

use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_SoftEdgesEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSoftEdgesEffect {
    #[serde(rename = "@rad")]
    pub rad_attr: String,
}

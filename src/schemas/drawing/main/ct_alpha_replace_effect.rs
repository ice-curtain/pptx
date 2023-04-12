use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AlphaReplaceEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAlphaReplaceEffect {
    #[serde(rename = "@a")]
    pub a_attr: String,
}

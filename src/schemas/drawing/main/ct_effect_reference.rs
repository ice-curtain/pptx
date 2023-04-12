use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_EffectReference
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEffectReference {
    #[serde(rename = "@ref")]
    pub ref_attr: String,
}

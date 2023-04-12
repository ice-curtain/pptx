use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtEffectContainer;

/**
 * @author : zhilong.zhou
 * @description : CT_BlendEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBlendEffect {
    #[serde(rename = "@blend")]
    pub blend_attr: String,

    #[serde(rename(serialize = "a:cont", deserialize = "cont"))]
    pub cont: Box<CtEffectContainer>,
}

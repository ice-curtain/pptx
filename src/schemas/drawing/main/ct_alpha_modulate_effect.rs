use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtEffectContainer;

/**
 * @author : zhilong.zhou
 * @description : CT_AlphaModulateEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAlphaModulateEffect {
    #[serde(rename(serialize = "a:cont", deserialize = "cont"))]
    pub cont: Box<CtEffectContainer>,
}

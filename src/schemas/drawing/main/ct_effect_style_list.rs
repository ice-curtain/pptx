use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtEffectStyleItem;

/**
 * @author : zhilong.zhou
 * @description : CT_EffectStyleList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEffectStyleList {
    #[serde(rename(serialize = "a:effectStyle", deserialize = "effectStyle"))]
    pub effect_style: Vec<CtEffectStyleItem>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtLineProperties;

use crate::schemas::drawing::main::CtEffectContainer;

use crate::schemas::drawing::main::CtEffectList;

/**
 * @author : zhilong.zhou
 * @description : CT_WholeE2oFormatting
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtWholeE2oFormatting {
    #[serde(rename(serialize = "a:ln", deserialize = "ln"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:effectLst", deserialize = "effectLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_lst: Option<Box<CtEffectList>>,

    #[serde(rename(serialize = "a:effectDag", deserialize = "effectDag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_dag: Option<Box<CtEffectContainer>>,
}

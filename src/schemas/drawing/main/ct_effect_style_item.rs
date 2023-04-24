use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtShape3D;

use crate::schemas::drawing::main::CtEffectContainer;

use crate::schemas::drawing::main::CtScene3D;

use crate::schemas::drawing::main::CtEffectList;

/**
 * @author : zhilong.zhou
 * @description : CT_EffectStyleItem
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEffectStyleItem {


    #[serde(rename(serialize = "a:effectLst", deserialize = "effectLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_lst: Option<Box<CtEffectList>>,

    #[serde(rename(serialize = "a:effectDag", deserialize = "effectDag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_dag: Option<Box<CtEffectContainer>>,


    #[serde(rename(serialize = "a:scene3d", deserialize = "scene3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene3d: Option<CtScene3D>,

    #[serde(rename(serialize = "a:sp3d", deserialize = "sp3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp3d: Option<CtShape3D>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlurEffect;

use crate::schemas::drawing::main::CtGlowEffect;

use crate::schemas::drawing::main::CtFillOverlayEffect;

use crate::schemas::drawing::main::CtPresetShadowEffect;

use crate::schemas::drawing::main::CtSoftEdgesEffect;

use crate::schemas::drawing::main::CtInnerShadowEffect;

use crate::schemas::drawing::main::CtReflectionEffect;

use crate::schemas::drawing::main::CtOuterShadowEffect;

/**
 * @author : zhilong.zhou
 * @description : CT_EffectList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEffectList {
    #[serde(rename(serialize = "a:blur", deserialize = "blur"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<CtBlurEffect>,

    #[serde(rename(serialize = "a:fillOverlay", deserialize = "fillOverlay"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_overlay: Option<Box<CtFillOverlayEffect>>,

    #[serde(rename(serialize = "a:glow", deserialize = "glow"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow: Option<CtGlowEffect>,

    #[serde(rename(serialize = "a:innerShdw", deserialize = "innerShdw"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_shdw: Option<CtInnerShadowEffect>,

    #[serde(rename(serialize = "a:outerShdw", deserialize = "outerShdw"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_shdw: Option<CtOuterShadowEffect>,

    #[serde(rename(serialize = "a:prstShdw", deserialize = "prstShdw"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_shdw: Option<CtPresetShadowEffect>,

    #[serde(rename(serialize = "a:reflection", deserialize = "reflection"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflection: Option<CtReflectionEffect>,

    #[serde(rename(serialize = "a:softEdge", deserialize = "softEdge"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_edge: Option<CtSoftEdgesEffect>,
}

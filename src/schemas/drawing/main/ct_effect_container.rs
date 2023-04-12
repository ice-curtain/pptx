use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAlphaCeilingEffect;

use crate::schemas::drawing::main::CtPresetShadowEffect;

use crate::schemas::drawing::main::CtGrayscaleEffect;

use crate::schemas::drawing::main::CtAlphaModulateFixedEffect;

use crate::schemas::drawing::main::CtHslEffect;

use crate::schemas::drawing::main::CtTintEffect;

use crate::schemas::drawing::main::CtBlurEffect;

use crate::schemas::drawing::main::CtAlphaReplaceEffect;

use crate::schemas::drawing::main::CtLuminanceEffect;

use crate::schemas::drawing::main::CtRelativeOffsetEffect;

use crate::schemas::drawing::main::CtGlowEffect;

use crate::schemas::drawing::main::CtFillOverlayEffect;

use crate::schemas::drawing::main::CtDuotoneEffect;

use crate::schemas::drawing::main::CtSoftEdgesEffect;

use crate::schemas::drawing::main::CtAlphaModulateEffect;

use crate::schemas::drawing::main::CtAlphaBiLevelEffect;

use crate::schemas::drawing::main::CtFillEffect;

use crate::schemas::drawing::main::CtColorChangeEffect;

use crate::schemas::drawing::main::CtAlphaFloorEffect;

use crate::schemas::drawing::main::CtBiLevelEffect;

use crate::schemas::drawing::main::CtTransformEffect;

use crate::schemas::drawing::main::CtEffectReference;

use crate::schemas::drawing::main::CtBlendEffect;

use crate::schemas::drawing::main::CtColorReplaceEffect;

use crate::schemas::drawing::main::CtAlphaInverseEffect;

use crate::schemas::drawing::main::CtInnerShadowEffect;

use crate::schemas::drawing::main::CtReflectionEffect;

use crate::schemas::drawing::main::CtAlphaOutsetEffect;

use crate::schemas::drawing::main::CtOuterShadowEffect;

/**
 * @author : zhilong.zhou
 * @description : CT_EffectContainer
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEffectContainer {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

    #[serde(rename(serialize = "a:cont", deserialize = "cont"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cont: Option<Box<CtEffectContainer>>,

    #[serde(rename(serialize = "a:effect", deserialize = "effect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<CtEffectReference>,

    #[serde(rename(serialize = "a:alphaBiLevel", deserialize = "alphaBiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_bi_level: Option<CtAlphaBiLevelEffect>,

    #[serde(rename(serialize = "a:alphaCeiling", deserialize = "alphaCeiling"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_ceiling: Option<CtAlphaCeilingEffect>,

    #[serde(rename(serialize = "a:alphaFloor", deserialize = "alphaFloor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_floor: Option<CtAlphaFloorEffect>,

    #[serde(rename(serialize = "a:alphaInv", deserialize = "alphaInv"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_inv: Option<CtAlphaInverseEffect>,

    #[serde(rename(serialize = "a:alphaMod", deserialize = "alphaMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_mod: Option<Box<CtAlphaModulateEffect>>,

    #[serde(rename(serialize = "a:alphaModFix", deserialize = "alphaModFix"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_mod_fix: Option<CtAlphaModulateFixedEffect>,

    #[serde(rename(serialize = "a:alphaOutset", deserialize = "alphaOutset"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_outset: Option<CtAlphaOutsetEffect>,

    #[serde(rename(serialize = "a:alphaRepl", deserialize = "alphaRepl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_repl: Option<CtAlphaReplaceEffect>,

    #[serde(rename(serialize = "a:biLevel", deserialize = "biLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bi_level: Option<CtBiLevelEffect>,

    #[serde(rename(serialize = "a:blend", deserialize = "blend"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blend: Option<Box<CtBlendEffect>>,

    #[serde(rename(serialize = "a:blur", deserialize = "blur"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<CtBlurEffect>,

    #[serde(rename(serialize = "a:clrChange", deserialize = "clrChange"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_change: Option<CtColorChangeEffect>,

    #[serde(rename(serialize = "a:clrRepl", deserialize = "clrRepl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_repl: Option<CtColorReplaceEffect>,

    #[serde(rename(serialize = "a:duotone", deserialize = "duotone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duotone: Option<CtDuotoneEffect>,

    #[serde(rename(serialize = "a:fill", deserialize = "fill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill: Option<Box<CtFillEffect>>,

    #[serde(rename(serialize = "a:fillOverlay", deserialize = "fillOverlay"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_overlay: Option<Box<CtFillOverlayEffect>>,

    #[serde(rename(serialize = "a:glow", deserialize = "glow"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub glow: Option<CtGlowEffect>,

    #[serde(rename(serialize = "a:grayscl", deserialize = "grayscl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscl: Option<CtGrayscaleEffect>,

    #[serde(rename(serialize = "a:hsl", deserialize = "hsl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsl: Option<CtHslEffect>,

    #[serde(rename(serialize = "a:innerShdw", deserialize = "innerShdw"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inner_shdw: Option<CtInnerShadowEffect>,

    #[serde(rename(serialize = "a:lum", deserialize = "lum"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lum: Option<CtLuminanceEffect>,

    #[serde(rename(serialize = "a:outerShdw", deserialize = "outerShdw"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outer_shdw: Option<CtOuterShadowEffect>,

    #[serde(rename(serialize = "a:prstShdw", deserialize = "prstShdw"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_shdw: Option<CtPresetShadowEffect>,

    #[serde(rename(serialize = "a:reflection", deserialize = "reflection"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reflection: Option<CtReflectionEffect>,

    #[serde(rename(serialize = "a:relOff", deserialize = "relOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rel_off: Option<CtRelativeOffsetEffect>,

    #[serde(rename(serialize = "a:softEdge", deserialize = "softEdge"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_edge: Option<CtSoftEdgesEffect>,

    #[serde(rename(serialize = "a:tint", deserialize = "tint"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<CtTintEffect>,

    #[serde(rename(serialize = "a:xfrm", deserialize = "xfrm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xfrm: Option<CtTransformEffect>,
}

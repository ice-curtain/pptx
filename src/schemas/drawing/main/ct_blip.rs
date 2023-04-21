use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAlphaCeilingEffect;

use crate::schemas::drawing::main::CtBlurEffect;

use crate::schemas::drawing::main::CtAlphaReplaceEffect;

use crate::schemas::drawing::main::CtLuminanceEffect;

use crate::schemas::drawing::main::CtColorChangeEffect;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtFillOverlayEffect;

use crate::schemas::drawing::main::CtAlphaFloorEffect;

use crate::schemas::drawing::main::CtDuotoneEffect;

use crate::schemas::drawing::main::CtGrayscaleEffect;

use crate::schemas::drawing::main::CtAlphaModulateFixedEffect;

use crate::schemas::drawing::main::CtColorReplaceEffect;

use crate::schemas::drawing::main::CtAlphaModulateEffect;

use crate::schemas::drawing::main::CtHslEffect;

use crate::schemas::drawing::main::CtAlphaInverseEffect;

use crate::schemas::drawing::main::CtBiLevelEffect;

use crate::schemas::drawing::main::CtAlphaBiLevelEffect;

use crate::schemas::drawing::main::CtTintEffect;

/**
 * @author : zhilong.zhou
 * @description : CT_Blip
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "a:blip", deserialize = "blip"))]

pub struct CtBlip {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "@r:embed", deserialize = "@embed"))]
    pub embed_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(serialize = "@r:link", deserialize = "@link"))]
    pub link_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@cstate")]
    pub cstate_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,

    #[serde(rename(serialize = "a:alphaBiLevel", deserialize = "alphaBiLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_bi_level: Option<Vec<CtAlphaBiLevelEffect>>,

    #[serde(rename(serialize = "a:alphaCeiling", deserialize = "alphaCeiling"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_ceiling: Option<Vec<CtAlphaCeilingEffect>>,

    #[serde(rename(serialize = "a:alphaFloor", deserialize = "alphaFloor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_floor: Option<Vec<CtAlphaFloorEffect>>,

    #[serde(rename(serialize = "a:alphaInv", deserialize = "alphaInv"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_inv: Option<Vec<CtAlphaInverseEffect>>,

    #[serde(rename(serialize = "a:alphaMod", deserialize = "alphaMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_mod: Option<Vec<CtAlphaModulateEffect>>,

    #[serde(rename(serialize = "a:alphaModFix", deserialize = "alphaModFix"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_mod_fix: Option<Vec<CtAlphaModulateFixedEffect>>,

    #[serde(rename(serialize = "a:alphaRepl", deserialize = "alphaRepl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_repl: Option<Vec<CtAlphaReplaceEffect>>,

    #[serde(rename(serialize = "a:biLevel", deserialize = "biLevel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bi_level: Option<Vec<CtBiLevelEffect>>,

    #[serde(rename(serialize = "a:blur", deserialize = "blur"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blur: Option<Vec<CtBlurEffect>>,

    #[serde(rename(serialize = "a:clrChange", deserialize = "clrChange"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_change: Option<Vec<CtColorChangeEffect>>,

    #[serde(rename(serialize = "a:clrRepl", deserialize = "clrRepl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_repl: Option<Vec<CtColorReplaceEffect>>,

    #[serde(rename(serialize = "a:duotone", deserialize = "duotone"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duotone: Option<Vec<CtDuotoneEffect>>,

    #[serde(rename(serialize = "a:fillOverlay", deserialize = "fillOverlay"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fill_overlay: Option<Vec<CtFillOverlayEffect>>,

    #[serde(rename(serialize = "a:grayscl", deserialize = "grayscl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscl: Option<Vec<CtGrayscaleEffect>>,

    #[serde(rename(serialize = "a:hsl", deserialize = "hsl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsl: Option<Vec<CtHslEffect>>,

    #[serde(rename(serialize = "a:lum", deserialize = "lum"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lum: Option<Vec<CtLuminanceEffect>>,

    #[serde(rename(serialize = "a:tint", deserialize = "tint"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Vec<CtTintEffect>>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGammaTransform;

use crate::schemas::drawing::main::CtPercentage;

use crate::schemas::drawing::main::CtPositivePercentage;

use crate::schemas::drawing::main::CtComplementTransform;

use crate::schemas::drawing::main::CtGrayscaleTransform;

use crate::schemas::drawing::main::CtInverseGammaTransform;

use crate::schemas::drawing::main::CtFixedPercentage;

use crate::schemas::drawing::main::CtAngle;

use crate::schemas::drawing::main::CtPositiveFixedAngle;

use crate::schemas::drawing::main::CtPositiveFixedPercentage;

use crate::schemas::drawing::main::CtInverseTransform;

/**
 * @author : zhilong.zhou
 * @description : CT_PresetColor
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPresetColor {
    #[serde(rename = "@val")]
    pub val_attr: String,

    #[serde(rename(serialize = "a:tint", deserialize = "tint"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tint: Option<Vec<CtPositiveFixedPercentage>>,

    #[serde(rename(serialize = "a:shade", deserialize = "shade"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shade: Option<Vec<CtPositiveFixedPercentage>>,

    #[serde(rename(serialize = "a:comp", deserialize = "comp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comp: Option<Vec<CtComplementTransform>>,

    #[serde(rename(serialize = "a:inv", deserialize = "inv"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv: Option<Vec<CtInverseTransform>>,

    #[serde(rename(serialize = "a:gray", deserialize = "gray"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gray: Option<Vec<CtGrayscaleTransform>>,

    #[serde(rename(serialize = "a:alpha", deserialize = "alpha"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<Vec<CtPositiveFixedPercentage>>,

    #[serde(rename(serialize = "a:alphaOff", deserialize = "alphaOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_off: Option<Vec<CtFixedPercentage>>,

    #[serde(rename(serialize = "a:alphaMod", deserialize = "alphaMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha_mod: Option<Vec<CtPositivePercentage>>,

    #[serde(rename(serialize = "a:hue", deserialize = "hue"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue: Option<Vec<CtPositiveFixedAngle>>,

    #[serde(rename(serialize = "a:hueOff", deserialize = "hueOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue_off: Option<Vec<CtAngle>>,

    #[serde(rename(serialize = "a:hueMod", deserialize = "hueMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hue_mod: Option<Vec<CtPositivePercentage>>,

    #[serde(rename(serialize = "a:sat", deserialize = "sat"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:satOff", deserialize = "satOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat_off: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:satMod", deserialize = "satMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sat_mod: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:lum", deserialize = "lum"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lum: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:lumOff", deserialize = "lumOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lum_off: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:lumMod", deserialize = "lumMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lum_mod: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:red", deserialize = "red"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:redOff", deserialize = "redOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_off: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:redMod", deserialize = "redMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red_mod: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:green", deserialize = "green"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:greenOff", deserialize = "greenOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_off: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:greenMod", deserialize = "greenMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_mod: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:blue", deserialize = "blue"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:blueOff", deserialize = "blueOff"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_off: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:blueMod", deserialize = "blueMod"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_mod: Option<Vec<CtPercentage>>,

    #[serde(rename(serialize = "a:gamma", deserialize = "gamma"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gamma: Option<Vec<CtGammaTransform>>,

    #[serde(rename(serialize = "a:invGamma", deserialize = "invGamma"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inv_gamma: Option<Vec<CtInverseGammaTransform>>,
}

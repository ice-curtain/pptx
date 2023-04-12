use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScRgbColor;

use crate::schemas::drawing::main::CtSystemColor;

use crate::schemas::drawing::main::CtSchemeColor;

use crate::schemas::drawing::main::CtSRgbColor;

use crate::schemas::drawing::main::CtHslColor;

use crate::schemas::drawing::main::CtPresetColor;

/**
 * @author : zhilong.zhou
 * @description : CT_GlowEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGlowEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rad")]
    pub rad_attr: Option<String>,

    #[serde(rename(serialize = "a:scrgbClr", deserialize = "scrgbClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrgb_clr: Option<CtScRgbColor>,

    #[serde(rename(serialize = "a:srgbClr", deserialize = "srgbClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srgb_clr: Option<CtSRgbColor>,

    #[serde(rename(serialize = "a:hslClr", deserialize = "hslClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsl_clr: Option<CtHslColor>,

    #[serde(rename(serialize = "a:sysClr", deserialize = "sysClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_clr: Option<CtSystemColor>,

    #[serde(rename(serialize = "a:schemeClr", deserialize = "schemeClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_clr: Option<CtSchemeColor>,

    #[serde(rename(serialize = "a:prstClr", deserialize = "prstClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_clr: Option<CtPresetColor>,
}

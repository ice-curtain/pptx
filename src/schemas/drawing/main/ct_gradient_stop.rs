use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScRgbColor;

use crate::schemas::drawing::main::CtSystemColor;

use crate::schemas::drawing::main::CtSchemeColor;

use crate::schemas::drawing::main::CtSRgbColor;

use crate::schemas::drawing::main::CtHslColor;

use crate::schemas::drawing::main::CtPresetColor;

/**
 * @author : zhilong.zhou
 * @description : CT_GradientStop
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGradientStop {
    #[serde(rename = "@pos")]
    pub pos_attr: String,

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

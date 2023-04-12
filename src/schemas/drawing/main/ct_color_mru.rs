use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScRgbColor;

use crate::schemas::drawing::main::CtSystemColor;

use crate::schemas::drawing::main::CtSchemeColor;

use crate::schemas::drawing::main::CtSRgbColor;

use crate::schemas::drawing::main::CtHslColor;

use crate::schemas::drawing::main::CtPresetColor;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorMRU
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColorMru {
    #[serde(rename(serialize = "a:scrgbClr", deserialize = "scrgbClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrgb_clr: Option<Vec<CtScRgbColor>>,

    #[serde(rename(serialize = "a:srgbClr", deserialize = "srgbClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srgb_clr: Option<Vec<CtSRgbColor>>,

    #[serde(rename(serialize = "a:hslClr", deserialize = "hslClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsl_clr: Option<Vec<CtHslColor>>,

    #[serde(rename(serialize = "a:sysClr", deserialize = "sysClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_clr: Option<Vec<CtSystemColor>>,

    #[serde(rename(serialize = "a:schemeClr", deserialize = "schemeClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_clr: Option<Vec<CtSchemeColor>>,

    #[serde(rename(serialize = "a:prstClr", deserialize = "prstClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_clr: Option<Vec<CtPresetColor>>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScRgbColor;

use crate::schemas::drawing::main::CtSystemColor;

use crate::schemas::drawing::main::CtSchemeColor;

use crate::schemas::drawing::main::CtSRgbColor;

use crate::schemas::drawing::main::CtHslColor;

use crate::schemas::drawing::main::CtPresetColor;

/**
 * @author : zhilong.zhou
 * @description : CT_Colors
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColors {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@meth")]
    pub meth_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hueDir")]
    pub hue_dir_attr: Option<String>,

    #[serde(rename(serialize = "scrgbClr", deserialize = "scrgbClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrgb_clr: Option<Vec<CtScRgbColor>>,

    #[serde(rename(serialize = "srgbClr", deserialize = "srgbClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srgb_clr: Option<Vec<CtSRgbColor>>,

    #[serde(rename(serialize = "hslClr", deserialize = "hslClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hsl_clr: Option<Vec<CtHslColor>>,

    #[serde(rename(serialize = "sysClr", deserialize = "sysClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_clr: Option<Vec<CtSystemColor>>,

    #[serde(rename(serialize = "schemeClr", deserialize = "schemeClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheme_clr: Option<Vec<CtSchemeColor>>,

    #[serde(rename(serialize = "prstClr", deserialize = "prstClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_clr: Option<Vec<CtPresetColor>>,
}

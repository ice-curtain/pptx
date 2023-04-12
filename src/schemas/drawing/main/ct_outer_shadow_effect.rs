use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScRgbColor;

use crate::schemas::drawing::main::CtSystemColor;

use crate::schemas::drawing::main::CtSchemeColor;

use crate::schemas::drawing::main::CtSRgbColor;

use crate::schemas::drawing::main::CtHslColor;

use crate::schemas::drawing::main::CtPresetColor;

/**
 * @author : zhilong.zhou
 * @description : CT_OuterShadowEffect
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOuterShadowEffect {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@blurRad")]
    pub blur_rad_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dist")]
    pub dist_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dir")]
    pub dir_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sx")]
    pub sx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sy")]
    pub sy_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@kx")]
    pub kx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ky")]
    pub ky_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@algn")]
    pub algn_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rotWithShape")]
    pub rot_with_shape_attr: Option<String>,

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

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtScRgbColor;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtSystemColor;

use crate::schemas::drawing::main::CtFontCollection;

use crate::schemas::drawing::main::CtSchemeColor;

use crate::schemas::drawing::main::CtSRgbColor;

use crate::schemas::drawing::main::CtHslColor;

use crate::schemas::drawing::main::CtPresetColor;

use crate::schemas::drawing::main::CtFontReference;

/**
 * @author : zhilong.zhou
 * @description : CT_TableStyleTextStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableStyleTextStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@b")]
    pub b_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@i")]
    pub i_attr: Option<String>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,

    #[serde(rename(serialize = "a:font", deserialize = "font"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font: Option<CtFontCollection>,

    #[serde(rename(serialize = "a:fontRef", deserialize = "fontRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_ref: Option<CtFontReference>,

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

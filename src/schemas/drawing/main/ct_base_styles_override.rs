use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtFontScheme;

use crate::schemas::drawing::main::CtColorScheme;

use crate::schemas::drawing::main::CtStyleMatrix;

/**
 * @author : zhilong.zhou
 * @description : CT_BaseStylesOverride
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "a:themeOverride", deserialize = "themeOverride"))]

pub struct CtBaseStylesOverride {
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

    #[serde(rename(serialize = "a:clrScheme", deserialize = "clrScheme"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_scheme: Option<CtColorScheme>,

    #[serde(rename(serialize = "a:fontScheme", deserialize = "fontScheme"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_scheme: Option<CtFontScheme>,

    #[serde(rename(serialize = "a:fmtScheme", deserialize = "fmtScheme"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmt_scheme: Option<Box<CtStyleMatrix>>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtColorSchemeList;

use crate::schemas::drawing::main::CtCustomColorList;

use crate::schemas::drawing::main::CtBaseStyles;

use crate::schemas::drawing::main::CtObjectStyleDefaults;

/**
 * @author : zhilong.zhou
 * @description : CT_OfficeStyleSheet
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "a:theme", deserialize = "theme"))]

pub struct CtOfficeStyleSheet {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,

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

    #[serde(rename(serialize = "a:themeElements", deserialize = "themeElements"))]
    pub theme_elements: Box<CtBaseStyles>,

    #[serde(rename(serialize = "a:objectDefaults", deserialize = "objectDefaults"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_defaults: Option<Box<CtObjectStyleDefaults>>,

    #[serde(rename(serialize = "a:extraClrSchemeLst", deserialize = "extraClrSchemeLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_clr_scheme_lst: Option<CtColorSchemeList>,

    #[serde(rename(serialize = "a:custClrLst", deserialize = "custClrLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_clr_lst: Option<CtCustomColorList>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

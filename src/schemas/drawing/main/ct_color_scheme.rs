use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtColor;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_ColorScheme
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtColorScheme {
    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename(serialize = "a:dk1", deserialize = "dk1"))]
    pub dk1: CtColor,

    #[serde(rename(serialize = "a:lt1", deserialize = "lt1"))]
    pub lt1: CtColor,

    #[serde(rename(serialize = "a:dk2", deserialize = "dk2"))]
    pub dk2: CtColor,

    #[serde(rename(serialize = "a:lt2", deserialize = "lt2"))]
    pub lt2: CtColor,

    #[serde(rename(serialize = "a:accent1", deserialize = "accent1"))]
    pub accent1: CtColor,

    #[serde(rename(serialize = "a:accent2", deserialize = "accent2"))]
    pub accent2: CtColor,

    #[serde(rename(serialize = "a:accent3", deserialize = "accent3"))]
    pub accent3: CtColor,

    #[serde(rename(serialize = "a:accent4", deserialize = "accent4"))]
    pub accent4: CtColor,

    #[serde(rename(serialize = "a:accent5", deserialize = "accent5"))]
    pub accent5: CtColor,

    #[serde(rename(serialize = "a:accent6", deserialize = "accent6"))]
    pub accent6: CtColor,

    #[serde(rename(serialize = "a:hlink", deserialize = "hlink"))]
    pub hlink: CtColor,

    #[serde(rename(serialize = "a:folHlink", deserialize = "folHlink"))]
    pub fol_hlink: CtColor,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

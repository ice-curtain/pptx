use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtAdjLst;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Shape
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShape {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rot")]
    pub rot_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@blip")]
    pub blip_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@zOrderOff")]
    pub z_order_off_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hideGeom")]
    pub hide_geom_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lkTxEntry")]
    pub lk_tx_entry_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@blipPhldr")]
    pub blip_phldr_attr: Option<String>,

    #[serde(rename(serialize = "adjLst", deserialize = "adjLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adj_lst: Option<CtAdjLst>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

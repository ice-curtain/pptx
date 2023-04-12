use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBevel;

use crate::schemas::drawing::main::CtColor;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Shape3D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShape3D {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@z")]
    pub z_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@extrusionH")]
    pub extrusion_h_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@contourW")]
    pub contour_w_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@prstMaterial")]
    pub prst_material_attr: Option<String>,

    #[serde(rename(serialize = "a:bevelT", deserialize = "bevelT"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bevel_t: Option<CtBevel>,

    #[serde(rename(serialize = "a:bevelB", deserialize = "bevelB"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bevel_b: Option<CtBevel>,

    #[serde(rename(serialize = "a:extrusionClr", deserialize = "extrusionClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extrusion_clr: Option<CtColor>,

    #[serde(rename(serialize = "a:contourClr", deserialize = "contourClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contour_clr: Option<CtColor>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

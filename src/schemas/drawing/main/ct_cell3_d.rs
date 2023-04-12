use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBevel;

use crate::schemas::drawing::main::CtLightRig;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Cell3D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCell3D {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@prstMaterial")]
    pub prst_material_attr: Option<String>,

    #[serde(rename(serialize = "a:bevel", deserialize = "bevel"))]
    pub bevel: CtBevel,

    #[serde(rename(serialize = "a:lightRig", deserialize = "lightRig"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub light_rig: Option<CtLightRig>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

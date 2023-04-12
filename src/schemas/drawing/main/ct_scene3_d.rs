use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtLightRig;

use crate::schemas::drawing::main::CtCamera;

use crate::schemas::drawing::main::CtBackdrop;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_Scene3D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtScene3D {
    #[serde(rename(serialize = "a:camera", deserialize = "camera"))]
    pub camera: CtCamera,

    #[serde(rename(serialize = "a:lightRig", deserialize = "lightRig"))]
    pub light_rig: CtLightRig,

    #[serde(rename(serialize = "a:backdrop", deserialize = "backdrop"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backdrop: Option<CtBackdrop>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

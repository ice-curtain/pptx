use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtVector3D;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtPoint3D;

/**
 * @author : zhilong.zhou
 * @description : CT_Backdrop
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBackdrop {
    #[serde(rename(serialize = "a:anchor", deserialize = "anchor"))]
    pub anchor: CtPoint3D,

    #[serde(rename(serialize = "a:norm", deserialize = "norm"))]
    pub norm: CtVector3D,

    #[serde(rename(serialize = "a:up", deserialize = "up"))]
    pub up: CtVector3D,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

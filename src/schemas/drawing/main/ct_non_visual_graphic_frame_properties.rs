use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGraphicalObjectFrameLocking;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NonVisualGraphicFrameProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNonVisualGraphicFrameProperties {
    #[serde(rename(serialize = "a:graphicFrameLocks", deserialize = "graphicFrameLocks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic_frame_locks: Option<CtGraphicalObjectFrameLocking>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

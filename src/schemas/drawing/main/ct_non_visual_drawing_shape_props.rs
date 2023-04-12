use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtShapeLocking;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NonVisualDrawingShapeProps
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNonVisualDrawingShapeProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@txBox")]
    pub tx_box_attr: Option<String>,

    #[serde(rename(serialize = "a:spLocks", deserialize = "spLocks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_locks: Option<CtShapeLocking>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

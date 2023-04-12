use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGroupLocking;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NonVisualGroupDrawingShapeProps
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNonVisualGroupDrawingShapeProps {
    #[serde(rename(serialize = "a:grpSpLocks", deserialize = "grpSpLocks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_sp_locks: Option<CtGroupLocking>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

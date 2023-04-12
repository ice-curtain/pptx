use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtContentPartLocking;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NonVisualContentPartProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNonVisualContentPartProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@isComment")]
    pub is_comment_attr: Option<String>,

    #[serde(rename(serialize = "a:cpLocks", deserialize = "cpLocks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cp_locks: Option<CtContentPartLocking>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

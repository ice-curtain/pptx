use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtConnection;

use crate::schemas::drawing::main::CtConnectorLocking;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_NonVisualConnectorProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNonVisualConnectorProperties {
    #[serde(rename(serialize = "a:cxnSpLocks", deserialize = "cxnSpLocks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn_sp_locks: Option<CtConnectorLocking>,

    #[serde(rename(serialize = "a:stCxn", deserialize = "stCxn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub st_cxn: Option<CtConnection>,

    #[serde(rename(serialize = "a:endCxn", deserialize = "endCxn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cxn: Option<CtConnection>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

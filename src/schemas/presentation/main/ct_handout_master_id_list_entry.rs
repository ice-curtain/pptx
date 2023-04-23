use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_HandoutMasterIdListEntry
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHandoutMasterIdListEntry {
    #[serde(rename(serialize = "@r:id", deserialize = "@id"))]
    pub r_id_attr: String,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

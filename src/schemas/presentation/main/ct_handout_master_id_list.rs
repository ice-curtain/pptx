use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtHandoutMasterIdListEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_HandoutMasterIdList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHandoutMasterIdList {
    #[serde(rename(serialize = "p:handoutMasterId", deserialize = "handoutMasterId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handout_master_id: Option<CtHandoutMasterIdListEntry>,
}

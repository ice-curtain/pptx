use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtSlideMasterIdListEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideMasterIdList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideMasterIdList {
    #[serde(rename(serialize = "p:sldMasterId", deserialize = "sldMasterId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_master_id: Option<Vec<CtSlideMasterIdListEntry>>,
}

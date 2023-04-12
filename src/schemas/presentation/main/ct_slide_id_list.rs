use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtSlideIdListEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideIdList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideIdList {
    #[serde(rename(serialize = "p:sldId", deserialize = "sldId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_id: Option<Vec<CtSlideIdListEntry>>,
}

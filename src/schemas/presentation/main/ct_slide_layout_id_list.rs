use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtSlideLayoutIdListEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideLayoutIdList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideLayoutIdList {
    #[serde(rename(serialize = "p:sldLayoutId", deserialize = "sldLayoutId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_layout_id: Option<Vec<CtSlideLayoutIdListEntry>>,
}

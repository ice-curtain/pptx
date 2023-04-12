use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtOutlineViewSlideEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_OutlineViewSlideList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtOutlineViewSlideList {
    #[serde(rename(serialize = "p:sld", deserialize = "sld"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld: Option<Vec<CtOutlineViewSlideEntry>>,
}

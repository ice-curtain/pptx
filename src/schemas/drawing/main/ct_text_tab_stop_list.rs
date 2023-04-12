use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextTabStop;

/**
 * @author : zhilong.zhou
 * @description : CT_TextTabStopList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextTabStopList {
    #[serde(rename(serialize = "a:tab", deserialize = "tab"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tab: Option<Vec<CtTextTabStop>>,
}

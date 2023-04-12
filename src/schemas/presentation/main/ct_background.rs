use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtStyleMatrixReference;

use crate::schemas::presentation::main::CtBackgroundProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_Background
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBackground {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bwMode")]
    pub bw_mode_attr: Option<String>,

    #[serde(rename(serialize = "p:bgPr", deserialize = "bgPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_pr: Option<Box<CtBackgroundProperties>>,

    #[serde(rename(serialize = "p:bgRef", deserialize = "bgRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg_ref: Option<CtStyleMatrixReference>,
}

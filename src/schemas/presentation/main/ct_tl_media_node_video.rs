use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonMediaNodeData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLMediaNodeVideo
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlMediaNodeVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fullScrn")]
    pub full_scrn_attr: Option<String>,

    #[serde(rename(serialize = "p:cMediaNode", deserialize = "cMediaNode"))]
    pub c_media_node: CtTlCommonMediaNodeData,
}

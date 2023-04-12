use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlCommonMediaNodeData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLMediaNodeAudio
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlMediaNodeAudio {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@isNarration")]
    pub is_narration_attr: Option<String>,

    #[serde(rename(serialize = "p:cMediaNode", deserialize = "cMediaNode"))]
    pub c_media_node: CtTlCommonMediaNodeData,
}

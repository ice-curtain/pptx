use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTimeTargetElement;

use crate::schemas::presentation::main::CtTlCommonTimeNodeData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLCommonMediaNodeData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlCommonMediaNodeData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@vol")]
    pub vol_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@mute")]
    pub mute_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@numSld")]
    pub num_sld_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showWhenStopped")]
    pub show_when_stopped_attr: Option<String>,

    #[serde(rename(serialize = "p:cTn", deserialize = "cTn"))]
    pub c_tn: CtTlCommonTimeNodeData,

    #[serde(rename(serialize = "p:tgtEl", deserialize = "tgtEl"))]
    pub tgt_el: CtTlTimeTargetElement,
}

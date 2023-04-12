use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlBehaviorAttributeNameList;

use crate::schemas::presentation::main::CtTlTimeTargetElement;

use crate::schemas::presentation::main::CtTlCommonTimeNodeData;

/**
 * @author : zhilong.zhou
 * @description : CT_TLCommonBehaviorData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlCommonBehaviorData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@additive")]
    pub additive_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@accumulate")]
    pub accumulate_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xfrmType")]
    pub xfrm_type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@from")]
    pub from_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@to")]
    pub to_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@by")]
    pub by_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rctx")]
    pub rctx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@override")]
    pub override_attr: Option<String>,

    #[serde(rename(serialize = "p:cTn", deserialize = "cTn"))]
    pub c_tn: CtTlCommonTimeNodeData,

    #[serde(rename(serialize = "p:tgtEl", deserialize = "tgtEl"))]
    pub tgt_el: CtTlTimeTargetElement,

    #[serde(rename(serialize = "p:attrNameLst", deserialize = "attrNameLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_name_lst: Option<CtTlBehaviorAttributeNameList>,
}

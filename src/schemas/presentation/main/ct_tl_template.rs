use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTimeNodeList;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTemplate
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTemplate {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lvl")]
    pub lvl_attr: Option<String>,

    #[serde(rename(serialize = "p:tnLst", deserialize = "tnLst"))]
    pub tn_lst: CtTimeNodeList,
}

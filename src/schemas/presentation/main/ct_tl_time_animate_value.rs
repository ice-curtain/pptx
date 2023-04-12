use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlAnimVariant;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeAnimateValue
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeAnimateValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tm")]
    pub tm_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fmla")]
    pub fmla_attr: Option<String>,

    #[serde(rename(serialize = "p:val", deserialize = "val"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<CtTlAnimVariant>,
}

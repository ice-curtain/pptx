use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtIndexRange;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTextTargetElement
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTextTargetElement {
    #[serde(rename(serialize = "p:charRg", deserialize = "charRg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_rg: Option<CtIndexRange>,

    #[serde(rename(serialize = "p:pRg", deserialize = "pRg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_rg: Option<CtIndexRange>,
}

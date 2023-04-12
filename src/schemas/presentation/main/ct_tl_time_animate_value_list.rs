use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTimeAnimateValue;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTimeAnimateValueList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTimeAnimateValueList {
    #[serde(rename(serialize = "p:tav", deserialize = "tav"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tav: Option<Vec<CtTlTimeAnimateValue>>,
}

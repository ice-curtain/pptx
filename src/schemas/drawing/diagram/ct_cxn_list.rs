use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtCxn;

/**
 * @author : zhilong.zhou
 * @description : CT_CxnList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCxnList {
    #[serde(rename(serialize = "cxn", deserialize = "cxn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn: Option<Vec<CtCxn>>,
}

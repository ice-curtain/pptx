use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtStrRef;

/**
 * @author : zhilong.zhou
 * @description : CT_SerTx
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSerTx {
    #[serde(rename(serialize = "strRef", deserialize = "strRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_ref: Option<CtStrRef>,

    #[serde(rename(serialize = "v", deserialize = "v"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub v: Option<String>,
}

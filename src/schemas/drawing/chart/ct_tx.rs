use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtStrRef;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_Tx
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTx {
    #[serde(rename(serialize = "strRef", deserialize = "strRef"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub str_ref: Option<CtStrRef>,

    #[serde(rename(serialize = "rich", deserialize = "rich"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rich: Option<Box<CtTextBody>>,
}

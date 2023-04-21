use serde::{Deserialize, Serialize};
use crate::schemas::drawing::main::CtTable;

/**
 * @author : zhilong.zhou
 * @description : CT_GraphicalObjectData
 */


//schema允许它包含任意元素，但是我们只考虑出现过的结构
#[derive(Serialize, Deserialize, Debug)]
pub struct CtGraphicalObjectData {
    #[serde(rename = "@uri")]
    pub uri_attr: String,

    #[serde(rename(serialize = "a:tbl", deserialize = "tbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    tbl:Option<CtTable>
}

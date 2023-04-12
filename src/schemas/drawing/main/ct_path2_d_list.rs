use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPath2D;

/**
 * @author : zhilong.zhou
 * @description : CT_Path2DList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPath2DList {
    #[serde(rename(serialize = "a:path", deserialize = "path"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<Vec<CtPath2D>>,
}

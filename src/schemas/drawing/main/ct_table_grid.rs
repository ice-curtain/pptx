use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTableCol;

/**
 * @author : zhilong.zhou
 * @description : CT_TableGrid
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableGrid {
    #[serde(rename(serialize = "a:gridCol", deserialize = "gridCol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_col: Option<Vec<CtTableCol>>,
}

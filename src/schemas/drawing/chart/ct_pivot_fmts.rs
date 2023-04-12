use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtPivotFmt;

/**
 * @author : zhilong.zhou
 * @description : CT_PivotFmts
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPivotFmts {
    #[serde(rename(serialize = "pivotFmt", deserialize = "pivotFmt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_fmt: Option<Vec<CtPivotFmt>>,
}

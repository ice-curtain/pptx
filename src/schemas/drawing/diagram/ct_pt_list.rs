use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtPt;

/**
 * @author : zhilong.zhou
 * @description : CT_PtList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPtList {
    #[serde(rename(serialize = "pt", deserialize = "pt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<Vec<CtPt>>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtAdj;

/**
 * @author : zhilong.zhou
 * @description : CT_AdjLst
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAdjLst {
    #[serde(rename(serialize = "adj", deserialize = "adj"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adj: Option<Vec<CtAdj>>,
}

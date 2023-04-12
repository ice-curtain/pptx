use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtUnsignedInt;

/**
 * @author : zhilong.zhou
 * @description : CT_CustSplit
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustSplit {
    #[serde(rename(serialize = "secondPiePt", deserialize = "secondPiePt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_pie_pt: Option<Vec<CtUnsignedInt>>,
}

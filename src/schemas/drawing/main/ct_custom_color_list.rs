use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtCustomColor;

/**
 * @author : zhilong.zhou
 * @description : CT_CustomColorList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustomColorList {
    #[serde(rename(serialize = "a:custClr", deserialize = "custClr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_clr: Option<Vec<CtCustomColor>>,
}

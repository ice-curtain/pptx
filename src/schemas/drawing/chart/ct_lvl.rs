use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtStrVal;

/**
 * @author : zhilong.zhou
 * @description : CT_Lvl
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtLvl {
    #[serde(rename(serialize = "pt", deserialize = "pt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<Vec<CtStrVal>>,
}

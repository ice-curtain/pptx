use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtCustomShow;

/**
 * @author : zhilong.zhou
 * @description : CT_CustomShowList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustomShowList {
    #[serde(rename(serialize = "p:custShow", deserialize = "custShow"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_show: Option<Vec<CtCustomShow>>,
}

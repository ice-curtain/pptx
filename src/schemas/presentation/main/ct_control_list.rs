use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtControl;

/**
 * @author : zhilong.zhou
 * @description : CT_ControlList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtControlList {
    #[serde(rename(serialize = "p:control", deserialize = "control"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control: Option<Vec<CtControl>>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtDashStop;

/**
 * @author : zhilong.zhou
 * @description : CT_DashStopList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtDashStopList {
    #[serde(rename(serialize = "a:ds", deserialize = "ds"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ds: Option<Vec<CtDashStop>>,
}

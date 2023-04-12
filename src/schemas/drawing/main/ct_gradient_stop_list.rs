use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGradientStop;

/**
 * @author : zhilong.zhou
 * @description : CT_GradientStopList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGradientStopList {
    #[serde(rename(serialize = "a:gs", deserialize = "gs"))]
    pub gs: Vec<CtGradientStop>,
}

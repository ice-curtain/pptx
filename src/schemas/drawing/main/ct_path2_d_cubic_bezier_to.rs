use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAdjPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_Path2DCubicBezierTo
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPath2DCubicBezierTo {
    #[serde(rename(serialize = "a:pt", deserialize = "pt"))]
    pub pt: Vec<CtAdjPoint2D>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAdjPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_Path2DMoveTo
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPath2DMoveTo {
    #[serde(rename(serialize = "a:pt", deserialize = "pt"))]
    pub pt: CtAdjPoint2D,
}

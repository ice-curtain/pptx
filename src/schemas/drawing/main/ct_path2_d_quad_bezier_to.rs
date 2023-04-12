use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAdjPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_Path2DQuadBezierTo
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPath2DQuadBezierTo {
    #[serde(rename(serialize = "a:pt", deserialize = "pt"))]
    pub pt: Vec<CtAdjPoint2D>,
}

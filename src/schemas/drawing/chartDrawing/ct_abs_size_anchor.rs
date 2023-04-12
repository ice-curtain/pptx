use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtMarker;

use crate::schemas::drawing::main::CtPositiveSize2D;

/**
 * @author : zhilong.zhou
 * @description : CT_AbsSizeAnchor
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAbsSizeAnchor {
    #[serde(rename(serialize = "from", deserialize = "from"))]
    pub from: Vec<CtMarker>,

    #[serde(rename(serialize = "ext", deserialize = "ext"))]
    pub ext: Vec<CtPositiveSize2D>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chartDrawing::CtMarker;

/**
 * @author : zhilong.zhou
 * @description : CT_RelSizeAnchor
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtRelSizeAnchor {
    #[serde(rename(serialize = "from", deserialize = "from"))]
    pub from: Vec<CtMarker>,

    #[serde(rename(serialize = "to", deserialize = "to"))]
    pub to: Vec<CtMarker>,
}

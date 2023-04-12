use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtRatio;

/**
 * @author : zhilong.zhou
 * @description : CT_Scale2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtScale2D {
    #[serde(rename(serialize = "a:sx", deserialize = "sx"))]
    pub sx: CtRatio,

    #[serde(rename(serialize = "a:sy", deserialize = "sy"))]
    pub sy: CtRatio,
}

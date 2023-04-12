use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAdjPoint2D;

/**
 * @author : zhilong.zhou
 * @description : CT_ConnectionSite
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConnectionSite {
    #[serde(rename = "@ang")]
    pub ang_attr: String,

    #[serde(rename(serialize = "a:pos", deserialize = "pos"))]
    pub pos: CtAdjPoint2D,
}

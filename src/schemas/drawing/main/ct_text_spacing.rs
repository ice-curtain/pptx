use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextSpacingPoint;

use crate::schemas::drawing::main::CtTextSpacingPercent;

/**
 * @author : zhilong.zhou
 * @description : CT_TextSpacing
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextSpacing {
    #[serde(rename(serialize = "a:spcPct", deserialize = "spcPct"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spc_pct: Option<CtTextSpacingPercent>,

    #[serde(rename(serialize = "a:spcPts", deserialize = "spcPts"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spc_pts: Option<CtTextSpacingPoint>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_BandFmt
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBandFmt {
    #[serde(rename(serialize = "idx", deserialize = "idx"))]
    pub idx: CtUnsignedInt,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,
}

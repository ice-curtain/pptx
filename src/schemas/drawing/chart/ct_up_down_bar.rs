use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_UpDownBar
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtUpDownBar {
    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,
}

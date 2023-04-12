use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGeomGuide;

/**
 * @author : zhilong.zhou
 * @description : CT_GeomGuideList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGeomGuideList {
    #[serde(rename(serialize = "a:gd", deserialize = "gd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gd: Option<Vec<CtGeomGuide>>,
}

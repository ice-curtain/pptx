use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtGeomGuideList;

/**
 * @author : zhilong.zhou
 * @description : CT_PresetGeometry2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPresetGeometry2D {
    #[serde(rename = "@prst")]
    pub prst_attr: String,

    #[serde(rename(serialize = "a:avLst", deserialize = "avLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av_lst: Option<CtGeomGuideList>,
}

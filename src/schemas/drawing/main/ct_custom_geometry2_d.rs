use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPath2DList;

use crate::schemas::drawing::main::CtGeomRect;

use crate::schemas::drawing::main::CtConnectionSiteList;

use crate::schemas::drawing::main::CtAdjustHandleList;

use crate::schemas::drawing::main::CtGeomGuideList;

/**
 * @author : zhilong.zhou
 * @description : CT_CustomGeometry2D
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCustomGeometry2D {
    #[serde(rename(serialize = "a:avLst", deserialize = "avLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub av_lst: Option<CtGeomGuideList>,

    #[serde(rename(serialize = "a:gdLst", deserialize = "gdLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gd_lst: Option<CtGeomGuideList>,

    #[serde(rename(serialize = "a:ahLst", deserialize = "ahLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ah_lst: Option<CtAdjustHandleList>,

    #[serde(rename(serialize = "a:cxnLst", deserialize = "cxnLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn_lst: Option<CtConnectionSiteList>,

    #[serde(rename(serialize = "a:rect", deserialize = "rect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rect: Option<CtGeomRect>,

    #[serde(rename(serialize = "a:pathLst", deserialize = "pathLst"))]
    pub path_lst: CtPath2DList,
}

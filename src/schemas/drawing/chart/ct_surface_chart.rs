use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtBandFmts;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtSurfaceSer;

/**
 * @author : zhilong.zhou
 * @description : CT_SurfaceChart
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSurfaceChart {
    #[serde(rename(serialize = "axId", deserialize = "axId"))]
    pub ax_id: Vec<CtUnsignedInt>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "wireframe", deserialize = "wireframe"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wireframe: Option<CtBoolean>,

    #[serde(rename(serialize = "ser", deserialize = "ser"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ser: Option<Vec<CtSurfaceSer>>,

    #[serde(rename(serialize = "bandFmts", deserialize = "bandFmts"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub band_fmts: Option<Box<CtBandFmts>>,
}

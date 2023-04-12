use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtSerTx;

use crate::schemas::drawing::chart::CtMarker;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtDPt;

use crate::schemas::drawing::chart::CtNumDataSource;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtAxDataSource;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_RadarSer
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtRadarSer {
    #[serde(rename(serialize = "marker", deserialize = "marker"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<Box<CtMarker>>,

    #[serde(rename(serialize = "dPt", deserialize = "dPt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_pt: Option<Vec<CtDPt>>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,

    #[serde(rename(serialize = "cat", deserialize = "cat"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<CtAxDataSource>,

    #[serde(rename(serialize = "val", deserialize = "val"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<CtNumDataSource>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "idx", deserialize = "idx"))]
    pub idx: CtUnsignedInt,

    #[serde(rename(serialize = "order", deserialize = "order"))]
    pub order: CtUnsignedInt,

    #[serde(rename(serialize = "tx", deserialize = "tx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<CtSerTx>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,
}

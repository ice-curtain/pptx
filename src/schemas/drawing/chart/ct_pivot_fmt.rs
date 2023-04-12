use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtDLbl;

use crate::schemas::drawing::chart::CtMarker;

use crate::schemas::drawing::main::CtTextBody;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_PivotFmt
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPivotFmt {
    #[serde(rename(serialize = "idx", deserialize = "idx"))]
    pub idx: CtUnsignedInt,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "txPr", deserialize = "txPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_pr: Option<Box<CtTextBody>>,

    #[serde(rename(serialize = "marker", deserialize = "marker"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marker: Option<Box<CtMarker>>,

    #[serde(rename(serialize = "dLbl", deserialize = "dLbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbl: Option<Box<CtDLbl>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

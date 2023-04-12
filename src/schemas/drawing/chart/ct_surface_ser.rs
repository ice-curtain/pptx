use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtSerTx;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtNumDataSource;

use crate::schemas::drawing::chart::CtAxDataSource;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_SurfaceSer
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSurfaceSer {
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

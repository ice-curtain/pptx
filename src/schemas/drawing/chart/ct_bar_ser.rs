use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtShape;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtErrBars;

use crate::schemas::drawing::chart::CtSerTx;

use crate::schemas::drawing::chart::CtPictureOptions;

use crate::schemas::drawing::chart::CtDLbls;

use crate::schemas::drawing::chart::CtDPt;

use crate::schemas::drawing::chart::CtNumDataSource;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtAxDataSource;

use crate::schemas::drawing::chart::CtTrendline;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_BarSer
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBarSer {
    #[serde(rename(serialize = "invertIfNegative", deserialize = "invertIfNegative"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invert_if_negative: Option<CtBoolean>,

    #[serde(rename(serialize = "pictureOptions", deserialize = "pictureOptions"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_options: Option<CtPictureOptions>,

    #[serde(rename(serialize = "dPt", deserialize = "dPt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_pt: Option<Vec<CtDPt>>,

    #[serde(rename(serialize = "dLbls", deserialize = "dLbls"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d_lbls: Option<Box<CtDLbls>>,

    #[serde(rename(serialize = "trendline", deserialize = "trendline"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendline: Option<Vec<CtTrendline>>,

    #[serde(rename(serialize = "errBars", deserialize = "errBars"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_bars: Option<Box<CtErrBars>>,

    #[serde(rename(serialize = "cat", deserialize = "cat"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cat: Option<CtAxDataSource>,

    #[serde(rename(serialize = "val", deserialize = "val"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<CtNumDataSource>,

    #[serde(rename(serialize = "shape", deserialize = "shape"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shape: Option<CtShape>,

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

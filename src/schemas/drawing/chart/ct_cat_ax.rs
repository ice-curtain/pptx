use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtTickMark;

use crate::schemas::drawing::chart::CtNumFmt;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtLblAlgn;

use crate::schemas::drawing::chart::CtSkip;

use crate::schemas::drawing::chart::CtLblOffset;

use crate::schemas::drawing::main::CtTextBody;

use crate::schemas::drawing::chart::CtUnsignedInt;

use crate::schemas::drawing::chart::CtScaling;

use crate::schemas::drawing::chart::CtTickLblPos;

use crate::schemas::drawing::chart::CtChartLines;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::chart::CtTitle;

use crate::schemas::drawing::chart::CtAxPos;

/**
 * @author : zhilong.zhou
 * @description : CT_CatAx
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCatAx {
    #[serde(rename(serialize = "auto", deserialize = "auto"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto: Option<CtBoolean>,

    #[serde(rename(serialize = "lblAlgn", deserialize = "lblAlgn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lbl_algn: Option<CtLblAlgn>,

    #[serde(rename(serialize = "lblOffset", deserialize = "lblOffset"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lbl_offset: Option<CtLblOffset>,

    #[serde(rename(serialize = "tickLblSkip", deserialize = "tickLblSkip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_lbl_skip: Option<CtSkip>,

    #[serde(rename(serialize = "tickMarkSkip", deserialize = "tickMarkSkip"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_mark_skip: Option<CtSkip>,

    #[serde(rename(serialize = "noMultiLvlLbl", deserialize = "noMultiLvlLbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_multi_lvl_lbl: Option<CtBoolean>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "axId", deserialize = "axId"))]
    pub ax_id: CtUnsignedInt,

    #[serde(rename(serialize = "scaling", deserialize = "scaling"))]
    pub scaling: CtScaling,

    #[serde(rename(serialize = "delete", deserialize = "delete"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<CtBoolean>,

    #[serde(rename(serialize = "axPos", deserialize = "axPos"))]
    pub ax_pos: CtAxPos,

    #[serde(rename(serialize = "majorGridlines", deserialize = "majorGridlines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_gridlines: Option<Box<CtChartLines>>,

    #[serde(rename(serialize = "minorGridlines", deserialize = "minorGridlines"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_gridlines: Option<Box<CtChartLines>>,

    #[serde(rename(serialize = "title", deserialize = "title"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Box<CtTitle>>,

    #[serde(rename(serialize = "numFmt", deserialize = "numFmt"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_fmt: Option<CtNumFmt>,

    #[serde(rename(serialize = "majorTickMark", deserialize = "majorTickMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub major_tick_mark: Option<CtTickMark>,

    #[serde(rename(serialize = "minorTickMark", deserialize = "minorTickMark"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minor_tick_mark: Option<CtTickMark>,

    #[serde(rename(serialize = "tickLblPos", deserialize = "tickLblPos"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tick_lbl_pos: Option<CtTickLblPos>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "txPr", deserialize = "txPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_pr: Option<Box<CtTextBody>>,

    #[serde(rename(serialize = "crossAx", deserialize = "crossAx"))]
    pub cross_ax: CtUnsignedInt,
}

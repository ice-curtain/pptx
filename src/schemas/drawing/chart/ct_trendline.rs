use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtTrendlineLbl;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtTrendlineType;

use crate::schemas::drawing::chart::CtOrder;

use crate::schemas::drawing::chart::CtDouble;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::chart::CtPeriod;

/**
 * @author : zhilong.zhou
 * @description : CT_Trendline
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTrendline {
    #[serde(rename(serialize = "name", deserialize = "name"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "trendlineType", deserialize = "trendlineType"))]
    pub trendline_type: CtTrendlineType,

    #[serde(rename(serialize = "order", deserialize = "order"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<CtOrder>,

    #[serde(rename(serialize = "period", deserialize = "period"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<CtPeriod>,

    #[serde(rename(serialize = "forward", deserialize = "forward"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward: Option<CtDouble>,

    #[serde(rename(serialize = "backward", deserialize = "backward"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backward: Option<CtDouble>,

    #[serde(rename(serialize = "intercept", deserialize = "intercept"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intercept: Option<CtDouble>,

    #[serde(rename(serialize = "dispRSqr", deserialize = "dispRSqr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disp_r_sqr: Option<CtBoolean>,

    #[serde(rename(serialize = "dispEq", deserialize = "dispEq"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disp_eq: Option<CtBoolean>,

    #[serde(rename(serialize = "trendlineLbl", deserialize = "trendlineLbl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trendline_lbl: Option<Box<CtTrendlineLbl>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

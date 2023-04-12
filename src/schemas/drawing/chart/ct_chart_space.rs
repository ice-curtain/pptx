use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtTextLanguageId;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtProtection;

use crate::schemas::drawing::chart::CtRelId;

use crate::schemas::drawing::chart::CtPivotSource;

use crate::schemas::drawing::chart::CtChart;

use crate::schemas::drawing::main::CtTextBody;

use crate::schemas::drawing::chart::CtExternalData;

use crate::schemas::drawing::chart::CtStyle;

use crate::schemas::drawing::main::CtColorMapping;

use crate::schemas::drawing::chart::CtPrintSettings;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_ChartSpace
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "chartSpace", deserialize = "chartSpace"))]

pub struct CtChartSpace {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:p")]
    pub p_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:a")]
    pub a_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:r")]
    pub r_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns:s")]
    pub s_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@xmlns")]
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "date1904", deserialize = "date1904"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date1904: Option<CtBoolean>,

    #[serde(rename(serialize = "lang", deserialize = "lang"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<CtTextLanguageId>,

    #[serde(rename(serialize = "roundedCorners", deserialize = "roundedCorners"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rounded_corners: Option<CtBoolean>,

    #[serde(rename(serialize = "style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtStyle>,

    #[serde(rename(serialize = "clrMapOvr", deserialize = "clrMapOvr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_map_ovr: Option<CtColorMapping>,

    #[serde(rename(serialize = "pivotSource", deserialize = "pivotSource"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pivot_source: Option<CtPivotSource>,

    #[serde(rename(serialize = "protection", deserialize = "protection"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protection: Option<CtProtection>,

    #[serde(rename(serialize = "chart", deserialize = "chart"))]
    pub chart: Box<CtChart>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "txPr", deserialize = "txPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_pr: Option<Box<CtTextBody>>,

    #[serde(rename(serialize = "externalData", deserialize = "externalData"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_data: Option<CtExternalData>,

    #[serde(rename(serialize = "printSettings", deserialize = "printSettings"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_settings: Option<CtPrintSettings>,

    #[serde(rename(serialize = "userShapes", deserialize = "userShapes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_shapes: Option<CtRelId>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

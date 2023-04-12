use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtMarkerSize;

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtMarkerStyle;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_Marker
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtMarker {
    #[serde(rename(serialize = "symbol", deserialize = "symbol"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<CtMarkerStyle>,

    #[serde(rename(serialize = "size", deserialize = "size"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<CtMarkerSize>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

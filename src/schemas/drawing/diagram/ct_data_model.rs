use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtBackgroundFormatting;

use crate::schemas::drawing::main::CtWholeE2oFormatting;

use crate::schemas::drawing::diagram::CtCxnList;

use crate::schemas::drawing::diagram::CtPtList;

/**
 * @author : zhilong.zhou
 * @description : CT_DataModel
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "dataModel", deserialize = "dataModel"))]

pub struct CtDataModel {
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

    #[serde(rename(serialize = "ptLst", deserialize = "ptLst"))]
    pub pt_lst: Vec<CtPtList>,

    #[serde(rename(serialize = "cxnLst", deserialize = "cxnLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn_lst: Option<CtCxnList>,

    #[serde(rename(serialize = "bg", deserialize = "bg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<Vec<CtBackgroundFormatting>>,

    #[serde(rename(serialize = "whole", deserialize = "whole"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whole: Option<Vec<CtWholeE2oFormatting>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

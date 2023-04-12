use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTableGrid;

use crate::schemas::drawing::main::CtTableProperties;

use crate::schemas::drawing::main::CtTableRow;

/**
 * @author : zhilong.zhou
 * @description : CT_Table
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "a:tbl", deserialize = "tbl"))]

pub struct CtTable {
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

    #[serde(rename(serialize = "a:tblPr", deserialize = "tblPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tbl_pr: Option<Box<CtTableProperties>>,

    #[serde(rename(serialize = "a:tblGrid", deserialize = "tblGrid"))]
    pub tbl_grid: CtTableGrid,

    #[serde(rename(serialize = "a:tr", deserialize = "tr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<Vec<CtTableRow>>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTableCellProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_TableCell
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTableCell {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rowSpan")]
    pub row_span_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@gridSpan")]
    pub grid_span_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hMerge")]
    pub h_merge_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@vMerge")]
    pub v_merge_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@id")]
    pub id_attr: Option<String>,

    #[serde(rename(serialize = "a:txBody", deserialize = "txBody"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_body: Option<Box<CtTextBody>>,

    #[serde(rename(serialize = "a:tcPr", deserialize = "tcPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_pr: Option<Box<CtTableCellProperties>>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

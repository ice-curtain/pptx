use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_PrintProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPrintProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@prnWhat")]
    pub prn_what_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@clrMode")]
    pub clr_mode_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hiddenSlides")]
    pub hidden_slides_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@scaleToFitPaper")]
    pub scale_to_fit_paper_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@frameSlides")]
    pub frame_slides_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

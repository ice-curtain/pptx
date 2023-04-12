use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

/**
 * @author : zhilong.zhou
 * @description : CT_HeaderFooter
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHeaderFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sldNum")]
    pub sld_num_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hdr")]
    pub hdr_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@ftr")]
    pub ftr_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@dt")]
    pub dt_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtPrintProperties;

use crate::schemas::presentation::main::CtShowProperties;

use crate::schemas::drawing::main::CtColorMru;

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_PresentationProperties
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:presentationPr", deserialize = "presentationPr"))]

pub struct CtPresentationProperties {
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

    #[serde(rename(serialize = "p:prnPr", deserialize = "prnPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prn_pr: Option<CtPrintProperties>,

    #[serde(rename(serialize = "p:showPr", deserialize = "showPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_pr: Option<CtShowProperties>,

    #[serde(rename(serialize = "p:clrMru", deserialize = "clrMru"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_mru: Option<CtColorMru>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionListModify;

/**
 * @author : zhilong.zhou
 * @description : CT_Placeholder
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtPlaceholder {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@type")]
    pub r#type_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@orient")]
    pub orient_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sz")]
    pub sz_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@idx")]
    pub idx_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hasCustomPrompt")]
    pub has_custom_prompt_attr: Option<String>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}

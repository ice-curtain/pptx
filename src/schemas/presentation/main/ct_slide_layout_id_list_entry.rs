use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideLayoutIdListEntry
 */
#[derive(Serialize, Debug)]
pub struct CtSlideLayoutIdListEntry {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@id")]
    pub id_attr: Option<String>,

    #[serde(rename = "@r:id")]
    pub r_id_attr: String,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

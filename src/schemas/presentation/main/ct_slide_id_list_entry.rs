use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideIdListEntry
 */

#[derive(Serialize, Debug)]
pub struct CtSlideIdListEntry {
    #[serde(rename = "@id")]
    pub id_attr: Option<u32>,

    #[serde(rename = "@r:id")]
    pub r_id_attr: String,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_CommentAuthor
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCommentAuthor {
    #[serde(rename = "@id")]
    pub id_attr: String,

    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename = "@initials")]
    pub initials_attr: String,

    #[serde(rename = "@lastIdx")]
    pub last_idx_attr: String,

    #[serde(rename = "@clrIdx")]
    pub clr_idx_attr: String,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

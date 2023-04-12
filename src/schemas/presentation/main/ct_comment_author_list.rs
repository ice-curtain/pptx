use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtCommentAuthor;

/**
 * @author : zhilong.zhou
 * @description : CT_CommentAuthorList
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:cmAuthorLst", deserialize = "cmAuthorLst"))]

pub struct CtCommentAuthorList {
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

    #[serde(rename(serialize = "p:cmAuthor", deserialize = "cmAuthor"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cm_author: Option<Vec<CtCommentAuthor>>,
}

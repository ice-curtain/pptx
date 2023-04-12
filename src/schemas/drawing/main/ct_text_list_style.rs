use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextParagraphProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

/**
 * @author : zhilong.zhou
 * @description : CT_TextListStyle
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextListStyle {
    #[serde(rename(serialize = "a:defPPr", deserialize = "defPPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub def_p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl1pPr", deserialize = "lvl1pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl1p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl2pPr", deserialize = "lvl2pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl2p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl3pPr", deserialize = "lvl3pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl3p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl4pPr", deserialize = "lvl4pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl4p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl5pPr", deserialize = "lvl5pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl5p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl6pPr", deserialize = "lvl6pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl6p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl7pPr", deserialize = "lvl7pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl7p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl8pPr", deserialize = "lvl8pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl8p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:lvl9pPr", deserialize = "lvl9pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lvl9p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

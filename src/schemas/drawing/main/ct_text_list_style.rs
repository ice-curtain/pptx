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

    ///此元素为具有属性 lvl="0" 的所有元素指定所有段落级文本属性。 总共允许 9 个级别的文本属性元素，级别 0-8。 建议按级别递增的顺序指定此和其他级别属性元素的顺序。 那就是 lvl2pPr 应该在 lvl3pPr 之前。 这允许较低级别的属性优先于较高级别的属性，因为它们首先被解析。
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

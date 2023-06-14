use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtRegularTextRun;
use crate::schemas::drawing::main::CtTextCharacterProperties;
use crate::schemas::drawing::main::CtTextField;
use crate::schemas::drawing::main::CtTextLineBreak;
use crate::schemas::drawing::main::CtTextParagraphProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_TextParagraph
 * 此元素指定包含文本正文中一段文本的存在。 段落是文本正文中最高级别的文本分隔机制。 段落可以包含与该段落关联的文本段落属性。 如果未列出任何属性，则使用 defPPr 元素中指定的属性
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CtTextParagraph {
    #[serde(rename(serialize = "a:pPr", deserialize = "pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:r", deserialize = "r"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<Vec<CtRegularTextRun>>,

    #[serde(rename(serialize = "a:br", deserialize = "br"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub br: Option<Vec<CtTextLineBreak>>,

    #[serde(rename(serialize = "a:fld", deserialize = "fld"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fld: Option<Vec<CtTextField>>,

    #[serde(rename(serialize = "a:endParaRPr", deserialize = "endParaRPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_para_r_pr: Option<Box<CtTextCharacterProperties>>,

}

impl Default for CtTextParagraph {
    fn default() -> Self {
        CtTextParagraph {
            p_pr: None,
            r: Some(vec![]),
            br: None,
            fld: None,
            end_para_r_pr: Some(Box::new(CtTextCharacterProperties::default())),
        }
    }
}






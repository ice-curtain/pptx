use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextLineBreak;

use crate::schemas::drawing::main::CtRegularTextRun;

use crate::schemas::drawing::main::CtTextField;

use crate::schemas::drawing::main::CtTextCharacterProperties;

use crate::schemas::drawing::main::CtTextParagraphProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_TextParagraph
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextParagraph {
    #[serde(rename(serialize = "a:pPr", deserialize = "pPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p_pr: Option<Box<CtTextParagraphProperties>>,

    #[serde(rename(serialize = "a:endParaRPr", deserialize = "endParaRPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_para_r_pr: Option<Box<CtTextCharacterProperties>>,

    #[serde(rename(serialize = "a:r", deserialize = "r"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r: Option<Vec<CtRegularTextRun>>,

    #[serde(rename(serialize = "a:br", deserialize = "br"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub br: Option<Vec<CtTextLineBreak>>,

    #[serde(rename(serialize = "a:fld", deserialize = "fld"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fld: Option<Vec<CtTextField>>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextBodyProperties;

use crate::schemas::drawing::main::CtTextListStyle;

use crate::schemas::drawing::main::CtTextParagraph;

/**
 * @author : zhilong.zhou
 * @description : CT_TextBody
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTextBody {
    #[serde(rename(serialize = "a:bodyPr", deserialize = "bodyPr"))]
    pub body_pr: CtTextBodyProperties,

    ///此元素指定与此文本正文关联的样式列表
    #[serde(rename(serialize = "a:lstStyle", deserialize = "lstStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lst_style: Option<Box<CtTextListStyle>>,

    #[serde(rename(serialize = "a:p", deserialize = "p"))]
    pub p: Vec<CtTextParagraph>,
}



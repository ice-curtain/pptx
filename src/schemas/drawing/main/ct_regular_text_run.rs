use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTextCharacterProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_RegularTextRun
 */
#[derive(Serialize, Deserialize, Debug)]
///此元素指定包含文本正文中是否存在一段文本。 run 元素是文本正文中最低级别的文本分隔机制。 文本运行可以包含与运行关联的文本运行属性。 如果未列出任何属性，则使用 defRPr 元素中指定的属性
pub struct CtRegularTextRun {
    #[serde(rename(serialize = "a:rPr", deserialize = "rPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r_pr: Option<Box<CtTextCharacterProperties>>,

    #[serde(rename(serialize = "a:t", deserialize = "t"))]
    pub t: String,
}


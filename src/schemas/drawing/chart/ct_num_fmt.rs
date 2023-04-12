use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_NumFmt
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNumFmt {
    #[serde(rename = "@formatCode")]
    pub format_code_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@sourceLinked")]
    pub source_linked_attr: Option<String>,
}

use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_HeaderFooter
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtHeaderFooter {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@alignWithMargins")]
    pub align_with_margins_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@differentOddEven")]
    pub different_odd_even_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@differentFirst")]
    pub different_first_attr: Option<String>,

    #[serde(rename(serialize = "oddHeader", deserialize = "oddHeader"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odd_header: Option<String>,

    #[serde(rename(serialize = "oddFooter", deserialize = "oddFooter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odd_footer: Option<String>,

    #[serde(rename(serialize = "evenHeader", deserialize = "evenHeader"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_header: Option<String>,

    #[serde(rename(serialize = "evenFooter", deserialize = "evenFooter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub even_footer: Option<String>,

    #[serde(rename(serialize = "firstHeader", deserialize = "firstHeader"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_header: Option<String>,

    #[serde(rename(serialize = "firstFooter", deserialize = "firstFooter"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_footer: Option<String>,
}

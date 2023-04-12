use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_TLOleChartTargetElement
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlOleChartTargetElement {
    #[serde(rename = "@type")]
    pub r#type_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lvl")]
    pub lvl_attr: Option<String>,
}

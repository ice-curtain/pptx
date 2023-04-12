use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_ModifyVerifier
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtModifyVerifier {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@algorithmName")]
    pub algorithm_name_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@hashValue")]
    pub hash_value_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@saltValue")]
    pub salt_value_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@spinValue")]
    pub spin_value_attr: Option<String>,
}

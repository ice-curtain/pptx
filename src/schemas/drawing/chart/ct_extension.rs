use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Extension
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtExtension {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@uri")]
    pub uri_attr: Option<String>,
}

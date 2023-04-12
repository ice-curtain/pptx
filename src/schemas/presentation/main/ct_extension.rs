use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_Extension
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtExtension {
    #[serde(rename = "@uri")]
    pub uri_attr: String,
}

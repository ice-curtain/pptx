use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTemplate;

/**
 * @author : zhilong.zhou
 * @description : CT_TLTemplateList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlTemplateList {
    #[serde(rename(serialize = "p:tmpl", deserialize = "tmpl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmpl: Option<Vec<CtTlTemplate>>,
}

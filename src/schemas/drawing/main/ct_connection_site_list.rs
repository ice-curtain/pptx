use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtConnectionSite;

/**
 * @author : zhilong.zhou
 * @description : CT_ConnectionSiteList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtConnectionSiteList {
    #[serde(rename(serialize = "a:cxn", deserialize = "cxn"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cxn: Option<Vec<CtConnectionSite>>,
}

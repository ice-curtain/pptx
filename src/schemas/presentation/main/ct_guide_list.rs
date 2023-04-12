use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtGuide;

/**
 * @author : zhilong.zhou
 * @description : CT_GuideList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGuideList {
    #[serde(rename(serialize = "p:guide", deserialize = "guide"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guide: Option<Vec<CtGuide>>,
}

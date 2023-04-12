use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtSlideRelationshipListEntry;

/**
 * @author : zhilong.zhou
 * @description : CT_SlideRelationshipList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSlideRelationshipList {
    #[serde(rename(serialize = "p:sld", deserialize = "sld"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld: Option<Vec<CtSlideRelationshipListEntry>>,
}

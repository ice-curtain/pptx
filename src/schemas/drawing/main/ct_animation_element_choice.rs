use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAnimationDgmElement;

use crate::schemas::drawing::main::CtAnimationChartElement;

/**
 * @author : zhilong.zhou
 * @description : CT_AnimationElementChoice
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAnimationElementChoice {
    #[serde(rename(serialize = "a:dgm", deserialize = "dgm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dgm: Option<CtAnimationDgmElement>,

    #[serde(rename(serialize = "a:chart", deserialize = "chart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<CtAnimationChartElement>,
}

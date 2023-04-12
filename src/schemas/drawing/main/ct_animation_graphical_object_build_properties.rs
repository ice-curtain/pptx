use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtAnimationDgmBuildProperties;

use crate::schemas::drawing::main::CtAnimationChartBuildProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_AnimationGraphicalObjectBuildProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAnimationGraphicalObjectBuildProperties {
    #[serde(rename(serialize = "a:bldDgm", deserialize = "bldDgm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_dgm: Option<CtAnimationDgmBuildProperties>,

    #[serde(rename(serialize = "a:bldChart", deserialize = "bldChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_chart: Option<CtAnimationChartBuildProperties>,
}

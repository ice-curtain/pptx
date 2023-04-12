use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlOleBuildChart;

use crate::schemas::presentation::main::CtTlBuildDiagram;

use crate::schemas::presentation::main::CtTlBuildParagraph;

use crate::schemas::presentation::main::CtTlGraphicalObjectBuild;

/**
 * @author : zhilong.zhou
 * @description : CT_BuildList
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtBuildList {
    #[serde(rename(serialize = "p:bldP", deserialize = "bldP"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_p: Option<Vec<CtTlBuildParagraph>>,

    #[serde(rename(serialize = "p:bldDgm", deserialize = "bldDgm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_dgm: Option<Vec<CtTlBuildDiagram>>,

    #[serde(rename(serialize = "p:bldOleChart", deserialize = "bldOleChart"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_ole_chart: Option<Vec<CtTlOleBuildChart>>,

    #[serde(rename(serialize = "p:bldGraphic", deserialize = "bldGraphic"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bld_graphic: Option<Vec<CtTlGraphicalObjectBuild>>,
}

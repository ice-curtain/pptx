use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtTlTextTargetElement;

use crate::schemas::presentation::main::CtEmpty;

use crate::schemas::drawing::main::CtAnimationElementChoice;

use crate::schemas::presentation::main::CtTlSubShapeId;

use crate::schemas::presentation::main::CtTlOleChartTargetElement;

/**
 * @author : zhilong.zhou
 * @description : CT_TLShapeTargetElement
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtTlShapeTargetElement {
    #[serde(rename = "@spid")]
    pub spid_attr: String,

    #[serde(rename(serialize = "p:bg", deserialize = "bg"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<CtEmpty>,

    #[serde(rename(serialize = "p:subSp", deserialize = "subSp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_sp: Option<CtTlSubShapeId>,

    #[serde(rename(serialize = "p:oleChartEl", deserialize = "oleChartEl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ole_chart_el: Option<CtTlOleChartTargetElement>,

    #[serde(rename(serialize = "p:txEl", deserialize = "txEl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_el: Option<CtTlTextTargetElement>,

    #[serde(rename(serialize = "p:graphicEl", deserialize = "graphicEl"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub graphic_el: Option<CtAnimationElementChoice>,
}

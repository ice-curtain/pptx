use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtTextProps;

use crate::schemas::drawing::main::CtScene3D;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtShape3D;

use crate::schemas::drawing::main::CtShapeStyle;

/**
 * @author : zhilong.zhou
 * @description : CT_StyleLabel
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtStyleLabel {
    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename(serialize = "scene3d", deserialize = "scene3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene3d: Option<CtScene3D>,

    #[serde(rename(serialize = "sp3d", deserialize = "sp3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp3d: Option<CtShape3D>,

    #[serde(rename(serialize = "txPr", deserialize = "txPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_pr: Option<CtTextProps>,

    #[serde(rename(serialize = "style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

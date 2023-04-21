use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::drawing::main::CtPatternFillProperties;

use crate::schemas::drawing::main::CtScene3D;

use crate::schemas::drawing::main::CtGradientFillProperties;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTransform2D;

use crate::schemas::drawing::main::CtShape3D;

use crate::schemas::drawing::main::CtNoFillProperties;

use crate::schemas::drawing::main::CtCustomGeometry2D;

use crate::schemas::drawing::main::CtEffectList;

use crate::schemas::drawing::main::CtLineProperties;

use crate::schemas::drawing::main::CtSolidColorFillProperties;

use crate::schemas::drawing::main::CtPresetGeometry2D;

use crate::schemas::drawing::main::CtGroupFillProperties;

use crate::schemas::drawing::main::CtEffectContainer;

/**
 * @author : zhilong.zhou
 * @description : CT_ShapeProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtShapeProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bwMode")]
    pub bw_mode_attr: Option<String>,

    #[serde(rename(serialize = "a:xfrm", deserialize = "xfrm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xfrm: Option<CtTransform2D>,

    #[serde(rename(serialize = "a:custGeom", deserialize = "custGeom"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_geom: Option<CtCustomGeometry2D>,

    #[serde(rename(serialize = "a:prstGeom", deserialize = "prstGeom"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_geom: Option<CtPresetGeometry2D>,


    #[serde(rename(serialize = "a:noFill", deserialize = "noFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_fill: Option<CtNoFillProperties>,

    #[serde(rename(serialize = "a:solidFill", deserialize = "solidFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub solid_fill: Option<CtSolidColorFillProperties>,

    #[serde(rename(serialize = "a:gradFill", deserialize = "gradFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grad_fill: Option<CtGradientFillProperties>,

    #[serde(rename(serialize = "a:blipFill", deserialize = "blipFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blip_fill: Option<Box<CtBlipFillProperties>>,

    #[serde(rename(serialize = "a:pattFill", deserialize = "pattFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patt_fill: Option<CtPatternFillProperties>,

    #[serde(rename(serialize = "a:grpFill", deserialize = "grpFill"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_fill: Option<CtGroupFillProperties>,

    #[serde(rename(serialize = "a:ln", deserialize = "ln"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ln: Option<CtLineProperties>,

    #[serde(rename(serialize = "a:effectLst", deserialize = "effectLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_lst: Option<Box<CtEffectList>>,

    #[serde(rename(serialize = "a:effectDag", deserialize = "effectDag"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_dag: Option<Box<CtEffectContainer>>,

    #[serde(rename(serialize = "a:scene3d", deserialize = "scene3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene3d: Option<CtScene3D>,

    #[serde(rename(serialize = "a:sp3d", deserialize = "sp3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp3d: Option<CtShape3D>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,





}

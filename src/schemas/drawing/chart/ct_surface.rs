use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtPictureOptions;

use crate::schemas::drawing::main::CtShapeProperties;

use crate::schemas::drawing::chart::CtThickness;

/**
 * @author : zhilong.zhou
 * @description : CT_Surface
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSurface {
    #[serde(rename(serialize = "thickness", deserialize = "thickness"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thickness: Option<CtThickness>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "pictureOptions", deserialize = "pictureOptions"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_options: Option<CtPictureOptions>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtPictureLocking;

/**
 * @author : zhilong.zhou
 * @description : CT_NonVisualPictureProperties
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtNonVisualPictureProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@preferRelativeResize")]
    pub prefer_relative_resize_attr: Option<String>,

    #[serde(rename(serialize = "a:picLocks", deserialize = "picLocks"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pic_locks: Option<CtPictureLocking>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,
}

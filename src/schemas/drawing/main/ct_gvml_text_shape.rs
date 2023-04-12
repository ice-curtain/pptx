use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtTransform2D;

use crate::schemas::drawing::main::CtGvmlUseShapeRectangle;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtTextBody;

/**
 * @author : zhilong.zhou
 * @description : CT_GvmlTextShape
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGvmlTextShape {
    #[serde(rename(serialize = "a:txBody", deserialize = "txBody"))]
    pub tx_body: Box<CtTextBody>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,

    #[serde(rename(serialize = "a:useSpRect", deserialize = "useSpRect"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_sp_rect: Option<CtGvmlUseShapeRectangle>,

    #[serde(rename(serialize = "a:xfrm", deserialize = "xfrm"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xfrm: Option<CtTransform2D>,
}

use serde::{Deserialize, Serialize};
use crate::schemas::standard::microsoft::presentation::{P14CtCreationId, P14CtDefaultImageDpi, P14CtDiscardImageEditData, P14ModId, P15ChartTrackingRefBased, P15NotesGuideLst, P15SldGuideLst};

/**
 * @author : zhilong.zhou
 * @description : CT_Extension
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtExtension {
    #[serde(rename = "@uri")]
    pub uri_attr: String,



    #[serde(rename(serialize = "p14:creationId", deserialize = "creationId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    creation_id: Option<P14CtCreationId>,


    #[serde(rename(serialize = "p14:discardImageEditData", deserialize = "discardImageEditData"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    discard_image_edit_data: Option<P14CtDiscardImageEditData>,

    #[serde(rename(serialize = "p14:defaultImageDpi", deserialize = "defaultImageDpi"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    default_image_dpi: Option<P14CtDefaultImageDpi>,

    #[serde(rename(serialize = "p15:chartTrackingRefBased", deserialize = "chartTrackingRefBased"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    chart_tracking_ref_based: Option<P15ChartTrackingRefBased>,


    #[serde(rename(serialize = "p15:notesGuideLst", deserialize = "notesGuideLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    notes_guide_lst: Option<P15NotesGuideLst>,

    #[serde(rename(serialize = "p15:sldGuideLst", deserialize = "sldGuideLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    sld_guide_lst: Option<P15SldGuideLst>,


    #[serde(rename(serialize = "p14:modId", deserialize = "modId"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    mod_id: Option<P14ModId>,
}

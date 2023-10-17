use serde::{Deserialize, Serialize};
use crate::abstraction::common_model::DigitalAnchor;

use crate::schemas::drawing::main::{CtBlip, CtGeomGuideList, CtNonVisualDrawingProps, CtNonVisualPictureProperties, CtPictureLocking, CtPoint2D, CtPositiveSize2D, CtPresetGeometry2D, CtRelativeRect, CtShapeStyle, CtStretchInfoProperties, CtTransform2D};

use crate::schemas::presentation::main::{CtApplicationNonVisualDrawingProps, CtExtensionListModify};

use crate::schemas::drawing::main::CtBlipFillProperties;

use crate::schemas::presentation::main::CtPictureNonVisual;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_Picture
 */
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:pic", deserialize = "pic"))]
pub struct CtPicture {
    //unused code
    /*    #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "@xmlns:p")]
        pub p_attr: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "@xmlns:a")]
        pub a_attr: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "@xmlns:r")]
        pub r_attr: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "@xmlns:s")]
        pub s_attr: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "@xmlns")]
        pub default_namespace_attr: Option<String>,*/

    #[serde(rename(serialize = "p:nvPicPr", deserialize = "nvPicPr"))]
    pub nv_pic_pr: CtPictureNonVisual,

    #[serde(rename(serialize = "p:blipFill", deserialize = "blipFill"))]
    pub blip_fill: CtBlipFillProperties,

    #[serde(rename(serialize = "p:spPr", deserialize = "spPr"))]
    pub sp_pr: CtShapeProperties,

    #[serde(rename(serialize = "p:style", deserialize = "style"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<CtShapeStyle>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionListModify>,
}


impl CtPicture {
    pub fn from_template(picture_id: &str, id: u32, anchor: DigitalAnchor) -> Self {
        CtPicture {
            nv_pic_pr: CtPictureNonVisual {
                c_nv_pr: CtNonVisualDrawingProps {
                    id_attr: id,
                    name_attr: "Picture".to_string(),
                    descr_attr: Some("".to_string()),
                    hidden_attr: None,
                    title_attr: None,
                    hlink_click: None,
                    hlink_hover: None,
                    ext_lst: None,
                },
                c_nv_pic_pr: CtNonVisualPictureProperties {
                    prefer_relative_resize_attr: None,
                    pic_locks: Some(CtPictureLocking {
                        no_grp_attr: None,
                        no_select_attr: None,
                        no_rot_attr: None,
                        no_change_aspect_attr: Some(true),
                        no_move_attr: None,
                        no_resize_attr: None,
                        no_edit_points_attr: None,
                        no_adjust_handles_attr: None,
                        no_change_arrowheads_attr: None,
                        no_change_shape_type_attr: None,
                        no_crop_attr: None,
                        ext_lst: None,
                    }),
                    ext_lst: None,
                },
                nv_pr: CtApplicationNonVisualDrawingProps {
                    is_photo_attr: None,
                    user_drawn_attr: None,
                    ph: None,
                    cust_data_lst: None,
                    ext_lst: None,
                    audio_cd: None,
                    wav_audio_file: None,
                    audio_file: None,
                    video_file: None,
                    quick_time_file: None,
                },
            },
            blip_fill: CtBlipFillProperties {
                dpi_attr: None,
                rot_with_shape_attr: None,
                blip: Some(CtBlip {
                    embed_attr: Some(picture_id.to_string()),
                    link_attr: None,
                    cstate_attr: None,
                    ext_lst: None,
                    alpha_bi_level: None,
                    alpha_ceiling: None,
                    alpha_floor: None,
                    alpha_inv: None,
                    alpha_mod: None,
                    alpha_mod_fix: None,
                    alpha_repl: None,
                    bi_level: None,
                    blur: None,
                    clr_change: None,
                    clr_repl: None,
                    duotone: None,
                    fill_overlay: None,
                    grayscl: None,
                    hsl: None,
                    lum: None,
                    tint: None,
                }),
                src_rect: None,
                tile: None,
                stretch: Some(CtStretchInfoProperties {
                    fill_rect: Some(CtRelativeRect {
                        l_attr: None,
                        t_attr: None,
                        r_attr: None,
                        b_attr: None,
                    }),
                }),
            },
            sp_pr: CtShapeProperties {
                bw_mode_attr: None,
                xfrm: Some(CtTransform2D {
                    rot_attr: None,
                    flip_h_attr: None,
                    flip_v_attr: None,
                    off: Some(CtPoint2D {
                        x_attr: anchor.x,
                        y_attr: anchor.y,
                    }),
                    ext: Some(CtPositiveSize2D {
                        cx_attr: anchor.cx,
                        cy_attr: anchor.cy,
                    }),
                }),
                cust_geom: None,
                prst_geom: Some(CtPresetGeometry2D {
                    prst_attr: "rect".to_string(),
                    av_lst: Some(CtGeomGuideList { gd: None }),
                }),
                no_fill: None,
                solid_fill: None,
                grad_fill: None,
                blip_fill: None,
                patt_fill: None,
                grp_fill: None,
                ln: None,
                effect_lst: None,
                effect_dag: None,
                scene3d: None,
                sp3d: None,
                ext_lst: None,
            },
            style: None,
            ext_lst: None,
        }
    }
}
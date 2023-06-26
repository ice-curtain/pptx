use serde::{Deserialize, Serialize};

use crate::schemas::drawing::main::CtPresetTextShape;

use crate::schemas::drawing::main::CtTextNormalAutofit;

use crate::schemas::drawing::main::CtScene3D;

use crate::schemas::drawing::main::CtOfficeArtExtensionList;

use crate::schemas::drawing::main::CtShape3D;

use crate::schemas::drawing::main::CtFlatText;

use crate::schemas::drawing::main::CtTextShapeAutofit;

use crate::schemas::drawing::main::CtTextNoAutofit;

/**
 * @author : zhilong.zhou
 * @description : CT_TextBodyProperties
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct CtTextBodyProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rot")]
    pub rot_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@spcFirstLastPara")]
    pub spc_first_last_para_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@vertOverflow")]
    pub vert_overflow_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@horzOverflow")]
    pub horz_overflow_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@vert")]
    pub vert_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@wrap")]
    pub wrap_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@lIns")]
    pub l_ins_attr: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@tIns")]
    pub t_ins_attr: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rIns")]
    pub r_ins_attr: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bIns")]
    pub b_ins_attr: Option<i32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@numCol")]
    pub num_col_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@spcCol")]
    pub spc_col_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rtlCol")]
    pub rtl_col_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@fromWordArt")]
    pub from_word_art_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@anchor")]
    pub anchor_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@anchorCtr")]
    pub anchor_ctr_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@forceAA")]
    pub force_aa_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@upright")]
    pub upright_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@compatLnSpc")]
    pub compat_ln_spc_attr: Option<String>,

    #[serde(rename(serialize = "a:prstTxWarp", deserialize = "prstTxWarp"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prst_tx_warp: Option<CtPresetTextShape>,

    #[serde(rename(serialize = "a:scene3d", deserialize = "scene3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene3d: Option<CtScene3D>,

    #[serde(rename(serialize = "a:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtOfficeArtExtensionList>,

    #[serde(rename(serialize = "a:noAutofit", deserialize = "noAutofit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_autofit: Option<CtTextNoAutofit>,

    #[serde(rename(serialize = "a:normAutofit", deserialize = "normAutofit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub norm_autofit: Option<CtTextNormalAutofit>,

    #[serde(rename(serialize = "a:spAutoFit", deserialize = "spAutoFit"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_auto_fit: Option<CtTextShapeAutofit>,

    #[serde(rename(serialize = "a:sp3d", deserialize = "sp3d"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp3d: Option<CtShape3D>,

    #[serde(rename(serialize = "a:flatTx", deserialize = "flatTx"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_tx: Option<CtFlatText>,
}

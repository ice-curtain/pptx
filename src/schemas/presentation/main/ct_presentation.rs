use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtHandoutMasterIdList;

use crate::schemas::presentation::main::CtNotesMasterIdList;

use crate::schemas::presentation::main::CtSlideIdList;

use crate::schemas::presentation::main::CtEmbeddedFontList;

use crate::schemas::presentation::main::CtPhotoAlbum;

use crate::schemas::drawing::main::CtPositiveSize2D;

use crate::schemas::presentation::main::CtSlideMasterIdList;

use crate::schemas::presentation::main::CtCustomShowList;

use crate::schemas::presentation::main::CtExtensionList;

use crate::schemas::presentation::main::CtSlideSize;

use crate::schemas::presentation::main::CtKinsoku;

use crate::schemas::presentation::main::CtSmartTags;

use crate::schemas::drawing::main::CtTextListStyle;

use crate::schemas::presentation::main::CtModifyVerifier;

use crate::schemas::presentation::main::CtCustomerDataList;

/**
 * @author : zhilong.zhou
 * @description : CT_Presentation
 */

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename(serialize = "p:presentation", deserialize = "presentation"))]

pub struct CtPresentation {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@serverZoom")]
    pub server_zoom_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@firstSlideNum")]
    pub first_slide_num_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@showSpecialPlsOnTitleSld")]
    pub show_special_pls_on_title_sld_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@rtl")]
    pub rtl_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@removePersonalInfoOnSave")]
    pub remove_personal_info_on_save_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@compatMode")]
    pub compat_mode_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@strictFirstAndLastChars")]
    pub strict_first_and_last_chars_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@embedTrueTypeFonts")]
    pub embed_true_type_fonts_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@saveSubsetFonts")]
    pub save_subset_fonts_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@autoCompressPictures")]
    pub auto_compress_pictures_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@bookmarkIdSeed")]
    pub bookmark_id_seed_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@conformance")]
    pub conformance_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
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
    pub default_namespace_attr: Option<String>,

    #[serde(rename(serialize = "p:sldMasterIdLst", deserialize = "sldMasterIdLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_master_id_lst: Option<CtSlideMasterIdList>,

    #[serde(rename(serialize = "p:notesMasterIdLst", deserialize = "notesMasterIdLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes_master_id_lst: Option<CtNotesMasterIdList>,

    #[serde(rename(serialize = "p:handoutMasterIdLst", deserialize = "handoutMasterIdLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handout_master_id_lst: Option<CtHandoutMasterIdList>,

    #[serde(rename(serialize = "p:sldIdLst", deserialize = "sldIdLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_id_lst: Option<CtSlideIdList>,

    #[serde(rename(serialize = "p:sldSz", deserialize = "sldSz"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sld_sz: Option<CtSlideSize>,

    #[serde(rename(serialize = "p:notesSz", deserialize = "notesSz"))]
    pub notes_sz: CtPositiveSize2D,

    #[serde(rename(serialize = "p:smartTags", deserialize = "smartTags"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tags: Option<CtSmartTags>,

    #[serde(rename(serialize = "p:embeddedFontLst", deserialize = "embeddedFontLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedded_font_lst: Option<CtEmbeddedFontList>,

    #[serde(rename(serialize = "p:custShowLst", deserialize = "custShowLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_show_lst: Option<CtCustomShowList>,

    #[serde(rename(serialize = "p:photoAlbum", deserialize = "photoAlbum"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_album: Option<CtPhotoAlbum>,

    #[serde(rename(serialize = "p:custDataLst", deserialize = "custDataLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_data_lst: Option<CtCustomerDataList>,

    #[serde(rename(serialize = "p:kinsoku", deserialize = "kinsoku"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kinsoku: Option<Vec<CtKinsoku>>,

    #[serde(rename(serialize = "p:defaultTextStyle", deserialize = "defaultTextStyle"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_text_style: Option<CtTextListStyle>,

    #[serde(rename(serialize = "p:modifyVerifier", deserialize = "modifyVerifier"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modify_verifier: Option<CtModifyVerifier>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

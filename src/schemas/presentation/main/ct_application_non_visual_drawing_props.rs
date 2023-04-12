use serde::{Deserialize, Serialize};

use crate::schemas::presentation::main::CtPlaceholder;

use crate::schemas::drawing::main::CtAudioFile;

use crate::schemas::drawing::main::CtQuickTimeFile;

use crate::schemas::drawing::main::CtEmbeddedWavAudioFile;

use crate::schemas::drawing::main::CtVideoFile;

use crate::schemas::presentation::main::CtExtensionList;

use crate::schemas::presentation::main::CtCustomerDataList;

use crate::schemas::drawing::main::CtAudioCd;

/**
 * @author : zhilong.zhou
 * @description : CT_ApplicationNonVisualDrawingProps
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtApplicationNonVisualDrawingProps {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@isPhoto")]
    pub is_photo_attr: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@userDrawn")]
    pub user_drawn_attr: Option<String>,

    #[serde(rename(serialize = "p:ph", deserialize = "ph"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ph: Option<CtPlaceholder>,

    #[serde(rename(serialize = "p:custDataLst", deserialize = "custDataLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cust_data_lst: Option<CtCustomerDataList>,

    #[serde(rename(serialize = "p:extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,

    #[serde(rename(serialize = "p:audioCd", deserialize = "audioCd"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_cd: Option<CtAudioCd>,

    #[serde(rename(serialize = "p:wavAudioFile", deserialize = "wavAudioFile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wav_audio_file: Option<CtEmbeddedWavAudioFile>,

    #[serde(rename(serialize = "p:audioFile", deserialize = "audioFile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_file: Option<CtAudioFile>,

    #[serde(rename(serialize = "p:videoFile", deserialize = "videoFile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_file: Option<CtVideoFile>,

    #[serde(rename(serialize = "p:quickTimeFile", deserialize = "quickTimeFile"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_time_file: Option<CtQuickTimeFile>,
}

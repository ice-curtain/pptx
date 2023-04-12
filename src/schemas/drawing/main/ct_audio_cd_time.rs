use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_AudioCDTime
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtAudioCdTime {
    #[serde(rename = "@track")]
    pub track_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@time")]
    pub time_attr: Option<String>,
}

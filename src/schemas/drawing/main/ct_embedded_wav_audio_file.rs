use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_EmbeddedWAVAudioFile
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtEmbeddedWavAudioFile {
    #[serde(rename = "@embed")]
    pub embed_attr: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,
}

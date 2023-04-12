use serde::{Deserialize, Serialize};

use crate::schemas::drawing::diagram::CtDataModel;

/**
 * @author : zhilong.zhou
 * @description : CT_SampleData
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtSampleData {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "@useDef")]
    pub use_def_attr: Option<String>,

    #[serde(rename(serialize = "dataModel", deserialize = "dataModel"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_model: Option<Vec<CtDataModel>>,
}

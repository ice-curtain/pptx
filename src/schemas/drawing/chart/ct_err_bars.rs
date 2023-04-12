use serde::{Deserialize, Serialize};

use crate::schemas::drawing::chart::CtExtensionList;

use crate::schemas::drawing::chart::CtBoolean;

use crate::schemas::drawing::chart::CtNumDataSource;

use crate::schemas::drawing::chart::CtDouble;

use crate::schemas::drawing::chart::CtErrBarType;

use crate::schemas::drawing::chart::CtErrValType;

use crate::schemas::drawing::chart::CtErrDir;

use crate::schemas::drawing::main::CtShapeProperties;

/**
 * @author : zhilong.zhou
 * @description : CT_ErrBars
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtErrBars {
    #[serde(rename(serialize = "errDir", deserialize = "errDir"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub err_dir: Option<CtErrDir>,

    #[serde(rename(serialize = "errBarType", deserialize = "errBarType"))]
    pub err_bar_type: CtErrBarType,

    #[serde(rename(serialize = "errValType", deserialize = "errValType"))]
    pub err_val_type: CtErrValType,

    #[serde(rename(serialize = "noEndCap", deserialize = "noEndCap"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_end_cap: Option<CtBoolean>,

    #[serde(rename(serialize = "plus", deserialize = "plus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plus: Option<CtNumDataSource>,

    #[serde(rename(serialize = "minus", deserialize = "minus"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minus: Option<CtNumDataSource>,

    #[serde(rename(serialize = "val", deserialize = "val"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val: Option<CtDouble>,

    #[serde(rename(serialize = "spPr", deserialize = "spPr"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_pr: Option<Box<CtShapeProperties>>,

    #[serde(rename(serialize = "extLst", deserialize = "extLst"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_lst: Option<CtExtensionList>,
}

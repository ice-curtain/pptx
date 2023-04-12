use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_GeomGuide
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtGeomGuide {
    #[serde(rename = "@name")]
    pub name_attr: String,

    #[serde(rename = "@fmla")]
    pub fmla_attr: String,
}

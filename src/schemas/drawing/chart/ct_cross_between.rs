use serde::{Deserialize, Serialize};

/**
 * @author : zhilong.zhou
 * @description : CT_CrossBetween
 */

#[derive(Serialize, Deserialize, Debug)]

pub struct CtCrossBetween {
    #[serde(rename = "@val")]
    pub val_attr: String,
}

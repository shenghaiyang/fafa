use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "en-US")]
    EnUs,
    #[serde(rename = "zh-CN")]
    ZhCn,
}

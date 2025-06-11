// Modified in 2025 from the original version
// Original source licensed under the Apache License 2.0

use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct QuotedIntWrapper {
    #[serde(with = "crate::serde_utils::quoted_u64::quoted_u64")]
    pub int: u64,
}

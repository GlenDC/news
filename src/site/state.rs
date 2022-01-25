use std::hash::Hasher;

use fnv::FnvHasher;

pub struct SiteState {
    pub info: SiteInfo,
}

impl SiteState {
    pub fn new() -> SiteState {
        let info = SiteInfo {
            version: (|| -> u64 {
                let timestamp_str = env!("VERGEN_BUILD_TIMESTAMP");
                let mut hasher: FnvHasher = Default::default();
                hasher.write(timestamp_str.as_bytes());
                hasher.finish()
            })(),
        };
        SiteState { info }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct SiteInfo {
    pub version: u64,
}

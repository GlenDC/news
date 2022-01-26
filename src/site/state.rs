use std::hash::Hasher;

use fnv::FnvHasher;

pub struct SiteState {
    pub info: SiteInfo,
}

impl SiteState {
    pub fn new() -> SiteState {
        let build_timestamp = env!("VERGEN_BUILD_TIMESTAMP");
        let build_date = build_timestamp.split("T").next().unwrap();
        let git_sha = env!("VERGEN_GIT_SHA");
        let git_sha_short = git_sha.chars().take(8).collect::<String>();
        let info = SiteInfo {
            version: (|| -> u64 {
                let mut hasher: FnvHasher = Default::default();
                hasher.write(build_timestamp.as_bytes());
                hasher.finish()
            })(),
            build_date,
            git_sha,
            git_sha_short,
        };
        SiteState { info }
    }
}

pub struct SiteInfo {
    pub version: u64,
    pub build_date: &'static str,
    pub git_sha: &'static str,
    pub git_sha_short: String,
}

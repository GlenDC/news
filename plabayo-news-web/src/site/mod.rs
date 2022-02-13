use std::hash::Hasher;

use fnv::FnvHasher;
use lazy_static::lazy_static;

pub mod assets;
pub mod l18n;
pub mod pages;
pub mod templates;

lazy_static! {
    static ref SITE_INFO: SiteInfo = SiteInfo::new();
}

pub struct SiteInfo {
    pub version: u64,
    pub build_date: &'static str,
    pub build_semver: &'static str,
    pub git_sha: &'static str,
    pub git_sha_short: String,
    pub repository: &'static str,
}

impl SiteInfo {
    fn new() -> SiteInfo {
        let build_timestamp = env!("VERGEN_BUILD_TIMESTAMP");
        let build_semver = env!("VERGEN_BUILD_SEMVER");
        let build_date = build_timestamp.split('T').next().unwrap();
        let git_sha = env!("VERGEN_GIT_SHA");
        let git_sha_short = git_sha.chars().take(8).collect::<String>();
        SiteInfo {
            version: {
                let mut hasher: FnvHasher = Default::default();
                hasher.write(build_timestamp.as_bytes());
                hasher.finish()
            },
            build_date,
            build_semver,
            git_sha,
            git_sha_short,
            repository: "https://github.com/plabayo/news",
        }
    }
}

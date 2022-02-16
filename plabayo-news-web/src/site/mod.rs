// Plabayo News
// Copyright (C) 2021  Glen Henri J. De Cauwsemaecker
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::hash::Hasher;

use fnv::FnvHasher;
use lazy_static::lazy_static;

pub mod assets;
pub mod extractors;
pub mod l18n;
pub mod middleware;
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

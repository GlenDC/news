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

mod generated;
pub mod models;

pub use generated::{static_response, PageItem, PageItems, PageSearch};

use crate::site::assets;

pub fn page_max_cache_age_sec(root: &str) -> u32 {
    if root.to_lowercase().as_str() == assets::ROOT {
        return 24 * 60 * 60;
    }
    // do no cache at this level for dynamic content,
    // not sure that granular level belongs on this layer either
    0
}

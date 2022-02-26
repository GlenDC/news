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

use plabayo_news_data::models;

pub struct ContentItems {
    pub items: Vec<Item>,
}

pub struct ContentItem {
    pub q: String,
}

pub struct ContentSearch {
    pub q: String,
}

pub struct Item {
    pub id: models::ItemID,
    pub hidden: bool,
    pub modified: bool,
    pub by: String,
    pub by_id: models::UserID,
    pub rel_time: String,
    pub votes: i64,
    pub title: String,
    pub url: Option<Url>,
    pub text: Option<String>,
    pub comments: Vec<models::ItemID>, // TODO
}

pub struct Url {
    pub full: String,
    pub domain: String,
}

impl Item {
    pub fn from_data(data: models::Item) -> Item {
        Item {
            id: data.id,
            hidden: !matches!(data.state, models::ItemState::Alive),
            modified: data.time < data.mod_time,
            by: format!("user#{}", data.by), // TODO: actually fetch user
            by_id: data.by,
            rel_time: "4 hours ago".to_owned(), // actually calculate based on current time
            votes: data.votes,
            title: data.title.unwrap_or_default(),
            url: data.url.map(|url| Url {
                full: url,
                domain: "example.org".to_owned(), // TODO
            }),
            text: data.text,
            comments: vec![],
        }
    }
}

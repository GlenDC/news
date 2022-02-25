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

use std::time::SystemTime;

use crate::models::{Item, ItemState, ItemKind};

#[derive(Clone, Copy)]
pub struct Database{}

impl Database {
    pub async fn get_news_ranked(&self) -> Vec<Item> {
        vec![
            Item{
                id: 1,
                state: ItemState::Alive,
                kind: ItemKind::Story,
                by: 100,
                time: SystemTime::now(),
                mod_time: SystemTime::now(),
                votes: 42,
                text: None,
                parent: None,
                kids: vec![],
                url: Some("https://www.example.org/".to_owned()),
                title: Some("an example news article".to_owned()),
            },
        ]
    }
}

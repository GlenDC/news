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

use std::fs::{self, File};
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use convert_case::{Case, Casing};
use itertools::Itertools;

use crate::i18n::codegen::common::generate_copyright_file_header;
use crate::i18n::config::Pages;

pub fn generate_pages(file_path: &Path, cfg: &Pages) -> Result<()> {
    println!("cargo:rerun-if-changed={}", cfg.path);

    let file = File::create(file_path)
        .with_context(|| format!("create locales rust file at {}", file_path.display()))?;

    let (not_found_template, templates) = get_templates(&cfg.path, &cfg.not_found)
        .with_context(|| format!("get templates for result at {}", file_path.display()))?;

    generate_copyright_file_header(&file).with_context(|| {
        format!(
            "generate locales module copyright (header) in {}",
            file_path.display()
        )
    })?;

    generate_pages_mod_docs(&file).with_context(|| {
        format!(
            "generate pages module docs (header) in {}",
            file_path.display()
        )
    })?;

    let (mut static_pages, mut dynamic_pages) =
        templates
            .into_iter()
            .fold((vec![], vec![]), |(mut sp, mut dp), page| {
                if cfg.static_pages.iter().any(|sp| sp == &page) {
                    sp.push(page);
                } else {
                    dp.push(page);
                }
                (sp, dp)
            });
    static_pages.sort();
    dynamic_pages.sort();

    generate_pages_imports(&file, &dynamic_pages[..])
        .with_context(|| format!("generate pages imports in {}", file_path.display()))?;

    generate_static_pages(
        &file,
        cfg.templates_dir.as_str(),
        &static_pages[..],
        not_found_template.as_str(),
    )?;

    generate_dynamic_pages(&file, cfg.templates_dir.as_str(), &dynamic_pages[..])?;

    Ok(())
}

fn generate_static_pages(
    mut w: impl std::io::Write,
    templates_dir: &str,
    pages: &[String],
    not_found: &str,
) -> Result<()> {
    w.write_all(
        b"//-------------------------------------
//------- STATIC PAGES
//-------------------------------------

",
    )?;

    w.write_all(
        b"pub fn static_response(endpoint: &str, page: PageState) -> Result<HttpResponse> {
    let (mut response, render_result) = match endpoint {
",
    )?;
    for page in pages {
        if page == not_found {
            continue;
        }
        w.write_all(
            format!(
                "        PAGE_{}_ENDPOINT => (HttpResponse::Ok(), Page{}::new(page).render()),
",
                page.to_case(Case::ScreamingSnake),
                page.to_case(Case::Pascal)
            )
            .as_bytes(),
        )?;
    }
    w.write_all(
        format!(
            "        _ => (HttpResponse::NotFound(), Page{}::new(page).render()),
",
            not_found.to_case(Case::Pascal)
        )
        .as_bytes(),
    )?;

    w.write_all(
        b"    };
    let s = render_result.map_err(ErrorInternalServerError)?;
    Ok(response.content_type(\"text/html\").body(s))
}

",
    )?;

    for page in pages {
        if page != not_found {
            w.write_all(
                format!(
                    "const PAGE_{page_upper}_ENDPOINT: &str = \"{page_snake}\";

",
                    page_upper = page.to_case(Case::ScreamingSnake),
                    page_snake = page.to_case(Case::Snake)
                )
                .as_bytes(),
            )?;
        }
        w.write_all(
            format!(
                "#[derive(Template)]
#[template(path = \"{dir}/{page_orig}.html\", escape = \"none\")]
struct Page{page}<'a> {{
    site_info: &'a SiteInfo,
    page: PageState,
}}

impl<'a> Page{page}<'a> {{
    pub fn new(page: PageState) -> Page{page}<'a> {{
        Page{page} {{
            site_info: &SITE_INFO,
            page,
        }}
    }}
}}

",
                dir = templates_dir,
                page_orig = &page,
                page = page.to_case(Case::Pascal)
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}

fn generate_dynamic_pages(
    mut w: impl std::io::Write,
    templates_dir: &str,
    pages: &[String],
) -> Result<()> {
    w.write_all(
        b"//-------------------------------------
//------- DYNAMIC PAGES
//-------------------------------------
",
    )?;

    for page in pages {
        w.write_all(
            format!(
                "
#[derive(Template)]
#[template(path = \"{dir}/{page_orig}.html\")]
pub struct Page{page}<'a> {{
    site_info: &'a SiteInfo,
    page: PageState,
    content: Content{page},
}}

impl<'a> Page{page}<'a> {{
    pub fn new_response(page: PageState, content: Content{page}) -> Result<HttpResponse> {{
        let page = Page{page} {{
            site_info: &SITE_INFO,
            page,
            content,
        }};
        let s = page.render().map_err(ErrorInternalServerError)?;
        Ok(HttpResponse::Ok().content_type(\"text/html\").body(s))
    }}
}}
",
                dir = templates_dir,
                page_orig = &page,
                page = page.to_case(Case::Pascal)
            )
            .as_bytes(),
        )?;
    }

    Ok(())
}

fn get_templates(templates_path: &str, not_found: &str) -> Result<(String, Vec<String>)> {
    let paths = fs::read_dir(templates_path)
        .with_context(|| format!("list all static page templates in {}", templates_path))?;
    let not_found_template = not_found.to_owned();
    let mut templates = vec![not_found_template.clone()];
    for path in paths {
        let path = path
            .with_context(|| format!("list a static page template found in {}", templates_path))?
            .path();
        let name = path
            .file_stem()
            .ok_or_else(|| {
                anyhow!(
                    "get file stem of a static page template found in {}",
                    templates_path
                )
            })?
            .to_str()
            .ok_or_else(|| {
                anyhow!(
                    "convert file stem of static page template found in {} to &str",
                    templates_path
                )
            })?;
        if name != not_found {
            templates.push(name.to_owned());
        }
    }
    Ok((not_found_template, templates))
}

fn generate_pages_mod_docs(mut w: impl std::io::Write) -> Result<()> {
    w.write_all(
        b"//! this pages module is auto-generated by the plabayo-news-builder::i18n crate.
//! DO NOT MODIFY MANUALLY AS IT WILL BE OVERWRITTEN NEXT TIME YOU BUILD USING CARGO!!!
//! ... Best to also not check in this file into remote repo.

",
    )?;
    Ok(())
}

fn generate_pages_imports(mut w: impl std::io::Write, dynamic_pages: &[String]) -> Result<()> {
    w.write_all(
        b"use actix_web::error::ErrorInternalServerError;
use actix_web::{HttpResponse, Result};
use askama::Template;

use crate::site::pages::PageState;
use crate::site::{SiteInfo, SITE_INFO};

use super::models::{",
    )?;
    w.write_all(
        dynamic_pages
            .iter()
            .map(|s| format!("Content{}", s.to_case(Case::Pascal)))
            .join(", ")
            .as_bytes(),
    )?;
    w.write_all(
        b"};

",
    )?;
    Ok(())
}

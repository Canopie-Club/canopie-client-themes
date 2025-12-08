use std::collections::HashMap;

use crate::models::{Menu, MenuItem};
use crate::schema::{menu_items, menus};
use crate::{db::PgPool, schema::route_records};
use crate::{
    models::{Page, PageContent, Project, Website},
    schema::{page_content, pages, projects, websites},
};
use diesel::{ExpressionMethods, JoinOnDsl};
use diesel::{NullableExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};

pub fn get_project(pool: &PgPool, project_id: String) -> Option<Project> {
    if project_id == "No ID" {
        return None;
    }

    let conn = &mut pool.get().unwrap();

    let project_result = projects::table
        .filter(projects::id.eq(project_id))
        .select(Project::as_select())
        .first::<Project>(conn);

    match project_result {
        Ok(project) => Some(project),
        Err(_) => None,
    }
}

pub fn get_website(pool: &PgPool, website_id: String) -> Option<Website> {
    if website_id == "No ID" {
        return None;
    }

    let conn = &mut pool.get().unwrap();

    let website_result = websites::table
        .filter(websites::id.eq(website_id))
        .select(Website::as_select())
        .first::<Website>(conn);

    match website_result {
        Ok(website) => Some(website),
        Err(_) => None,
    }
}

pub fn get_project_from_route(
    pool: &PgPool,
    domain: String,
    subdomain: Option<String>,
) -> Option<Project> {
    let project_id: String = get_project_id(pool, domain, subdomain);
    return get_project(pool, project_id);
}

pub fn get_website_from_route(
    pool: &PgPool,
    domain: String,
    subdomain: Option<String>,
) -> Option<Website> {
    let website_id: String = get_project_id(pool, domain, subdomain);
    return get_website(pool, website_id);
}

pub fn get_pages(
    pool: &PgPool,
    website_id: &String,
    page_route: Option<&str>,
) -> Option<Vec<Page>> {
    let conn = &mut pool.get().unwrap();

    let mut query = pages::table
        .filter(pages::website_id.eq(website_id))
        .into_boxed();

    if let Some(route) = page_route {
        query = query.filter(pages::slug.eq(route.trim_matches('/')));
    }

    let pages_result = query.select(Page::as_select()).load::<Page>(conn);

    match pages_result {
        Ok(pages) => Some(pages),
        Err(_) => None,
    }
}

#[derive(Clone)]
pub struct GetPageResult {
    pub page: Page,
    pub page_content: Option<PageContent>,
}

pub fn get_page(pool: &PgPool, website_id: &String, page_route: &str) -> Option<GetPageResult> {
    let conn = &mut pool.get().unwrap();

    let trimmed_route = page_route.trim_matches('/');

    let page_result = if trimmed_route.is_empty() {
        pages::table
            .filter(pages::website_id.eq(website_id))
            .filter(pages::home.eq(true))
            .select(Page::as_select())
            .first(conn)
    } else {
        pages::table
            .filter(pages::website_id.eq(website_id))
            .filter(pages::slug.eq(trimmed_route))
            .select(Page::as_select())
            .first(conn)
    };

    match page_result {
        Ok(page) => {
            let content_result = page_content::table
                .filter(page_content::page_id.eq(&page.id))
                .select(PageContent::as_select())
                .first::<PageContent>(conn);

            Some(GetPageResult {
                page,
                page_content: content_result.ok(),
            })
        }
        Err(_) => None,
    }
}

pub fn get_page_from_id(pool: &PgPool, page_id: &String) -> Option<(Page, Option<PageContent>)> {
    let conn = &mut pool.get().unwrap();

    let page_result = pages::table
        .filter(pages::id.eq(page_id))
        .select(Page::as_select())
        .first(conn);

    match page_result {
        Ok(page) => {
            let content_result = page_content::table
                .filter(page_content::page_id.eq(&page.id))
                .select(PageContent::as_select())
                .first::<PageContent>(conn);

            Some((page, content_result.ok()))
        }
        Err(_) => None,
    }
}

pub fn get_menus(pool: &PgPool, website_id: &String) -> Vec<(Menu, Vec<MenuItem>)> {
    let conn = &mut pool.get().unwrap();

    let menus = menus::table
        .filter(menus::website_id.eq(website_id))
        .left_join(menu_items::table.on(menu_items::menu_id.eq(menus::id)))
        .order(menu_items::order.asc())
        .select((Menu::as_select(), menu_items::all_columns.nullable()))
        .load::<(Menu, Option<MenuItem>)>(conn)
        .unwrap();

    // Now group them
    let mut grouped: HashMap<String, (Menu, Vec<MenuItem>)> = HashMap::new();

    for (menu, maybe_item) in menus {
        let entry = grouped
            .entry(menu.id.to_string())
            .or_insert_with(|| (menu, Vec::new()));
        if let Some(item) = maybe_item {
            entry.1.push(item);
        }
    }

    // Convert HashMap into Vec if needed
    let menus_with_items: Vec<(Menu, Vec<MenuItem>)> = grouped.into_values().collect();

    menus_with_items
}

pub fn get_website_id(pool: &PgPool, domain: String, subdomain: Option<String>) -> String {
    let conn = &mut pool.get().unwrap();

    let mut query = route_records::table
        .filter(route_records::domain.eq(domain))
        .into_boxed();

    if let Some(sub) = subdomain {
        query = query.filter(route_records::subdomain.eq(sub));
    } else {
        query = query.filter(route_records::subdomain.is_null());
    }

    let website_id: String = query
        .select(route_records::website_id)
        .first(conn)
        .unwrap_or("No ID".to_string());

    website_id
}

pub fn get_project_id(pool: &PgPool, domain: String, subdomain: Option<String>) -> String {
    let conn = &mut pool.get().unwrap();

    let mut query = route_records::table
        .filter(route_records::domain.eq(domain))
        .into_boxed();

    if let Some(sub) = subdomain {
        query = query.filter(route_records::subdomain.eq(sub));
    } else {
        query = query.filter(route_records::subdomain.is_null());
    }

    let project_id = query
        .left_join(websites::table.on(route_records::website_id.eq(websites::id)))
        .select(websites::project_id.nullable())
        .first::<Option<String>>(conn)
        .unwrap_or(Some("No ID".to_string()));

    project_id.unwrap_or("No ID".to_string())
}

use canopie_utils::{
    components::Formatter,
    header::Header,
    renderer::{PageResponse, PageResult},
};
use maud::html;

use crate::templates::single::single_page;

pub fn morningstar_not_found(formatter: Formatter, headers: &mut Header) -> PageResponse {
    PageResponse {
        title: "404 Page not found".to_string(),
        result: PageResult::NotFound(single_page(
            "/404",
            html! { ("404 Page not found!!") },
            headers,
        )),
        formatter,
    }
}

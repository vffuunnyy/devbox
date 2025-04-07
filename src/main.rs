mod app;
mod components;

use app::App;
use sycamore::prelude::*;

fn main() {
    console_error_panic_hook::set_once();
    sycamore::render(|| view! { App {} });
}
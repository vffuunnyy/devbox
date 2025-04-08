use sycamore::prelude::*;

use crate::components::{coders::list::CodersList, hashers::list::HashersList};

#[component]
pub fn Main() -> View {    
    view! {
        div(class="component-container") {
            div(class="header") {
                img(src="/public/logo.png", alt="Dev Utilities Logo", class="logo")
                h1 { "DevBox" }
                p { "DevBox is a collection of tools for developers" }
            }
            
            div(class="search-container") {
                div(class="search-box") {
                    input(placeholder="Search tools...")
                    span(class="shortcut") { "CTRL + F" }
                }
            }
            
            div(class="tool-groups") {
                CodersList()
                HashersList()
            }
        }
    }
}
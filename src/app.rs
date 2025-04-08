use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::components;

#[derive(Route, Debug, Clone, Copy, PartialEq, Eq)]
enum AppRoutes {
    #[to("/")]
    Index,

    #[to("/coders")]
    Coders,

    #[to("/query-json")]
    QueryJsonCoder,

    #[to("/hashers")]
    Hashers,

    #[not_found]
    NotFound,
}


#[component]
pub fn App() -> View {
    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {
            view! {
                div(class="app-container") {
                    nav(class="navbar") {
                        ul(class="navbar-tabs") {
                            li { 
                                a(href="/", rel="external", class=if matches!(route.get(), AppRoutes::Index) { "active" } else { "" }) { 
                                    "Home" 
                                }
                            }
                            li { 
                                a(href="/coders", rel="external", class=if matches!(route.get(), AppRoutes::Coders) { "active" } else { "" }) { 
                                    "Coders" 
                                }
                            }
                            li { 
                                a(href="/hashers", rel="external", class=if matches!(route.get(), AppRoutes::Hashers) { "active" } else { "" }) { 
                                    "Hashers" 
                                }
                            }
                        }
                    }
                    main(class="content") {
                        (match route.get() {
                            AppRoutes::Index => view! { components::main::Main {} },
                            AppRoutes::Coders => view! { components::coders::coders::Coders {} },
                            // AppRoutes::QueryJsonCoder => view! { components::coders::query_json::QueryJson {} },
                            // AppRoutes::Hashers => view! { components::hashers::hashers::Hashers {} },
                            AppRoutes::QueryJsonCoder => view! { p { "WORK IN PROGRESS" } },
                            AppRoutes::Hashers => view! { p { "WORK IN PROGRESS" } },
                            AppRoutes::NotFound => view! { p { "Страница не найдена" } },
                        })
                    }
                }
            }
        })
    }
}

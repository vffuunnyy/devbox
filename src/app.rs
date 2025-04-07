use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

// Импортируем наши компоненты
use crate::components;

// Определяем маршруты приложения
#[derive(Route, Debug, Clone, Copy, PartialEq, Eq)]
enum AppRoutes {
    #[to("/")]
    Index,
    #[to("/base64")]
    Base64Converter,
    // Добавьте сюда другие маршруты для Base58, MD5, SHA и т.д.
    #[not_found]
    NotFound,
}

// Главный компонент приложения
#[component]
pub fn App() -> View {
    view! {
        // Используем HistoryIntegration для роутинга на основе истории браузера
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {
            view! {
                div(class="app-container") {
                    nav(class="sidebar-menu") { // Меню навигации
                        ul {
                            li { a(href="/", rel="external") { "Главная" } }
                            li { a(href="/base64", rel="external") { "Base64" } }
                            // Добавьте ссылки для других компонентов здесь
                        }
                    }
                    main(class="content") { // Основное содержимое, где будут рендериться компоненты
                        // Отображаем компонент в зависимости от текущего маршрута
                        (match route.get() {
                            AppRoutes::Index => view! { p { "Добро пожаловать в DevBox! Выберите инструмент из меню." } },
                            AppRoutes::Base64Converter => view! { components::converters::base64::Base64 {} },
                            AppRoutes::NotFound => view! { p { "Страница не найдена" } },
                            // Добавьте обработку других маршрутов
                        })
                    }
                }
            }
        })
    }
}

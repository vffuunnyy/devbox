use sycamore::prelude::*;

#[component]
pub fn HashersList() -> View {
    let hashers_expanded = create_signal(true);
    let toggle_hashers = move |_| hashers_expanded.set(!hashers_expanded.get());

    view! {
        div(class="tool-group") {
            div(class="tool-group-header", on:click=toggle_hashers) {
                h2 { "Hashers" }
                div(class=format!("toggle-icon {}", if hashers_expanded.get() { "open" } else { "" })) {
                    "▼"
                }
            }
            div(class=format!("tool-group-content {}", if hashers_expanded.get() { "open" } else { "" })) {
                div(class="tools-grid") {
                    a(href="/hashers", class="tool-card") {
                        h3 { "Hashing" }
                        p { "Create SHA, MD, PBKDF2 hashes for your data." }
                    }
                }
            }
        }
    }
}

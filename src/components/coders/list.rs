use sycamore::prelude::*;

#[component]
pub fn CodersList() -> View {
    let coders_expanded = create_signal(true);
    let toggle_coders = move |_| coders_expanded.set(!coders_expanded.get());
    
    view! {
        div(class="tool-group") {
            div(class="tool-group-header", on:click=toggle_coders) {
                h2 { "Coders and Converters" }
                div(class=format!("toggle-icon {}", if coders_expanded.get() { "open" } else { "" })) {
                    "▼"
                }
            }
            div(class=format!("tool-group-content {}", if coders_expanded.get() { "open" } else { "" })) {
                div(class="tools-grid") {
                    a(href="/coders", class="tool-card") {
                        h3 { "Encode/Decode" }
                        p { "Easily encode and decode Base64 data with our online utility, so you can transmit your data safely or decode Base64-encoded strings." }
                    }
                    a(href="/query-json", class="tool-card") {
                        h3 { "Query params <-> JSON" }
                        p { "Convert between URL query parameters and JSON format for easier API integration." }
                    }
                }
            }
        }
    }
}

use base64::{engine::general_purpose::STANDARD as base64_standard, Engine as _};
use sycamore::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum CodeType {
    Base64,
    Base58,
    Url,
}

impl CodeType {
    fn name(&self) -> &'static str {
        match self {
            CodeType::Base64 => "Base64",
            CodeType::Base58 => "Base58",
            CodeType::Url => "URL",
        }
    }

    fn encode(&self, input: &str) -> Result<String, String> {
        match self {
            CodeType::Base64 => Ok(base64_standard.encode(input)),
            CodeType::Base58 => Err("Base58 encoding not implemented yet".to_string()),
            CodeType::Url => Ok(urlencoding::encode(input).to_string()),
        }
    }

    fn decode(&self, input: &str) -> Result<String, String> {
        match self {
            CodeType::Base64 => base64_standard.decode(input).map_err(|_| "Invalid Base64 input".to_string()).map(|bytes| String::from_utf8(bytes).unwrap()),
            CodeType::Base58 => Err("Base58 decoding not implemented yet".to_string()),
            CodeType::Url => urlencoding::decode(input).map_err(|_| "Invalid URL input".to_string()).map(|str| str.to_string()),
        }
    }

    fn action(&self, input: &str, encode_mode: bool) -> Result<String, String> {
        match self {
            CodeType::Base64 => if encode_mode { self.encode(input) } else { self.decode(input) },
            CodeType::Base58 => if encode_mode { self.encode(input) } else { self.decode(input) },
            CodeType::Url => if encode_mode { self.encode(input) } else { self.decode(input) },
        }
    }
}


#[component]
pub fn Coders() -> View {
    let input = create_signal(String::new());
    let output = create_signal(String::new());
    let encode_mode = create_signal(true);
    let code_type = create_signal(CodeType::Base64);

    let do_encode = move || {
        let result = code_type.get().action(&input.get_clone(), encode_mode.get());
        if let Ok(result) = result {
            output.set(result);
        } else {
            output.set(result.unwrap_err());
        }
    };

    let toggle_encode = move |_| {
        encode_mode.set(true);
        if !input.get_clone().is_empty() {
            do_encode();
        }
    };

    let toggle_decode = move |_| {
        encode_mode.set(false);
        if !input.get_clone().is_empty() {
            do_encode();
        }
    };

    let handle_input = move |_| {
        do_encode();
    };

    let set_code_type = move |typ: CodeType| {
        code_type.set(typ);
        if !input.get_clone().is_empty() {
            do_encode();
        }
    };

    let clear = move |_| {
        input.set(String::new());
        output.set(String::new());
    };

    let is_dropdown_open = create_signal(false);
    let toggle_dropdown = move |_| {
        is_dropdown_open.set(!is_dropdown_open.get());
    };

    view! {
        div(class="tool-section") {
            h2 { "Encode/Decode Utility" }

            div(class="form-row mb-4") {
                div(class="custom-select") {
                    div(
                        class=format!("custom-select-selected {}", if is_dropdown_open.get() { "open" } else { "" }),
                        on:click=toggle_dropdown
                    ) {
                        (code_type.get().name())
                    }
                    div(
                        class=format!("custom-select-options {}", if is_dropdown_open.get() { "open" } else { "" })
                    ) {
                        div(
                            class=format!("option {}", if matches!(code_type.get(), CodeType::Base64) { "selected" } else { "" }),
                            on:click=move |_| {
                                set_code_type(CodeType::Base64);
                                is_dropdown_open.set(false);
                            }
                        ) {
                            "Base64"
                        }
                        div(
                            class=format!("option {}", if matches!(code_type.get(), CodeType::Base58) { "selected" } else { "" }),
                            on:click=move |_| {
                                set_code_type(CodeType::Base58);
                                is_dropdown_open.set(false);
                            }
                        ) {
                            "Base58"
                        }
                        div(
                            class=format!("option {}", if matches!(code_type.get(), CodeType::Url) { "selected" } else { "" }),
                            on:click=move |_| {
                                set_code_type(CodeType::Url);
                                is_dropdown_open.set(false);
                            }
                        ) {
                            "URL"
                        }
                    }
                }

                div(class="tabs-container") {
                    div(class="tabs") {
                        div(
                            class=format!("tab {}", if encode_mode.get() { "active" } else { "" }),
                            on:click=toggle_encode
                        ) { "Encode" }
                        div(
                            class=format!("tab {}", if !encode_mode.get() { "active" } else { "" }),
                            on:click=toggle_decode
                        ) { "Decode" }
                    }
                }

                button(on:click=clear) { "Clear" }
            }

            div(class="form-group") {
                label {
                    (if encode_mode.get() {
                        format!("Text to encode ({} format)", code_type.get().name())
                    } else {
                        format!("{} to decode", code_type.get().name())
                    })
                }
                textarea(
                    placeholder=if encode_mode.get() {
                        format!("Enter text to encode as {}", code_type.get().name())
                    } else {
                        format!("Enter {} encoded text to decode", code_type.get().name())
                    },
                    bind:value=input,
                    on:input=handle_input
                )
            }

            div(class="form-group") {
                label { "Result" }
                textarea(readonly=true, bind:value=output)
            }
        }
    }
}

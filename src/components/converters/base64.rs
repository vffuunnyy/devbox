use sycamore::prelude::*;
use base64::{engine::general_purpose::STANDARD as base64_standard, Engine as _};

#[component]
pub fn Base64() -> View {
    let input_text = create_signal(String::new());
    let output_text = create_signal(String::new());
    let error_message = create_signal(String::new());

    let encode = move |_| {
        error_message.set(String::new());
        let input = input_text.get_clone();
        if input.is_empty() {
            output_text.set(String::new());
            return;
        }
        let encoded = base64_standard.encode(&*input);
        output_text.set(encoded);
    };

    let decode = move |_| {
        error_message.set(String::new());
        let input = input_text.get_clone();
        if input.is_empty() {
            output_text.set(String::new());
            return;
        }
        match base64_standard.decode(&*input) {
            Ok(decoded_bytes) => {
                match String::from_utf8(decoded_bytes) {
                    Ok(decoded_string) => output_text.set(decoded_string),
                    Err(_) => error_message.set("Ошибка: Не удалось декодировать в UTF-8".to_string()),
                }
            }
            Err(e) => error_message.set(format!("Ошибка декодирования Base64: {}", e)),
        }
    };

    view! {
        div(class="component-container") {
            h2 { "Base64 Конвертер" }
            div(class="input-output-container") {
                textarea(
                    class="textarea-input",
                    placeholder="Введите текст здесь...",
                    bind:value=input_text
                ) {}
                textarea(
                    class="textarea-output",
                    readonly=true,
                    prop:value=output_text.get_clone()
                ) {}
            }
             div(class="button-container") {
                button(class="button-encode", on:click=encode) { "Закодировать" }
                button(class="button-decode", on:click=decode) { "Декодировать" }
            }
            (if !error_message.get_clone().is_empty() {
                view! { p(class="error-message") { (error_message.get_clone()) } }
            } else {
                view! {}
            })

        }
    }
} 
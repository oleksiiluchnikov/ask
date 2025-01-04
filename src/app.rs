use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct PromptArgs<'a> {
    content: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    let prompt_input_ref = use_node_ref();
    let input_value = use_state(|| String::new());

    {
        use_effect(move || {
            spawn_local(async move {
                // Fetch the theme CSS from backend using the invoke command
                let result = invoke("get_theme_css", JsValue::NULL)
                    .await
                    .as_string()
                    .expect("Failed to get theme CSS from backend");

                let css = result.as_str();
                let document = web_sys::window().unwrap().document().unwrap();
                let style_element = document.create_element("style").unwrap();
                style_element.set_text_content(Some(&css));

                // Append the <style> element to the <head> to apply globally
                document
                    .head()
                    .unwrap()
                    .append_child(&style_element)
                    .unwrap();
            });
            || ()
        });
    };

    // Focus the input after component mount using use_effect
    {
        let prompt_input_ref = prompt_input_ref.clone();
        use_effect(move || {
            if let Some(input) = prompt_input_ref.cast::<HtmlInputElement>() {
                input.focus().unwrap();
            }
            || ()
        });
    }

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    let onkeydown = {
        let input_value = input_value.clone();
        Callback::from(move |e: KeyboardEvent| {
            // If the user presses the Enter key, process the input
            if e.key_code() == 13 {
                let value = (*input_value).clone();

                spawn_local(async move {
                    invoke(
                        "process_input",
                        serde_wasm_bindgen::to_value(&PromptArgs { content: &value }).unwrap(),
                    )
                    .await;
                });
            }
            // If the user presses the Escape key, hide the window
            else if e.key_code() == 27 {
                spawn_local(async move {
                    invoke("show_panel", JsValue::NULL).await;
                });
            }
        })
    };

    html! {
        <div class="container">
            <input
                id="prompt-input"
                ref={prompt_input_ref}
                placeholder="Type something..."
                autocomplete="off"
                spellcheck="false"
                oninput={oninput}
                onkeydown={onkeydown}
                autofocus=true
            />
        </div>
    }
}

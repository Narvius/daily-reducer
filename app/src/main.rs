use std::cell::RefCell;

use daily_reducer::{DailyReducer, supported_games};
use web_sys::{HtmlTextAreaElement, wasm_bindgen::JsCast as _};
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let reducer = use_state(|| RefCell::new(DailyReducer::new()));
    let result_ref = use_node_ref();

    let oninput = {
        let result_ref = result_ref.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e
                .target()
                .and_then(|t| t.dyn_into::<HtmlTextAreaElement>().ok())
            {
                let mut reducer = reducer.borrow_mut();
                if reducer.insert(target.value().as_str()) {
                    result_ref
                        .cast::<HtmlTextAreaElement>()
                        .unwrap()
                        .set_value(reducer.to_forum_block().as_str());
                } else {
                    result_ref.cast::<HtmlTextAreaElement>().unwrap().set_value(
                        format!("fucked it I guess via:\n\n{}", target.value()).as_str(),
                    );
                }

                target.set_value("");
            }
        })
    };

    let fragments = supported_games()
        .into_iter()
        .map(|game| {
            html! {
                <div key={game.0}><a target="_blank" href={game.1}>{ format!("# {}", game.0) }</a></div>
            }
        })
        .collect::<Html>();

    html! {
        <div id="root">
            <textarea {oninput} id="paste" rows=1 placeholder="Paste here"/>
            { fragments }
            <textarea ref={result_ref} id="result" cols=80 rows=24 readonly=true/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

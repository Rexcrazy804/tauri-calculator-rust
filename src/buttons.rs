use crate::calculate;
use leptos::*;

#[component]
pub(crate) fn Button(ch: char) -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
		.expect("Failed to Get Context String");

    let update_text = move |_| input_text.update(|text| text.push(ch));

    view! {
        <button
            class:operators = move || !ch.is_ascii_digit()
            on:click = update_text
        >
        {ch}
        </button>
    }
}

#[component]
pub(crate) fn ClearButton() -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
		.expect("Failed to Get Context String");

    let clear_text = move |_| input_text.update(|text| text.clear());

    view! {
        <button
            class="funcop"
            on:click = clear_text
        >
        {'C'}
        </button>
    }
}

#[component]
pub(crate) fn EvalButton() -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
		.expect("Failed to Get Context String");

    let clear_text = move |_| {
        if input_text.get().is_empty() {
            return;
        }
        input_text.set(calculate::compute(&input_text.get()).to_string())
    };

    view! {
        <button
            class="funcop"
            on:click = clear_text
        >
        {'='}
        </button>
    }
}

#[component]
pub(crate) fn BackButton() -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
		.expect("Failed to Get Context String");

    let clear_text = move |_| {
        if input_text.get().is_empty() {
            return;
        }
        input_text.update(|text| {
            let _ = text.pop();
        })
    };

    view! {
        <button
            class="funcop"
            on:click = clear_text
        >
        {'D'}
        </button>
    }
}

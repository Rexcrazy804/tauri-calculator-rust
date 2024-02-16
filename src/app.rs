use leptos::*;
use crate::calculate;
// use serde::{Deserialize, Serialize}; will needa learn to use this bastard

#[component]
pub fn App() -> impl IntoView {
    let input_text = create_rw_signal(String::new());

    provide_context(input_text);

    view! {
        <div class="flexcol">
            <div class = "header">
                <p>
                    {input_text}
                </p>
            </div>

            <div class="flexrow">
                <Button ch='+'/>
                <Button ch='7'/>
                <Button ch='8'/>
                <Button ch='9'/>
                <Button ch='/'/>
            </div>
            <div class="flexrow">
                <Button ch='-'/>
                <Button ch='4'/>
                <Button ch='5'/>
                <Button ch='6'/>
                <Button ch='.'/>
            </div>
            <div class="flexrow">
                <Button ch='*'/>
                <Button ch='1'/>
                <Button ch='2'/>
                <Button ch='3'/>
                <BackButton/>
            </div>
            <div class="flexrow">
                <Button ch='('/>
                <Button ch=')'/>
                <Button ch='0'/>
                <EvalButton/>
                <ClearButton/>
            </div>
        </div>
    }
}

#[component]
fn Button(ch: char) -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
        .expect("Failed to Get Context String");

    let update_text = move |_| {
        input_text.update(|text| text.push(ch))
    };

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
fn ClearButton() -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
        .expect("Failed to Get Context String");

    let clear_text = move |_| {
        input_text.update(|text| text.clear())
    };

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
fn EvalButton() -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
        .expect("Failed to Get Context String");

    let clear_text = move |_| {
        if input_text.get().is_empty() { return }
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
fn BackButton() -> impl IntoView {
    let input_text = use_context::<RwSignal<String>>()
        .expect("Failed to Get Context String");

    let clear_text = move |_| {
        if input_text.get().is_empty() { return }
        input_text.update(|text|{
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

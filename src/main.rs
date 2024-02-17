use buttons::{BackButton, Button, ClearButton, EvalButton};
use leptos::*;

mod buttons;
mod calculate;

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}

#[component]
fn App() -> impl IntoView {
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

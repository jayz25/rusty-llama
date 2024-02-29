use leptos::{*, html::Input};

#[component]
pub fn TypeArea(send: Action<String, Result<String, ServerFnError>>) -> impl IntoView {
    let input_ref: NodeRef<Input> = create_node_ref::<Input>();

    view! {
        <div class="type-area__wrapper">
            <form class="type-area__form" on:submit=move |ev| {
                ev.prevent_default();
                let input = input_ref.get().expect("input to exist");
                send.dispatch(input.value());
                input.set_value("");

            }>
            <div class="type-area__interaction-controls">
                <input type="text" class="type-area__chat-input" node_ref=input_ref/>
                <input type="submit" class="type-area__chat-submit-button" />
            </div>
            </form>
        </div>
        
    }
}
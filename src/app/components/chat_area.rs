use leptos::{html::Div, *};

use crate::model::conversation::Conversation;

#[component]
pub fn ChatArea(conversation: ReadSignal<Conversation>) -> impl IntoView {
    let chat_div_ref = create_node_ref::<Div>();
    create_effect(move |_| {
        conversation.get();
        if let Some(div) = chat_div_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! {
        <div class="chat-area__container" node_ref=chat_div_ref>
            {
                move || conversation.get().messages.iter().map(move |message| {
                    if (message.user) {
                        view! {
                            <div class="chat-area__chat-item-user">
                                {message.text.clone()}
                            </div>
                        }
                    } else {
                        view! {
                            <div class="chat-area__chat-item-bot">
                                {message.text.clone()}
                            </div>
                        }
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}
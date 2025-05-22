use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

const AVATARS: [&str; 6] = [
    "/icons/default.png",
    "/icons/eagle.png",
    "/icons/owl.png",
    "/icons/parrot.png",
    "/icons/parrot(1).png",
    "/icons/pelican.png",
];

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let selected_avatar = use_state(|| AVATARS[0].to_string());
    let user = use_context::<User>().expect("No context found.");
    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let selected_avatar = selected_avatar.clone();
        let user = user.clone();
        Callback::from(move |_| {
            *user.username.borrow_mut() = (*username).clone();
            *user.avatar.borrow_mut() = (*selected_avatar).clone();
        })
    };
    html! {
        <div class="bg-gray-800 flex w-screen h-screen items-center justify-center">
            <div class="bg-white p-8 rounded-lg shadow-lg text-center">
                <h1 class="text-2xl font-bold mb-4">{"Choose your name and avatar"}</h1>
                <input {oninput} placeholder="Username"
                    class="rounded p-2 border mb-4 w-full text-gray-800" />
                <div class="grid grid-cols-2 gap-4 mb-4">
                    {
                        AVATARS.iter().map(|url| {
                            let selected_avatar = selected_avatar.clone();
                            let url_clone = url.to_string();
                            let is_selected = *selected_avatar == url_clone;
                            html! {
                                <img
                                    src={url.to_string()}
                                    class={classes!(
                                        "cursor-pointer", "rounded-full", "w-20", "h-20", "border-4",
                                        if is_selected { "border-blue-600" } else { "border-transparent" }
                                    )}

                                    onclick={Callback::from(move |_| selected_avatar.set(url_clone.clone()))}
                                />
                            }
                        }).collect::<Html>()
                    }
                </div>
                <Link<Route> to={Route::Chat}>
                    <button {onclick}
                        disabled={username.len() < 1}
                        class="bg-violet-600 text-white px-4 py-2 rounded disabled:opacity-50">
                        {"Go Chatting!"}
                    </button>
                </Link<Route>>
            </div>
        </div>
    }
}


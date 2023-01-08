use wasm_bindgen::JsCast;
use yew::prelude::*;
use yew::{html};
use web_sys::{HtmlInputElement, EventTarget};


enum QuizState {
    Incorrect,
    Correct,
    CheckCapitalization,
    Unanswered
}
// struct Page {
//     birthday_boy_raw: String,
//     learning_rust_raw: String
// }

// impl Page {
//     fn html(&mut self) -> Html {
//         let (birthday_boy,set_birthday_boy)=use_state(|| "")
//         let (learning_rust,set_learning_rust)=use_state(|| "")
//         let submit=Callback::from(move |e| {println! {e}})
//         html! {
//             <>
//             <h1>{ "HAPPY BIRTHDAY DOUG!!!!" }</h1>
//             <h2>{ "Now it's time for a little quiz..."}</h2>
//             <label>{"Who is the special birthday boy? "}<input value={birthday_boy}/></label>
//             <br />
//             <label><>{"Who is learning Rust? "}<input /></></label>
//             <br />
//             <button onclick=submit} >{"Check Answers"}</button>
//             </>
//         }
//     }
// }

#[function_component(App)]
fn app() -> Html {
    let quiz_state: UseStateHandle<QuizState> = use_state(|| QuizState::Unanswered);
    let birthday_boy_handle =use_state(String::default);
    let birthday_boy = (*birthday_boy_handle).clone();
    let learning_rust_handle :UseStateHandle<String>=use_state(String::default);
    let learning_rust= (*learning_rust_handle).clone();
    //let refs: Vec<NodeRef>=vec![birthday_boy_input,learning_rust_input];
    let update = | handle : UseStateHandle<String>| {
        let handle = handle.clone();
        let quiz_state=quiz_state.clone();

        Callback::from(move |e : Event| {
            let t :Option<EventTarget> = e.target();
            let el = t.and_then(|tg| tg.dyn_into::<HtmlInputElement>().ok());
            match el {
                Some(el) => {handle.set(el.value()); quiz_state.set(QuizState::Unanswered);},
                None => (),
            }
        })
    };
    let check_answers= {
        let quiz_state=quiz_state.clone();
        Callback::from(move | _e| {
            if birthday_boy == "Doug" && learning_rust == "Nate" {
                quiz_state.set(QuizState::Correct);
            } else if birthday_boy.to_lowercase() == "doug" && learning_rust.to_lowercase() == "nate" {
                quiz_state.set(QuizState::CheckCapitalization);
            } else {
                quiz_state.set(QuizState::Incorrect);
            }
        })
    };

    let clear = {
        let quiz_state=quiz_state.clone();
        Callback::from(move | _e| {
            quiz_state.set(QuizState::Unanswered);
        })
    };

    html! {
        <form method="post" action="javascript:void(0);">
        <h1>{ "HAPPY BIRTHDAY DOUG!!!!" }</h1>
        <h2>{ "Now it's time for a little quiz..."}</h2>
        <label>{"Who is the special birthday boy? "}<input onfocus={clear.clone()} onchange={update(birthday_boy_handle)} name="birthday_boy"  /></label>
        <br />
        <label><>{"Who is learning Rust? "}<input onfocus={clear.clone()} onchange={update(learning_rust_handle)} /></></label>
        <br />
        <button onclick={check_answers}>{"Check Answers"}</button>
        <p>{match *quiz_state {
            QuizState::Incorrect => "Incorrect",
            QuizState::Correct => "Correct",
            QuizState::CheckCapitalization => "Check Capitalization!",
            QuizState::Unanswered => ""

        }}</p>
        </form>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

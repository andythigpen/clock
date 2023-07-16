use js_sys::Array;
use log::info;
use wasm_bindgen::prelude::*;
use web_sys::window;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Carousel)]
pub fn carousel(props: &Props) -> Html {
    let child = use_state(|| props.children.iter().next());
    let children_iter = use_mut_ref(|| props.children.clone().into_iter().cycle());

    use_effect_with_deps(
        {
            let child = child.clone();
            let children_iter = children_iter.clone();
            move |_| {
                let win = window().unwrap();
                let callback = Closure::<dyn Fn()>::wrap(Box::new(move || {
                    child.set((*children_iter).borrow_mut().next());
                }));
                let _ = win.set_interval_with_callback_and_timeout_and_arguments(
                    callback.as_ref().dyn_ref().unwrap(),
                    5000, // TODO: configurable
                    &Array::new(),
                );
                || drop(callback)
            }
        },
        (),
    );

    html! {
        <div class={classes!("flex", "flex-1", "transition")}>
            {(*child).clone()}
        </div>
    }
}

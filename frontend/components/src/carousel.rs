use gloo_timers::callback::{Interval, Timeout};
use web_sys::Element;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub children: Children,
}

#[function_component(Carousel)]
pub fn carousel(props: &Props) -> Html {
    let child = use_state(|| props.children.iter().next());
    let children_iter = use_mut_ref(|| props.children.clone().into_iter().cycle());
    let node = use_node_ref();
    let classes = classes!("flex", "flex-1", "transition", "duration-700");

    use_effect_with_deps(
        {
            let child = child.clone();
            let children_iter = children_iter.clone();
            let node = node.clone();
            let classes = classes.clone();
            move |_| {
                let interval = Interval::new(10000, move || {
                    if let Some(elem) = node.cast::<Element>() {
                        let mut hidden_classes = classes.clone();
                        hidden_classes.push("opacity-0");
                        elem.set_class_name(&hidden_classes.to_string());

                        let classes = classes.clone();
                        let children_iter = children_iter.clone();
                        let child = child.clone();
                        let timeout = Timeout::new(700, move || {
                            child.set((*children_iter).borrow_mut().next());
                            elem.set_class_name(&classes.to_string());
                        });
                        timeout.forget();
                    }
                });
                || {
                    interval.cancel();
                }
            }
        },
        (),
    );

    html! {
        <div ref={node} class={classes}>
            {(*child).clone()}
        </div>
    }
}

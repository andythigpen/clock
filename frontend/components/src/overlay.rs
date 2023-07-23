use stores::WidgetStore;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Overlay)]
pub fn overlay() -> Html {
    let mut classes = classes!(
        "fixed",
        "top-0",
        "left-0",
        "bottom-0",
        "right-0",
        "bg-[#000000]",
        "z-50",
        "transition",
        "duration-[5000ms]",
    );
    let _ = classes!(
        "opacity-0",
        "opacity-10",
        "opacity-20",
        "opacity-30",
        "opacity-40",
        "opacity-50",
        "opacity-60",
        "opacity-70",
        "opacity-80",
        "opacity-90",
        "opacity-100",
    );
    let opacity = use_selector(|s: &WidgetStore| s.opacity);
    classes.push(format!("opacity-{opacity}"));

    html! {
        <div class={classes}>
        </div>
    }
}

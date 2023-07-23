use components::*;
use stores::WidgetStore;
use yew::prelude::*;
use yewdux::prelude::use_selector;

#[function_component(Home)]
pub fn home() -> Html {
    let classes = classes!(
        "bg-gradient-45",
        "from-darkbg-900",
        "via-darkbg-800",
        "to-darkbg-900",
        // "bg-[#000000]",
        "flex",
        "items-center",
        "justify-center",
        "h-screen",
        "w-screen",
        // "animate-gradient",
        "bg-[size:400%_400%]",
        "p-6",
        "gap-16",
        // "opacity-30"
    );

    let display = use_selector(|s: &WidgetStore| s.display);

    html! {
        <div class={classes}>
            if *display {
                <Clock />
                <Carousel />
            } else {
                <span class={classes!("text-7xl", "text-gray-400")}>{"Disabled"}</span>
            }
        </div>
    }
}

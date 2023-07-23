use components::*;
use yew::prelude::*;

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
    html! {
        <div class={classes}>
            <Clock />
            <Carousel />
        </div>
    }
}

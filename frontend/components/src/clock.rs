use yew::prelude::*;

#[function_component(Clock)]
pub fn clock() -> Html {
    html! {
        <div class="flex-1 flex flex-col">
            <div class="text-8xl">
                {"Sat Jul 15"}
            </div>
            <div class="text-[20rem] leading-[20rem] font-dosis">
                {"05:16"}
            </div>
        </div>
    }
}

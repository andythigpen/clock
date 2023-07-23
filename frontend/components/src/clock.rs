use gloo_timers::callback::Interval;
use js_sys::Date;
use yew::prelude::*;

use crate::date_util::*;

#[function_component(Clock)]
pub fn clock() -> Html {
    let now = use_state(|| Date::new_0());

    use_effect_with_deps(
        {
            let now = now.clone();
            move |_| {
                let interval = Interval::new(1000, move || {
                    now.set(Date::new_0());
                });
                || {
                    interval.cancel();
                }
            }
        },
        (),
    );

    html! {
        <div class="flex flex-col min-w-[35%]">
            <div class="text-9xl">
                {formatted_date(&*now)}
            </div>
            <div class="text-[18rem] leading-[18rem]">
                {formatted_time(&*now)}
            </div>
        </div>
    }
}

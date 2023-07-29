use crate::{WeatherCurrent, WeatherForecast, WeatherHumidity, WeatherPrecipitation};
use gloo_timers::callback::{Interval, Timeout};
use stores::{Widget, WidgetStore};
use web_sys::Element;
use yew::prelude::*;
use yewdux::prelude::*;

#[function_component(Carousel)]
pub fn carousel() -> Html {
    let current = use_selector(|s: &WidgetStore| s.current());
    let node = use_node_ref();
    let classes = classes!("flex", "flex-1", "transition", "duration-700");

    use_effect_with_deps(
        {
            let node = node.clone();
            let classes = classes.clone();
            move |_| {
                let interval = Interval::new(15000, move || {
                    if let Some(elem) = node.cast::<Element>() {
                        let mut hidden_classes = classes.clone();
                        hidden_classes.push("opacity-0");
                        elem.set_class_name(&hidden_classes.to_string());

                        let classes = classes.clone();
                        let timeout = Timeout::new(700, move || {
                            Dispatch::<WidgetStore>::new().reduce_mut(|s| s.next());
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

    let widget = (*current).as_ref().map(|c| match c {
        Widget::WeatherCurrent => html! { <WeatherCurrent /> },
        Widget::WeatherForecast(index) => html! { <WeatherForecast index={index} /> },
        Widget::WeatherHumidity => html! { <WeatherHumidity /> },
        Widget::WeatherPrecipitation => html! { <WeatherPrecipitation /> },
    });

    html! {
        <div ref={node} class={classes}>
            {widget}
        </div>
    }
}

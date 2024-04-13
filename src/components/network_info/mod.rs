mod echarts_binding;

use stylist::{yew::styled_component, StyleSource};
use web_sys::HtmlDivElement;
use yew::{classes, html, use_effect, use_node_ref, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct NetworkInfoProps {}

#[styled_component(NetworkInfo)]
pub fn network_info(props: &NetworkInfoProps) -> Html {
    let chart_target = use_node_ref();
    let style = StyleSource::try_from(include_str!("network_info.css")).unwrap();
    {
        let chart_target = chart_target.clone();
        use_effect(move || {
            let chart_div = chart_target.cast::<HtmlDivElement>().unwrap();
            let echart = echarts_binding::init(chart_div);
            || {}
        });
    }
    html! {
        <div class={classes!(style)} ref={chart_target}>
        </div>
    }
}

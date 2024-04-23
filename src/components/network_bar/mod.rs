use stylist::StyleSource;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, Clone, PartialEq, Debug)]
pub struct NetworkBarProps {
    pub upload_mb_amount: f64,
    pub upload_mb_per_second: f64,
    pub download_mb_amount: f64,
    pub download_mb_per_second: f64,
}

#[function_component(NetworkBar)]
pub fn network_bar(props: &NetworkBarProps) -> Html {
    let style = StyleSource::try_from(include_str!("network_bar.css")).unwrap();

    let upload_network_info = format!(
        "↑↑↑ Total: {:.2} MB; Avg {:.2} MB/S",
        props.upload_mb_amount, props.upload_mb_per_second
    );
    let download_network_info = format!(
        "↓↓↓ Total: {:.2} MB; Avg: {:.2} MB/S",
        props.download_mb_amount, props.download_mb_per_second
    );
    html! {
        <div class={style}>
            <div class="upload">
                {upload_network_info}
            </div>
            <div class="download">
                {download_network_info}
            </div>
        </div>
    }
}

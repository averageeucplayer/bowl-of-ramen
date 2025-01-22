use log::info;
use yew::prelude::*;
use app_core::models::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub percentage: f32,
}

#[function_component(DpsBar)]
pub fn dps_bar(props: &ProgressBarProps) -> Html {

    let mut value = String::from("#4ade8022");

    let percentage_u32 = (props.percentage.floor() / 10.0) as u32;
    info!("{}", percentage_u32);
    for index in 1..=10 {
      
        if index > percentage_u32 {
            value += ", #FFFFFF11";
        }
        else {
            value += ", #4ade8033";
        }
    }

    let gradient_style = format!(
        "linear-gradient(to right, {value})"
    );
    let style = format!("background: {};", gradient_style);

    html! {
        <div class="absolute w-full h-6 overflow-hidden">
            <div class="h-full transition-all duration-300 w-full" style={style}>
            </div>
        </div>
    }
}
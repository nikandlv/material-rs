//! Material Design 3 Progress Indicator
//!
//! Supports Linear, Circular, and Wavy progress indicators.

use stylist::css;
use stylist::yew::{use_style, Global};
use yew::prelude::*;

use crate::theme::Theme;
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Progress indicator type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ProgressType {
    /// Horizontal bar.
    #[default]
    Linear,
    /// Circular spinner.
    Circular,
    /// Wavy/squiggly line (expressive variant).
    Wavy,
}

#[derive(Properties, PartialEq)]
pub struct ProgressIndicatorProps {
    /// Progress type.
    #[prop_or_default]
    pub progress_type: ProgressType,

    /// Determinate value (0.0–1.0). None for indeterminate.
    #[prop_or_default]
    pub value: Option<f64>,

    /// HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,
}

#[component]
pub fn ProgressIndicator(props: &ProgressIndicatorProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let track_color = with_alpha(&theme.colors.primary, 0.12).unwrap_or_default();

    let linear_style = use_style!(
        r#"
        width: 100%;
        height: 4px;
        border-radius: 2px;
        background-color: ${track_color};
        overflow: hidden;
        "#,
        track_color = track_color,
    );

    let component_override = theme.component_style("ProgressIndicator").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    match props.progress_type {
        ProgressType::Linear => {
            let indicator_color = &theme.colors.primary;

            let indicator_style = if let Some(val) = props.value {
                format!(
                    "width: {}%; transition: width 500ms cubic-bezier(0.2, 0, 0, 1);",
                    (val.clamp(0.0, 1.0) * 100.0)
                )
            } else {
                "width: 33%; animation: md-linear-indeterminate 2s infinite linear;".into()
            };

            let linear_indicator_class = dynamic_style(format!(
                "height: 100%; border-radius: 2px; background-color: {}; {}",
                indicator_color, indicator_style
            ));

            html! {
                <div class={yew::classes![linear_style.get_class_name().to_string(), &props.class, &component_override]} id={props.id.clone()} role="progressbar"
                    aria-valuemin={"0"}
                    aria-valuemax={"1"}
                    aria-valuenow={props.value.map(|v| v.to_string()).unwrap_or_else(|| "0".into())}
                >
                    <div class={linear_indicator_class} />
                    if props.value.is_none() {
                        <Global css={css!(r#"
                            @keyframes md-linear-indeterminate {
                                0% { transform: translateX(-100%); }
                                50% { transform: translateX(200%); }
                                100% { transform: translateX(400%); }
                            }
                        "#)} />
                    }
                </div>
            }
        }
        ProgressType::Circular => {
            let size = 48u32;
            let stroke_width = 4u32;
            let radius = ((size - stroke_width) / 2) as f64;
            let circumference = 2.0 * std::f64::consts::PI * radius;

            let stroke_dashoffset = if let Some(val) = props.value {
                let progress = val.clamp(0.0, 1.0);
                circumference * (1.0 - progress)
            } else {
                circumference * 0.75 // Indeterminate start
            };

            let svg_animation = if props.value.is_none() {
                "animation: md-circular-rotate 2s linear infinite;"
            } else {
                ""
            };

            let circle_style = if props.value.is_none() {
                "animation: md-circular-dash 1.5s ease-in-out infinite;".to_string()
            } else {
                "transition: stroke-dashoffset 500ms cubic-bezier(0.2, 0, 0, 1);".to_string()
            };

            let svg_class = dynamic_style(format!("transform: rotate(-90deg); {}", svg_animation));
            let circle_class = dynamic_style(circle_style);

            html! {
                <div class={yew::classes![&props.class, &component_override]} id={props.id.clone()} role="progressbar"
                    aria-valuemin={"0"}
                    aria-valuemax={"1"}
                    aria-valuenow={props.value.map(|v| v.to_string()).unwrap_or_else(|| "0".into())}
                    style="display: inline-flex; align-items: center; justify-content: center;"
                >
                    <svg width={size.to_string()} height={size.to_string()} viewBox={format!("0 0 {} {}", size, size)}
                        class={svg_class}>
                        // Track circle
                        <circle
                            cx={(size / 2).to_string()} cy={(size / 2).to_string()} r={radius.to_string()}
                            fill="none"
                            stroke={with_alpha(&theme.colors.primary, 0.12).unwrap_or_default()}
                            stroke-width={stroke_width.to_string()}
                        />
                        // Indicator arc
                        <circle
                            cx={(size / 2).to_string()} cy={(size / 2).to_string()} r={radius.to_string()}
                            fill="none"
                            stroke={theme.colors.primary.clone()}
                            stroke-width={stroke_width.to_string()}
                            stroke-linecap="round"
                            stroke-dasharray={circumference.to_string()}
                            stroke-dashoffset={stroke_dashoffset.to_string()}
                            class={circle_class}
                        />
                    </svg>
                    if props.value.is_none() {
                        <Global css={css!(r#"
                            @keyframes md-circular-rotate {
                                0% { transform: rotate(-90deg); }
                                100% { transform: rotate(270deg); }
                            }
                            @keyframes md-circular-dash {
                                0% { stroke-dasharray: 1, 150; stroke-dashoffset: 0; }
                                50% { stroke-dasharray: 90, 150; stroke-dashoffset: -35; }
                                100% { stroke-dasharray: 90, 150; stroke-dashoffset: -124; }
                            }
                        "#)} />
                    }
                </div>
            }
        }
        ProgressType::Wavy => {
            let indicator_color = theme.colors.primary.clone();
            let track_bg = with_alpha(&theme.colors.primary, 0.20).unwrap_or_default();
            
            // Encode clean SVG sine wave with primary theme color:
            // Symmetrical peaks, width 20, height 12.
            let svg_encoded = format!(
                "data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='20' height='12' viewBox='0 0 20 12'%3E%3Cpath d='M 0 6 Q 5 1, 10 6 T 20 6' fill='none' stroke='{}' stroke-width='3' stroke-linecap='round'/%3E%3C/svg%3E",
                indicator_color.replace("#", "%23")
            );

            let wavy_track_class = dynamic_style(format!(
                "position: absolute; inset-inline-start: 0; inset-inline-end: 0; top: 5px; height: 2px; \
                 background-color: {}; border-radius: 1px;",
                track_bg
            ));
            let wavy_overlay_class = dynamic_style(format!(
                "position: absolute; inset: 0; background-image: url(\"{}\"); \
                 background-repeat: repeat-x; \
                 animation: md-wavy-slide 0.8s linear infinite;",
                svg_encoded
            ));

            html! {
                <div class={yew::classes![&props.class, &component_override]} id={props.id.clone()} role="progressbar"
                    style="width: 100%; height: 12px; overflow: hidden; position: relative; display: flex; align-items: center;"
                >
                    // Straight baseline background track
                    <div class={wavy_track_class} />

                    // Sliding wave overlay
                    <div class={wavy_overlay_class} />

                    <Global css={css!(r#"
                        @keyframes md-wavy-slide {
                            0% { background-position-x: 0px; }
                            100% { background-position-x: 20px; }
                        }
                    "#)} />
                </div>
            }
        }
    }
}
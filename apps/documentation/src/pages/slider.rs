use yew::prelude::*;
use material_rs::components::Slider;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

/// Sliders allow users to make selections from a range of values. They are
/// ideal for adjusting settings such as volume, brightness, or any value
/// that falls within a continuous range. Sliders can display labels,
/// tick marks, and support custom min/max/step intervals.

#[function_component]
pub fn SliderPage() -> Html {
    let basic_value = use_state(|| 30.0f64);
    let labeled_value = use_state(|| 50.0f64);
    let custom_value = use_state(|| 25.0f64);

    html! {
        <>
            <Section title="Slider">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Sliders let users select a value from a continuous range by dragging \
                      a thumb along a track. They support labels, tick marks, custom ranges, \
                      and step increments for precise control."}
                </p>

                // ── Basic Slider ──
                <Demo title="Basic Slider">
                    <div style="width: 100%; max-width: 360px;">
                        <Slider value={*basic_value} onchange={let v = basic_value.clone(); Callback::from(move |val: f64| v.set(val))} />
                    </div>
                </Demo>
                <CodeBlock code={BASIC_CODE.to_string()} language={"rust".to_string()} />

                // ── Slider with Label and Ticks ──
                <Demo title="Slider with Label and Ticks">
                    <div style="width: 100%; max-width: 360px;">
                        <Slider
                            label={format!("Volume: {}", *labeled_value as i32)}
                            value={*labeled_value}
                            min={0.0}
                            max={100.0}
                            step={1.0}
                            ticks={true}
                            onchange={let v = labeled_value.clone(); Callback::from(move |val: f64| v.set(val))}
                        />
                    </div>
                </Demo>
                <CodeBlock code={LABELED_CODE.to_string()} language={"rust".to_string()} />

                // ── Custom Min/Max/Step ──
                <Demo title="Custom Min / Max / Step">
                    <div style="width: 100%; max-width: 360px;">
                        <Slider
                            label={format!("Temperature: {}°C", *custom_value as i32)}
                            value={*custom_value}
                            min={-20.0}
                            max={50.0}
                            step={5.0}
                            ticks={true}
                            onchange={let v = custom_value.clone(); Callback::from(move |val: f64| v.set(val))}
                        />
                    </div>
                </Demo>
                <CodeBlock code={CUSTOM_CODE.to_string()} language={"rust".to_string()} />

                // ── Disabled Slider ──
                <Demo title="Disabled Slider">
                    <div style="width: 100%; max-width: 360px;">
                        <Slider
                            label="Locked value"
                            value={50.0}
                            min={0.0}
                            max={100.0}
                            disabled={true}
                        />
                    </div>
                </Demo>
                <CodeBlock code={DISABLED_CODE.to_string()} language={"rust".to_string()} />

                // ── Props Table ──
                <PropTable rows={vec![
                    PropRow { name: "value".into(), r#type: "f64".into(), default_value: "0.0".into(), description: "Current value of the slider.".into() },
                    PropRow { name: "min".into(), r#type: "f64".into(), default_value: "0.0".into(), description: "Minimum value of the slider range.".into() },
                    PropRow { name: "max".into(), r#type: "f64".into(), default_value: "100.0".into(), description: "Maximum value of the slider range.".into() },
                    PropRow { name: "step".into(), r#type: "f64".into(), default_value: "0.0".into(), description: "Step increment. When 0 the slider moves continuously.".into() },
                    PropRow { name: "label".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Optional label displayed above the slider track.".into() },
                    PropRow { name: "ticks".into(), r#type: "bool".into(), default_value: "false".into(), description: "Shows tick marks at each step interval.".into() },
                    PropRow { name: "disabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Disables interaction and reduces opacity.".into() },
                    PropRow { name: "onchange".into(), r#type: "Callback<f64>".into(), default_value: "Callback::noop()".into(), description: "Fires with the new value when the slider is moved.".into() },
                ]} />
            </Section>
        </>
    }
}

// ── Code snippets ─────────────────────────────────────────────────

const BASIC_CODE: &str = r#"<Slider value={value} onchange={Callback::from(move |val: f64| set_value(val))} />"#;

const LABELED_CODE: &str = r#"<Slider
    label={format!("Volume: {}", value as i32)}
    value={value}
    min={0.0}
    max={100.0}
    step={1.0}
    ticks={true}
    onchange={Callback::from(move |val: f64| set_value(val))}
/>"#;

const CUSTOM_CODE: &str = r#"<Slider
    label={format!("Temperature: {}°C", value as i32)}
    value={value}
    min={-20.0}
    max={50.0}
    step={5.0}
    ticks={true}
    onchange={Callback::from(move |val: f64| set_value(val))}
/>"#;

const DISABLED_CODE: &str = r#"<Slider
    label="Locked value"
    value={50.0}
    min={0.0}
    max={100.0}
    disabled={true}
/>"#;

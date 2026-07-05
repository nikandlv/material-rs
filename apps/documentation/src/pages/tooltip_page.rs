use yew::prelude::*;
use material_rs::components::*;
use material_rs::theme::Theme;

use super::{Section, Demo, CodeBlock, PropTable, PropRow};

#[function_component]
pub fn TooltipPage() -> Html {
    let _theme = use_context::<Theme>().expect("Theme context required");

    let tooltip_props = vec![
        PropRow { name: "text".into(), r#type: "String".into(), default_value: "required".into(), description: "Tooltip text content.".into() },
        PropRow { name: "position".into(), r#type: "TooltipPosition".into(), default_value: "Top".into(), description: "Position relative to the child anchor element.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Anchor element(s) the tooltip attaches to.".into() },
    ];

    let position_props = vec![
        PropRow { name: "Top".into(), r#type: "TooltipPosition".into(), default_value: "Default".into(), description: "Above the anchor, centered horizontally.".into() },
        PropRow { name: "Bottom".into(), r#type: "TooltipPosition".into(), default_value: "".into(), description: "Below the anchor, centered horizontally.".into() },
        PropRow { name: "Left".into(), r#type: "TooltipPosition".into(), default_value: "".into(), description: "Left of the anchor, centered vertically.".into() },
        PropRow { name: "Right".into(), r#type: "TooltipPosition".into(), default_value: "".into(), description: "Right of the anchor, centered vertically.".into() },
        PropRow { name: "TopLeft".into(), r#type: "TooltipPosition".into(), default_value: "".into(), description: "Above the anchor, aligned to the right edge.".into() },
        PropRow { name: "TopRight".into(), r#type: "TooltipPosition".into(), default_value: "".into(), description: "Above the anchor, aligned to the left edge.".into() },
        PropRow { name: "BottomLeft".into(), r#type: "TooltipPosition".into(), default_value: "".into(), description: "Below the anchor, aligned to the right edge.".into() },
        PropRow { name: "BottomRight".into(), r#type: "TooltipPosition".into(), default_value: "".into(), description: "Below the anchor, aligned to the left edge.".into() },
    ];

    html! {
        <>
            <Section title="Tooltip">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Tooltips display informative text when users hover over, focus on, or \
                      touch an element. They are used to label elements that have no visible \
                      text label, such as icon buttons. Tooltips support eight positions \
                      around the anchor element."}
                </p>

                <Demo title="All 8 Positions">
                    <div style="display: flex; gap: 24px; flex-wrap: wrap; justify-content: center; padding: 80px 40px;">
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Top position" position={TooltipPosition::Top}>
                                <Button label="Top" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Bottom position" position={TooltipPosition::Bottom}>
                                <Button label="Bottom" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Left position" position={TooltipPosition::Left}>
                                <Button label="Left" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Right position" position={TooltipPosition::Right}>
                                <Button label="Right" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Top-left position" position={TooltipPosition::TopLeft}>
                                <Button label="TopLeft" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Top-right position" position={TooltipPosition::TopRight}>
                                <Button label="TopRight" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Bottom-left position" position={TooltipPosition::BottomLeft}>
                                <Button label="BottomLeft" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                        <div style="display: flex; flex-direction: column; align-items: center; gap: 8px;">
                            <Tooltip text="Bottom-right position" position={TooltipPosition::BottomRight}>
                                <Button label="BottomRight" variant={ButtonVariant::FilledTonal} />
                            </Tooltip>
                        </div>
                    </div>
                </Demo>
                <CodeBlock code={"// Top (default)\n<Tooltip text=\"Top position\" position={TooltipPosition::Top}>\n    <Button label=\"Top\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>\n\n// Bottom\n<Tooltip text=\"Bottom position\" position={TooltipPosition::Bottom}>\n    <Button label=\"Bottom\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>\n\n// Left\n<Tooltip text=\"Left position\" position={TooltipPosition::Left}>\n    <Button label=\"Left\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>\n\n// Right\n<Tooltip text=\"Right position\" position={TooltipPosition::Right}>\n    <Button label=\"Right\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>\n\n// TopLeft\n<Tooltip text=\"Top-left position\" position={TooltipPosition::TopLeft}>\n    <Button label=\"TopLeft\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>\n\n// TopRight\n<Tooltip text=\"Top-right position\" position={TooltipPosition::TopRight}>\n    <Button label=\"TopRight\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>\n\n// BottomLeft\n<Tooltip text=\"Bottom-left position\" position={TooltipPosition::BottomLeft}>\n    <Button label=\"BottomLeft\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>\n\n// BottomRight\n<Tooltip text=\"Bottom-right position\" position={TooltipPosition::BottomRight}>\n    <Button label=\"BottomRight\" variant={ButtonVariant::FilledTonal} />\n</Tooltip>".to_string()} language={"rust".to_string()} />

                <Demo title="Icon Buttons with Tooltips">
                    <div style="display: flex; gap: 8px; align-items: center;">
                        <Tooltip text="Home" position={TooltipPosition::Bottom}>
                            <IconButton icon="home" variant={IconButtonVariant::Standard} label="Home" />
                        </Tooltip>
                        <Tooltip text="Search" position={TooltipPosition::Bottom}>
                            <IconButton icon="search" variant={IconButtonVariant::Standard} label="Search" />
                        </Tooltip>
                        <Tooltip text="Settings" position={TooltipPosition::Bottom}>
                            <IconButton icon="settings" variant={IconButtonVariant::Standard} label="Settings" />
                        </Tooltip>
                        <Tooltip text="Notifications" position={TooltipPosition::Bottom}>
                            <IconButton icon="notifications" variant={IconButtonVariant::Standard} label="Notifications" />
                        </Tooltip>
                        <Tooltip text="Favorite" position={TooltipPosition::Bottom}>
                            <IconButton icon="favorite" variant={IconButtonVariant::Standard} label="Favorite" />
                        </Tooltip>
                    </div>
                </Demo>
                <CodeBlock code={"<Tooltip text=\"Home\" position={TooltipPosition::Bottom}>\n    <IconButton icon=\"home\" variant={IconButtonVariant::Standard} label=\"Home\" />\n</Tooltip>\n<Tooltip text=\"Search\" position={TooltipPosition::Bottom}>\n    <IconButton icon=\"search\" variant={IconButtonVariant::Standard} label=\"Search\" />\n</Tooltip>\n<Tooltip text=\"Settings\" position={TooltipPosition::Bottom}>\n    <IconButton icon=\"settings\" variant={IconButtonVariant::Standard} label=\"Settings\" />\n</Tooltip>\n<Tooltip text=\"Notifications\" position={TooltipPosition::Bottom}>\n    <IconButton icon=\"notifications\" variant={IconButtonVariant::Standard} label=\"Notifications\" />\n</Tooltip>\n<Tooltip text=\"Favorite\" position={TooltipPosition::Bottom}>\n    <IconButton icon=\"favorite\" variant={IconButtonVariant::Standard} label=\"Favorite\" />\n</Tooltip>".to_string()} language={"rust".to_string()} />

                <PropTable rows={tooltip_props} />
            </Section>

            <Section title="TooltipPosition">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"TooltipPosition determines where the tooltip appears relative to its \
                      anchor element. There are 8 positions: 4 cardinal (Top, Bottom, Left, \
                      Right) and 4 corner positions (TopLeft, TopRight, BottomLeft, BottomRight)."}
                </p>
                <PropTable rows={position_props} />
            </Section>
        </>
    }
}

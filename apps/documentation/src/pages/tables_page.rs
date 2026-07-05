use yew::prelude::*;
use material_rs::components::{Table, TableRow};
use std::collections::HashSet;
use super::{Section, CodeBlock, PropTable, PropRow, Demo};

#[function_component]
pub fn TablesPage() -> Html {
    let row_props = vec![
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "(required)".into(), description: "Unique identifier for the row, used for selection tracking.".into() },
        PropRow { name: "cells".into(), r#type: "Vec<String>".into(), default_value: "(required)".into(), description: "Cell contents for each column, corresponding to the headers.".into() },
    ];

    let table_props = vec![
        PropRow { name: "headers".into(), r#type: "Vec<String>".into(), default_value: "(required)".into(), description: "Column header labels displayed in the table head.".into() },
        PropRow { name: "rows".into(), r#type: "Vec<TableRow>".into(), default_value: "vec![]".into(), description: "Data rows to render. If empty, children are used as fallback.".into() },
        PropRow { name: "select_mode".into(), r#type: "bool".into(), default_value: "false".into(), description: "Enables checkbox selection column and row click selection.".into() },
        PropRow { name: "selected_ids".into(), r#type: "HashSet<String>".into(), default_value: "HashSet::new()".into(), description: "Currently selected row IDs (used with select_mode).".into() },
        PropRow { name: "on_selection_change".into(), r#type: "Callback<HashSet<String>>".into(), default_value: "Callback::noop()".into(), description: "Fires when the selection state changes.".into() },
        PropRow { name: "search_enabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Shows a search bar that filters rows client-side.".into() },
        PropRow { name: "pagination_enabled".into(), r#type: "bool".into(), default_value: "false".into(), description: "Enables client-side pagination with prev/next controls.".into() },
        PropRow { name: "rows_per_page".into(), r#type: "usize".into(), default_value: "5".into(), description: "Number of rows displayed per page when pagination is enabled.".into() },
        PropRow { name: "children".into(), r#type: "Children".into(), default_value: "default".into(), description: "Fallback HTML content (tr/td) when rows is empty.".into() },
        PropRow { name: "id".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "HTML id attribute.".into() },
        PropRow { name: "class".into(), r#type: "String".into(), default_value: "\"\"".into(), description: "Additional CSS class names.".into() },
    ];

    html! {
        <>
            <Section title="Table">
                <p style="font-size: 14px; line-height: 1.6; color: var(--md-sys-color-on-surface-variant); margin-bottom: 24px;">
                    {"Tables display structured data in rows and columns with header labels. \
                      They support checkbox selection mode for multi-row operations, client-side \
                      search filtering, and paginated navigation. The component renders with \
                      hover state layers, responsive horizontal scrolling, and outlined borders."}
                </p>

                // ── Simple Table ──
                <Demo title="Simple Table">
                    <div style="max-width: 640px; width: 100%;">
                        <Table
                            headers={vec!["Name".into(), "Role".into(), "Status".into()]}
                            rows={vec![
                                TableRow { id: "1".into(), cells: vec!["Alice".into(), "Engineer".into(), "Active".into()] },
                                TableRow { id: "2".into(), cells: vec!["Bob".into(), "Designer".into(), "Active".into()] },
                                TableRow { id: "3".into(), cells: vec!["Charlie".into(), "Manager".into(), "Away".into()] },
                            ]}
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<Table\n    headers={vec![\"Name\".into(), \"Role\".into(), \"Status\".into()]}\n    rows={vec![\n        TableRow { id: \"1\".into(), cells: vec![\"Alice\".into(), \"Engineer\".into(), \"Active\".into()] },\n        TableRow { id: \"2\".into(), cells: vec![\"Bob\".into(), \"Designer\".into(), \"Active\".into()] },\n        TableRow { id: \"3\".into(), cells: vec![\"Charlie\".into(), \"Manager\".into(), \"Away\".into()] },\n    ]}\n/>".to_string()} language={"rust".to_string()} />

                // ── Table with Selection ──
                <Demo title="Table with Selection">
                    <TableSelectionDemo />
                </Demo>
                <CodeBlock code={"let selected = use_state(HashSet::new);\n\n<Table\n    headers={vec![\"Name\".into(), \"Role\".into(), \"Status\".into()]}\n    rows={vec![\n        TableRow { id: \"1\".into(), cells: vec![\"Alice\".into(), \"Engineer\".into(), \"Active\".into()] },\n        // ... more rows\n    ]}\n    select_mode={true}\n    selected_ids={(*selected).clone()}\n    on_selection_change={Callback::from(move |ids: HashSet<String>| selected.set(ids))}\n/>".to_string()} language={"rust".to_string()} />

                // ── Table with Search and Pagination ──
                <Demo title="Table with Search and Pagination">
                    <div style="max-width: 640px; width: 100%;">
                        <Table
                            headers={vec!["Name".into(), "Role".into(), "Status".into()]}
                            rows={vec![
                                TableRow { id: "1".into(), cells: vec!["Nikan".into(), "Core Developer".into(), "Active".into()] },
                                TableRow { id: "2".into(), cells: vec!["Gemini".into(), "AI Architect".into(), "Active".into()] },
                                TableRow { id: "3".into(), cells: vec!["Guest".into(), "Tester".into(), "Inactive".into()] },
                                TableRow { id: "4".into(), cells: vec!["Alice".into(), "Security Specialist".into(), "Active".into()] },
                                TableRow { id: "5".into(), cells: vec!["Bob".into(), "DevOps Engineer".into(), "Active".into()] },
                                TableRow { id: "6".into(), cells: vec!["Charlie".into(), "UI Designer".into(), "Active".into()] },
                            ]}
                            search_enabled={true}
                            pagination_enabled={true}
                            rows_per_page={3}
                        />
                    </div>
                </Demo>
                <CodeBlock code={"<Table\n    headers={vec![\"Name\".into(), \"Role\".into(), \"Status\".into()]}\n    rows={vec![\n        TableRow { id: \"1\".into(), cells: vec![\"Nikan\".into(), \"Core Developer\".into(), \"Active\".into()] },\n        TableRow { id: \"2\".into(), cells: vec![\"Gemini\".into(), \"AI Architect\".into(), \"Active\".into()] },\n        // ... more rows\n    ]}\n    search_enabled={true}\n    pagination_enabled={true}\n    rows_per_page={3}\n/>".to_string()} language={"rust".to_string()} />

                // ── Full Featured Table ──
                <Demo title="Full Featured Table">
                    <TableFullDemo />
                </Demo>
                <CodeBlock code={"let selected = use_state(HashSet::new);\n\n<Table\n    headers={vec![\"Name\".into(), \"Role\".into(), \"Status\".into()]}\n    rows={vec![\n        TableRow { id: \"1\".into(), cells: vec![\"Nikan\".into(), \"Core Developer\".into(), \"Active\".into()] },\n        // ... more rows\n    ]}\n    select_mode={true}\n    selected_ids={(*selected).clone()}\n    on_selection_change={Callback::from(move |ids: HashSet<String>| selected.set(ids))}\n    search_enabled={true}\n    pagination_enabled={true}\n    rows_per_page={3}\n/>".to_string()} language={"rust".to_string()} />

                // ── Props Tables ──
                <PropTable rows={table_props} />
                <PropTable rows={row_props} />
            </Section>
        </>
    }
}

#[function_component]
fn TableSelectionDemo() -> Html {
    let selected = use_state(HashSet::new);
    html! {
        <div style="max-width: 640px; width: 100%;">
            <Table
                headers={vec!["Name".into(), "Role".into(), "Status".into()]}
                rows={vec![
                    TableRow { id: "1".into(), cells: vec!["Alice".into(), "Engineer".into(), "Active".into()] },
                    TableRow { id: "2".into(), cells: vec!["Bob".into(), "Designer".into(), "Active".into()] },
                    TableRow { id: "3".into(), cells: vec!["Charlie".into(), "Manager".into(), "Away".into()] },
                    TableRow { id: "4".into(), cells: vec!["Diana".into(), "Analyst".into(), "Active".into()] },
                ]}
                select_mode={true}
                selected_ids={(*selected).clone()}
                on_selection_change={let s = selected.clone(); Callback::from(move |ids: HashSet<String>| s.set(ids))}
            />
        </div>
    }
}

#[function_component]
fn TableFullDemo() -> Html {
    let selected = use_state(HashSet::new);
    html! {
        <div style="max-width: 640px; width: 100%;">
            <Table
                headers={vec!["Name".into(), "Role".into(), "Status".into()]}
                rows={vec![
                    TableRow { id: "1".into(), cells: vec!["Nikan".into(), "Core Developer".into(), "Active".into()] },
                    TableRow { id: "2".into(), cells: vec!["Gemini".into(), "AI Architect".into(), "Active".into()] },
                    TableRow { id: "3".into(), cells: vec!["Guest".into(), "Tester".into(), "Inactive".into()] },
                    TableRow { id: "4".into(), cells: vec!["Alice".into(), "Security Specialist".into(), "Active".into()] },
                    TableRow { id: "5".into(), cells: vec!["Bob".into(), "DevOps Engineer".into(), "Active".into()] },
                    TableRow { id: "6".into(), cells: vec!["Charlie".into(), "UI Designer".into(), "Active".into()] },
                ]}
                select_mode={true}
                selected_ids={(*selected).clone()}
                on_selection_change={let s = selected.clone(); Callback::from(move |ids: HashSet<String>| s.set(ids))}
                search_enabled={true}
                pagination_enabled={true}
                rows_per_page={3}
            />
        </div>
    }
}

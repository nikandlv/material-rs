//! Material Design 3 Table Component
//!
//! A styled data table with headers, responsive scroll columns, hover state layers,
//! and full support for Select Mode (checkbox columns), Search, and Pagination.

use stylist::yew::use_style;
use yew::prelude::*;
use std::collections::HashSet;

use crate::theme::Theme;
use crate::components::{Checkbox, IconButton, IconButtonVariant, TextField, TextFieldVariant};
use crate::utils::color::with_alpha;
use crate::utils::dynamic_style::dynamic_style;

/// Represents a single row in the data table when in data mode.
#[derive(Clone, PartialEq, Debug)]
pub struct TableRow {
    /// Unique identifier for selection modes.
    pub id: String,
    /// List of cell contents corresponding to columns headers.
    pub cells: Vec<String>,
}

#[derive(Properties, PartialEq)]
pub struct TableProps {
    /// Table headers columns labels.
    pub headers: Vec<String>,

    /// Optional HTML id.
    #[prop_or_default]
    pub id: String,

    /// Additional CSS classes.
    #[prop_or_default]
    pub class: String,

    /// Data rows for select, search, and pagination.
    #[prop_or_default]
    pub rows: Vec<TableRow>,

    /// Whether Select Mode is enabled (adds checkboxes column).
    #[prop_or(false)]
    pub select_mode: bool,

    /// Currently selected row IDs.
    #[prop_or_default]
    pub selected_ids: HashSet<String>,

    /// Callback when selection state changes.
    #[prop_or_default]
    pub on_selection_change: Callback<HashSet<String>>,

    /// Whether Client-side Search is enabled.
    #[prop_or(false)]
    pub search_enabled: bool,

    /// Whether Client-side Pagination is enabled.
    #[prop_or(false)]
    pub pagination_enabled: bool,

    /// Number of rows per page.
    #[prop_or(5)]
    pub rows_per_page: usize,

    /// Traditional child rows (`tr` and `td` cells content). Fallback if rows is empty.
    #[prop_or_default]
    pub children: Children,
}

#[component]
pub fn Table(props: &TableProps) -> Html {
    let theme = use_context::<Theme>().expect("Theme context required");

    let search_query = use_state(String::new);
    let search_open = use_state(|| false);
    let current_page = use_state(|| 0usize);

    let style = use_style!(
        r#"
        width: 100%;
        border-collapse: collapse;
        text-align: start;
        font-family: ${font_family}, sans-serif;
        box-sizing: border-box;

        th {
            padding: 12px 16px;
            font-size: 14px;
            font-weight: 600;
            color: ${on_surface};
            border-bottom: 1px solid ${outline};
            background-color: ${bg_header};
        }

        td {
            padding: 16px;
            font-size: 14px;
            color: ${on_surface_variant};
            border-bottom: 1px solid ${outline_variant};
            transition: background-color 150ms cubic-bezier(0.2, 0, 0, 1), color 150ms;
        }

        tr {
            transition: background-color 150ms cubic-bezier(0.2, 0, 0, 1);
        }

        tbody tr.selectable-row:hover {
            background-color: ${bg_hover} !important;
            cursor: pointer;
        }

        tbody tr.selectable-row:active {
            background-color: ${bg_active} !important;
        }
        "#,
        font_family = theme.font_family,
        on_surface = theme.colors.on_surface,
        on_surface_variant = theme.colors.on_surface_variant,
        outline = theme.colors.outline,
        outline_variant = theme.colors.outline_variant,
        bg_header = theme.colors.surface_container_low,
        bg_hover = theme.colors.surface_container_high,
        bg_active = theme.colors.surface_container_highest,
    );

    let on_search_change = {
        let search_query = search_query.clone();
        let current_page = current_page.clone();
        Callback::from(move |val: String| {
            search_query.set(val);
            current_page.set(0);
        })
    };

    let on_prev_page = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            if *current_page > 0 {
                current_page.set(*current_page - 1);
            }
        })
    };

    let on_next_page = {
        let current_page = current_page.clone();
        Callback::from(move |_| {
            current_page.set(*current_page + 1);
        })
    };

    let query = search_query.to_lowercase();
    let filtered_rows: Vec<&TableRow> = if query.is_empty() {
        props.rows.iter().collect()
    } else {
        props.rows
            .iter()
            .filter(|row| {
                row.cells
                    .iter()
                    .any(|cell| cell.to_lowercase().contains(&query))
            })
            .collect()
    };

    let total_filtered = filtered_rows.len();
    let page = *current_page;
    let paginated_rows = if props.pagination_enabled {
        let start = page * props.rows_per_page;
        let end = std::cmp::min(start + props.rows_per_page, total_filtered);
        if start < total_filtered {
            &filtered_rows[start..end]
        } else {
            &[]
        }
    } else {
        &filtered_rows[..]
    };

    let start_idx = page * props.rows_per_page + 1;
    let end_idx = std::cmp::min((page + 1) * props.rows_per_page, total_filtered);
    let prev_disabled = page == 0;
    let next_disabled = (page + 1) * props.rows_per_page >= total_filtered;

    let all_filtered_selected = !filtered_rows.is_empty() && filtered_rows.iter().all(|r| props.selected_ids.contains(&r.id));
    let on_select_all = {
        let selected_ids = props.selected_ids.clone();
        let on_selection_change = props.on_selection_change.clone();
        let filtered_ids: Vec<String> = filtered_rows.iter().map(|r| r.id.clone()).collect();
        Callback::from(move |checked: bool| {
            let mut next = selected_ids.clone();
            if checked {
                for id in &filtered_ids {
                    next.insert(id.clone());
                }
            } else {
                for id in &filtered_ids {
                    next.remove(id);
                }
            }
            on_selection_change.emit(next);
        })
    };

    let on_row_select = {
        let selected_ids = props.selected_ids.clone();
        let on_selection_change = props.on_selection_change.clone();
        Callback::from(move |(row_id, checked): (String, bool)| {
            let mut next = selected_ids.clone();
            if checked {
                next.insert(row_id);
            } else {
                next.remove(&row_id);
            }
            on_selection_change.emit(next);
        })
    };

    // ── Styles ──

    let outer_style = use_style!(
        r#"
        display: flex;
        flex-direction: column;
        width: 100%;
        "#,
    );

    let search_bar_style = use_style!(
        r#"
        padding: 8px 16px;
        display: flex;
        justify-content: flex-end;
        align-items: center;
        gap: 8px;
        flex-wrap: wrap;
        "#,
    );

    let search_input_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        gap: 8px;
        width: 100%;
        min-width: 200px;
        max-width: 400px;
        "#,
    );

    let scroll_container_style = use_style!(
        r#"
        width: 100%;
        overflow-x: auto;
        -webkit-overflow-scrolling: touch;
        border-radius: 8px;
        border: 1px solid ${border};
        "#,
        border = theme.colors.outline_variant,
    );

    let select_header_style = use_style!(
        r#"
        width: 48px;
        text-align: center;
        "#,
    );

    let pagination_style = use_style!(
        r#"
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px 16px;
        gap: 8px;
        flex-wrap: wrap;
        border-top: 1px solid ${border};
        font-family: ${font_family}, sans-serif;
        font-size: 12px;
        color: ${color};
        "#,
        border = theme.colors.outline_variant,
        font_family = theme.font_family,
        color = theme.colors.on_surface_variant,
    );

    let pagination_btns_style = use_style!(
        r#"
        display: flex;
        gap: 8px;
        "#,
    );

    let component_override = theme.component_style("Table").map(|css| dynamic_style(css.to_string())).unwrap_or_default();

    html! {
        <div class={yew::classes![outer_style.get_class_name().to_string(), &component_override]}>
            if props.search_enabled && !props.rows.is_empty() {
                <div class={search_bar_style.get_class_name().to_string()}>
                    if *search_open {
                        <div class={search_input_style.get_class_name().to_string()}>
                            <TextField
                                label="Search table..."
                                value={(*search_query).clone()}
                                onchange={on_search_change.clone()}
                                leading_icon="search"
                                variant={TextFieldVariant::Outlined}
                            />
                            <IconButton
                                icon="close"
                                variant={IconButtonVariant::Standard}
                                label="Close search"
                                onclick={Callback::from({
                                    let search_open = search_open.clone();
                                    let search_query = search_query.clone();
                                    move |_: MouseEvent| {
                                        search_open.set(false);
                                        search_query.set(String::new());
                                    }
                                })}
                            />
                        </div>
                    } else {
                        <IconButton
                            icon="search"
                            variant={IconButtonVariant::Standard}
                            label="Search"
                            onclick={Callback::from({
                                let search_open = search_open.clone();
                                move |_: MouseEvent| search_open.set(true)
                            })}
                        />
                    }
                </div>
            }

            <div
                class={yew::classes![scroll_container_style.get_class_name().to_string(), &props.class]}
                id={props.id.clone()}
            >
                <table class={style.get_class_name().to_string()}>
                    <thead>
                        <tr>
                            if props.select_mode && !props.rows.is_empty() {
                                <th class={select_header_style.get_class_name().to_string()}>
                                    <Checkbox
                                        checked={all_filtered_selected}
                                        onchange={on_select_all}
                                    />
                                </th>
                            }
                            { for props.headers.iter().map(|header| {
                                html! { <th>{ header }</th> }
                            })}
                        </tr>
                    </thead>
                    <tbody>
                        if props.rows.is_empty() {
                            { props.children.clone() }
                        } else {
                            { for paginated_rows.iter().map(|row| {
                                let row_id = row.id.clone();
                                let is_selected = props.selected_ids.contains(&row_id);
                                let on_checkbox_change = {
                                    let on_row_select = on_row_select.clone();
                                    let row_id = row_id.clone();
                                    Callback::from(move |checked: bool| {
                                        on_row_select.emit((row_id.clone(), checked));
                                    })
                                };

                                let on_row_click = {
                                    let on_row_select = on_row_select.clone();
                                    let row_id = row_id.clone();
                                    Callback::from(move |_: MouseEvent| {
                                        on_row_select.emit((row_id.clone(), !is_selected));
                                    })
                                };

                                let row_bg = if is_selected {
                                    with_alpha(&theme.colors.primary, 0.08).unwrap_or_default()
                                } else {
                                    "transparent".to_string()
                                };

                                let row_bg_class = dynamic_style(format!("background-color: {};", row_bg));

                                html! {
                                    <tr
                                        key={row.id.clone()}
                                        class={yew::classes!["selectable-row", row_bg_class]}
                                        onclick={if props.select_mode { Some(on_row_click) } else { None }}
                                    >
                                        if props.select_mode {
                                            <td
                                                class={select_header_style.get_class_name().to_string()}
                                                onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}
                                            >
                                                <Checkbox
                                                    checked={is_selected}
                                                    onchange={on_checkbox_change}
                                                />
                                            </td>
                                        }
                                        { for row.cells.iter().map(|cell| {
                                            html! { <td>{ cell }</td> }
                                        })}
                                    </tr>
                                }
                            })}
                        }
                    </tbody>
                </table>
            </div>

            if props.pagination_enabled && !props.rows.is_empty() && total_filtered > 0 {
                <div class={pagination_style.get_class_name().to_string()}>
                    <span>{ format!("{}-{} of {}", start_idx, end_idx, total_filtered) }</span>
                    <div class={pagination_btns_style.get_class_name().to_string()}>
                        <IconButton icon="chevron_left" disabled={prev_disabled} onclick={on_prev_page} label="Previous Page" />
                        <IconButton icon="chevron_right" disabled={next_disabled} onclick={on_next_page} label="Next Page" />
                    </div>
                </div>
            }
        </div>
    }
}

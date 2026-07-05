//! Accessibility utility helpers.

/// Generates standard ARIA attributes for interactive elements.
pub struct A11y;

impl A11y {
    /// Returns the appropriate `role` for a component.
    pub fn role_for(component: &str) -> &'static str {
        match component {
            "button" => "button",
            "checkbox" => "checkbox",
            "radio" => "radio",
            "switch" => "switch",
            "slider" => "slider",
            "dialog" => "dialog",
            "listbox" => "listbox",
            "option" => "option",
            "tab" => "tab",
            "tablist" => "tablist",
            "tabpanel" => "tabpanel",
            "navigation" => "navigation",
            "banner" => "banner",
            "main" => "main",
            "complementary" => "complementary",
            "alertdialog" => "alertdialog",
            "tooltip" => "tooltip",
            "progressbar" => "progressbar",
            "menu" => "menu",
            "menuitem" => "menuitem",
            _ => "generic",
        }
    }

    /// Returns a common `aria-label` description pattern for toggled states.
    pub fn toggle_label(label: &str, checked: bool) -> String {
        if checked {
            format!("{} (on)", label)
        } else {
            format!("{} (off)", label)
        }
    }
}
//! Material Design 3 Components for Yew
//!
//! This module provides a comprehensive set of MD3 components.

pub mod accordion;
pub mod alert_box;
pub mod avatar;
pub mod badge;
pub mod bottom_sheet;
pub mod box_layout;
pub mod breadcrumb;
pub mod button;
pub mod card;
pub mod carousel;
pub mod checkbox;
pub mod chip;
pub mod code_view;
pub mod container;
pub mod dialog;
pub mod divider;
pub mod grid;
pub mod icon;
pub mod icon_button;
pub mod image_list;
pub mod list;
pub mod menu;
pub mod navigation;
pub mod progress;
pub mod radio;
pub mod ripple;
pub mod select;
pub mod shapes;
pub mod skeleton;
pub mod slider;
pub mod snackbar;
pub mod spacer;
pub mod stepper;
pub mod surface;
pub mod switch;
pub mod table;
pub mod text_area;
pub mod text_field;
pub mod toggle_button_group;
pub mod toolbar;
pub mod tooltip;
pub mod top_app_bar;
pub mod typography;

// Re-exports for convenient access
pub use accordion::Accordion;
pub use alert_box::{AlertBox, AlertSeverity};
pub use avatar::{Avatar, AvatarShape};
pub use badge::{Badge, BadgeSize};
pub use bottom_sheet::{BottomSheet, BottomSheetVariant};
pub use box_layout::{Box, BoxProps, BoxTag};
pub use breadcrumb::{Breadcrumb, BreadcrumbItem};
pub use button::{Button, ButtonProps, ButtonSize, ButtonVariant, SplitButton};
pub use card::{Card, CardVariant};
pub use carousel::{Carousel, CarouselItem, CarouselSize};
pub use checkbox::Checkbox;
pub use chip::{Chip, ChipType};
pub use code_view::{CodeView, highlight::Language as CodeLanguage};
pub use container::{Container, ContainerProps};
pub use dialog::Dialog;
pub use divider::{Divider, DividerVariant};
pub use grid::{Grid, GridProps};
pub use icon::{Icon, IconWeight};
pub use icon_button::{IconButton, IconButtonVariant};
pub use image_list::{ImageList, ImageListItem, ImageListVariant};
pub use list::ListItem;
pub use menu::{Menu, MenuItem};
pub use navigation::{
    DrawerItem, DrawerVariant, NavDestination, NavigationBar, NavigationDrawer, NavigationRail,
    RailDestination, TabBar, TabItem,
};
pub use progress::{ProgressIndicator, ProgressType};
pub use radio::RadioButton;
pub use ripple::Ripple;
pub use select::{Select, SelectOption};
pub use shapes::{Shape, ShapeType};
pub use shapes::{
    ShapeArch, ShapeArrow, ShapeBoom, ShapeBun, ShapeBurst, ShapeClamshell, ShapeFan, ShapeFlower,
    ShapeGem, ShapeGhostIsh, ShapeHeart, ShapeLeafClover4, ShapeLeafClover8, ShapeOval,
    ShapePentagon, ShapePixelCircle, ShapePixelTriangle, ShapePuffy, ShapePuffyDiamond,
    ShapeSemicircle, ShapeSidedCookie4, ShapeSidedCookie6, ShapeSidedCookie7, ShapeSidedCookie9,
    ShapeSidedCookie12, ShapeSlanted, ShapeSoftBoom, ShapeSoftBurst, ShapeSunny, ShapeTriangle,
    ShapeVerySunny,
};
pub use skeleton::{Skeleton, SkeletonCard, SkeletonShape, SkeletonText};
pub use slider::Slider;
pub use snackbar::Snackbar;
pub use spacer::Spacer;
pub use stepper::{Step, Stepper};
pub use surface::SurfaceVariant;
pub use surface::{
    CheckPaperPropsAll, CheckSurfacePropsAll, Paper, PaperProps, PaperPropsBuilder, Surface,
    SurfaceProps, SurfacePropsBuilder,
};
pub use switch::Switch;
pub use table::{Table, TableRow};
pub use text_area::TextArea;
pub use text_field::{TextField, TextFieldVariant};
pub use toggle_button_group::{ToggleButtonGroup, ToggleButtonItem, ToggleGroupMode};
pub use toolbar::Toolbar;
pub use tooltip::{Tooltip, TooltipPosition};
pub use top_app_bar::{TopAppBar, TopAppBarPosition, TopAppBarVariant};
pub use typography::{Typography, TypographyVariant};

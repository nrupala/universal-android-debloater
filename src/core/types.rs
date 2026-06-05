#[cfg(target_os = "android")]
pub use android_types::*;

#[cfg(target_os = "android")]
mod android_types {
    use crate::core::uad_lists::{PackageState, Removal, UadList};

    #[derive(Clone, Debug)]
    pub struct PackageRow {
        pub name: String,
        pub state: PackageState,
        pub description: String,
        pub uad_list: UadList,
        pub removal: Removal,
        pub selected: bool,
        pub current: bool,
    }

    impl PackageRow {
        pub fn new(
            name: &str,
            state: PackageState,
            description: &str,
            uad_list: UadList,
            removal: Removal,
            selected: bool,
            current: bool,
        ) -> Self {
            Self {
                name: name.to_string(),
                state,
                description: description.to_string(),
                uad_list,
                removal,
                selected,
                current,
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct PackageInfo {
        pub i_user: usize,
        pub index: usize,
        pub removal: String,
    }
}

#[cfg(all(not(target_os = "android"), feature = "gui"))]
pub use gui_types::*;

#[cfg(all(not(target_os = "android"), feature = "gui"))]
mod gui_types {
    pub use crate::gui::widgets::package_row::PackageRow;
    pub use crate::gui::views::list::PackageInfo;
}

#[cfg(all(not(target_os = "android"), not(feature = "gui")))]
pub use cli_types::*;

#[cfg(all(not(target_os = "android"), not(feature = "gui")))]
mod cli_types {
    use crate::core::uad_lists::{PackageState, Removal, UadList};

    #[derive(Clone, Debug)]
    pub struct PackageRow {
        pub name: String,
        pub state: PackageState,
        pub description: String,
        pub uad_list: UadList,
        pub removal: Removal,
        pub selected: bool,
        pub current: bool,
    }

    impl PackageRow {
        pub fn new(
            name: &str,
            state: PackageState,
            description: &str,
            uad_list: UadList,
            removal: Removal,
            selected: bool,
            current: bool,
        ) -> Self {
            Self {
                name: name.to_string(),
                state,
                description: description.to_string(),
                uad_list,
                removal,
                selected,
                current,
            }
        }
    }

    #[derive(Debug, Default, Clone)]
    pub struct PackageInfo {
        pub i_user: usize,
        pub index: usize,
        pub removal: String,
    }
}

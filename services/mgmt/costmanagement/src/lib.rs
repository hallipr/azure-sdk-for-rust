#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-06")]
pub mod package_preview_2022_06;
#[cfg(all(feature = "package-preview-2022-06", not(feature = "no-default-tag")))]
pub use package_preview_2022_06::*;
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(all(feature = "package-preview-2022-04", not(feature = "no-default-tag")))]
pub use package_preview_2022_04::*;
#[cfg(feature = "package-preview-2022-02")]
pub mod package_preview_2022_02;
#[cfg(all(feature = "package-preview-2022-02", not(feature = "no-default-tag")))]
pub use package_preview_2022_02::*;
#[cfg(feature = "package-preview-2020-12")]
pub mod package_preview_2020_12;
#[cfg(all(feature = "package-preview-2020-12", not(feature = "no-default-tag")))]
pub use package_preview_2020_12::*;
#[cfg(feature = "package-preview-2020-03")]
pub mod package_preview_2020_03;
#[cfg(all(feature = "package-preview-2020-03", not(feature = "no-default-tag")))]
pub use package_preview_2020_03::*;

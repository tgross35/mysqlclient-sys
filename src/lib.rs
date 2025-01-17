#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

#[allow(dead_code)]
mod bindings {
    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mysql_5_7_x,
        not(target_os = "windows"),
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    include!("../bindings/bindings_5_7_42_x86_64_linux.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mysql_8_0_x,
        not(target_os = "windows"),
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    include!("../bindings/bindings_8_0_36_x86_64_linux.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mysql_8_3_x,
        not(target_os = "windows"),
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    include!("../bindings/bindings_8_3_0_x86_64_linux.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mariadb_10_x,
        not(target_os = "windows"),
        any(target_arch = "x86_64", target_arch = "aarch64")
    ))]
    include!("../bindings/bindings_mariadb_10_11_x86_64_linux.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mysql_8_0_x,
        target_os = "windows",
        target_arch = "x86_64"
    ))]
    include!("../bindings/bindings_8_0_36_x86_64_windows.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mysql_8_0_x,
        target_os = "windows",
        target_arch = "x86"
    ))]
    include!("../bindings/bindings_8_0_36_i868_windows.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mysql_8_3_x,
        target_os = "windows",
        target_arch = "x86_64"
    ))]
    include!("../bindings/bindings_8_3_0_x86_64_windows.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mysql_8_3_x,
        target_os = "windows",
        target_arch = "x86"
    ))]
    include!("../bindings/bindings_8_3_0_i868_windows.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mariadb_10_x,
        target_os = "windows",
        target_arch = "x86_64"
    ))]
    include!("../bindings/bindings_mariadb_10_11_x86_64_windows.rs");

    #[cfg(all(
        not(feature = "buildtime_bindgen"),
        mariadb_10_x,
        target_os = "windows",
        target_arch = "x86"
    ))]
    include!("../bindings/bindings_mariadb_10_11_i686_windows.rs");

    #[cfg(feature = "buildtime_bindgen")]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use self::bindings::*;

// the following type devs are compatibility shims for diesel

#[cfg(all(not(mysql_5_7_x), not(mariadb_10_x)))]
pub type my_bool = bool;

#[cfg(all(not(mysql_5_7_x), not(mariadb_10_x)))]
pub const FALSE: my_bool = false;
#[cfg(all(not(mysql_5_7_x), not(mariadb_10_x)))]
pub const TRUE: my_bool = true;

#[cfg(any(mysql_5_7_x, mariadb_10_x))]
pub const FALSE: my_bool = 0;

#[cfg(any(mysql_5_7_x, mariadb_10_x))]
pub const TRUE: my_bool = 1;

pub const SUPPORTS_MYSQL_SSL_MODE: bool = !cfg!(mariadb_10_x);

#[cfg(mariadb_10_x)]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum mysql_ssl_mode {
    SSL_MODE_DISABLED = 1,
    SSL_MODE_PREFERRED = 2,
    SSL_MODE_REQUIRED = 3,
    SSL_MODE_VERIFY_CA = 4,
    SSL_MODE_VERIFY_IDENTITY = 5,
}

#[cfg(mariadb_10_x)]
pub mod mysql_option {
    /// that's not supported, do not use it
    pub const MYSQL_OPT_SSL_MODE: crate::bindings::mysql_option =
        crate::bindings::mysql_option::MYSQL_SERVER_PUBLIC_KEY;
    pub const MYSQL_OPT_SSL_CA: crate::bindings::mysql_option =
        crate::bindings::mysql_option::MYSQL_OPT_SSL_CA;
    pub const MYSQL_OPT_SSL_CERT: crate::bindings::mysql_option =
        crate::bindings::mysql_option::MYSQL_OPT_SSL_CERT;
    pub const MYSQL_OPT_SSL_KEY: crate::bindings::mysql_option =
        crate::bindings::mysql_option::MYSQL_OPT_SSL_KEY;
    pub const MYSQL_SET_CHARSET_NAME: crate::bindings::mysql_option =
        crate::bindings::mysql_option::MYSQL_SET_CHARSET_NAME;
}

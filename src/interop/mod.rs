#[cfg(feature = "dep_tinystr")]
mod tinystr;

#[cfg(feature = "dep_ascii")]
mod ascii;

#[cfg(all(test, feature = "dep_tinystr"))]
mod tinystr_test;

#[cfg(all(test, feature = "dep_ascii"))]
mod ascii_test;

#![allow(dead_code, warnings)]
mod app;
mod common;
mod evolution;
mod math;
mod nn;
mod physics;
mod storage;
pub use app::TemplateApp;
pub use evolution::evolution;
pub use evolution::RUN_EVOLUTION;
#[macro_export]
macro_rules! error {
	(format, $format_string:literal, $($args:expr),*) => {
		println!("Ошибка на {}:{}. {}", file!(), line!(), format!($format_string, $($args),*))
	};
	(debug, $context:expr) => {
		println!("Ошибка на {}:{}, контекст отладки: {:?}", file!(), line!(), $context)
	};
	() => {
		println!("Ошибка на {}:{}", file!(), line!())
	};
}
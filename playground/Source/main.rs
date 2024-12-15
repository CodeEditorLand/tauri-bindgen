use miette::NamedSource;
use pulldown_cmark::{Options, Parser, html};
use tauri_bindgen_core::GeneratorBuilder;
use wasm_bindgen::prelude::*;
use wit_parser::Interface;

#[wasm_bindgen(module = "/editor/dist/index.js")]
extern {
	fn setup(on_change:&Closure<dyn FnMut(JsValue)>);
	#[wasm_bindgen(js_name = "updateOutput")]
	fn update_output(id:&str, value:&str);
}

fn main() {
	console_error_panic_hook::set_once();

	wasm_logger::init(wasm_logger::Config::default());

	let on_change = Closure::wrap(Box::new(|value:JsValue| {
		let source = value.as_string().unwrap();

		let parse_res = wit_parser::parse_and_resolve_str(&source, |_| false)
			.map_err(|err| err.with_source_code(NamedSource::new("input", source)));

		log::debug!("value: {:?}", parse_res);

		match parse_res {
			Ok(iface) => {
				update_output("errors", "");

				update_output(
					"guest-rust",
					&gen_interface(
						tauri_bindgen_gen_guest_rust::Builder {
							fmt:true,
							unchecked:false,
							no_std:false,
						},
						iface.clone(),
					),
				);

				update_output(
					"host",
					&gen_interface(
						tauri_bindgen_gen_host::Builder { fmt:true, tracing:false, async_:false },
						iface.clone(),
					),
				);

				update_output(
					"guest-js",
					&gen_interface(
						tauri_bindgen_gen_guest_js::Builder { prettier:false, romefmt:false },
						iface.clone(),
					),
				);

				update_output(
					"guest-ts",
					&gen_interface(
						tauri_bindgen_gen_guest_ts::Builder { prettier:false, romefmt:false },
						iface.clone(),
					),
				);

				let markdown = gen_interface(tauri_bindgen_gen_markdown::Builder {}, iface);

				let parser = Parser::new_ext(
					&markdown,
					Options::ENABLE_STRIKETHROUGH
						| Options::ENABLE_FOOTNOTES
						| Options::ENABLE_TABLES
						| Options::ENABLE_TASKLISTS,
				);

				let mut html_output = String::new();

				html::push_html(&mut html_output, parser);

				update_output("markdown", &html_output);
			},
			Err(err) => {
				update_output("errors", &format!("{:?}", err));
			},
		}
	}) as Box<dyn FnMut(JsValue)>);

	setup(&on_change);

	on_change.forget();
}

fn gen_interface<B>(builder:B, iface:Interface) -> String
where
	B: GeneratorBuilder, {
	let mut gen = builder.build(iface);

	gen.to_file().1
}

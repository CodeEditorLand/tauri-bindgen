#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use tauri_bindgen_host::ipc_router_wip::{BuilderExt, Router};
use tracing_subscriber;

tauri_bindgen_host::generate!({
	path: "../greet.wit",
	async: true,
	tracing: true
});

#[derive(Clone, Copy)]
struct GreetCtx;

#[::tauri_bindgen_host::async_trait]
impl greet::Greet for GreetCtx {
	async fn greet(&self, name:String) -> String {
		format!("Hello, {}! You've been greeted from code-generated Rust!", name)
	}
}

fn main() {
	tracing_subscriber::fmt().with_max_level(tracing::Level::TRACE).init();

	let mut router:Router<GreetCtx> = Router::new(GreetCtx {});

	greet::add_to_router(&mut router, |ctx| ctx).unwrap();

	tauri::Builder::default()
		.ipc_router(router)
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}

#[allow(unused_imports, unused_variables, dead_code)]
#[rustfmt::skip]
pub mod chars {
    use ::tauri_bindgen_host::serde;
    use ::tauri_bindgen_host::bitflags;
    pub type A = String;
    pub trait Chars: Sized {
        ///A function that accepts a character
        fn take_char(&self, x: char);
        ///A function that returns a character
        fn return_char(&self) -> A;
    }
    pub fn add_to_router<T, U>(
        router: &mut ::tauri_bindgen_host::ipc_router_wip::Router<T>,
        get_cx: impl Fn(&T) -> &U + Send + Sync + 'static,
    ) -> Result<(), ::tauri_bindgen_host::ipc_router_wip::Error>
    where
        U: Chars + Send + Sync + 'static,
    {
        let wrapped_get_cx = ::std::sync::Arc::new(get_cx);
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "chars",
                "take_char",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                    x: char,
                | -> ::tauri_bindgen_host::anyhow::Result<()> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.take_char(x))
                },
            )?;
        let get_cx = ::std::sync::Arc::clone(&wrapped_get_cx);
        router
            .func_wrap(
                "chars",
                "return_char",
                move |
                    ctx: ::tauri_bindgen_host::ipc_router_wip::Caller<T>,
                | -> ::tauri_bindgen_host::anyhow::Result<A> {
                    let ctx = get_cx(ctx.data());
                    Ok(ctx.return_char())
                },
            )?;
        Ok(())
    }
}

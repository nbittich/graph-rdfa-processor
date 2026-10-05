#![cfg(target_arch = "wasm32")]
mod utils;
use graph_rdfa_processor::RdfaGraph;
use lol_alloc::{AssumeSingleThreaded, FreeListAllocator};
use std::cell::RefCell;
use tortank::turtle::turtle_doc::TurtleDoc;
use wasm_bindgen::prelude::*;
// SAFETY: This application is single threaded, so using AssumeSingleThreaded is allowed.
#[global_allocator]
static ALLOCATOR: AssumeSingleThreaded<FreeListAllocator> =
    unsafe { AssumeSingleThreaded::new(FreeListAllocator::new()) };

thread_local! {
    static JS_UUID_FN: RefCell<Option<js_sys::Function>> = const { RefCell::new(None) };
        static JS_UUID_ERR: RefCell<Option<JsValue>> = const { RefCell::new(None) };
}

fn js_uuid_gen() -> String {
    let f = JS_UUID_FN.with(|f| f.borrow().clone());
    let res = match f {
        Some(f) => f.call0(&JsValue::NULL).and_then(|v| {
            v.as_string()
                .ok_or_else(|| js_err("uuid function must return a string"))
        }),
        None => Err(js_err("uuid function not registered")),
    };
    res.unwrap_or_else(|e| {
        JS_UUID_ERR.with(|slot| {
            slot.borrow_mut().get_or_insert(e);
        });
        String::new()
    })
}

fn take_uuid_err() -> Result<(), JsValue> {
    match JS_UUID_ERR.with(|slot| slot.borrow_mut().take()) {
        Some(e) => Err(e),
        None => Ok(()),
    }
}
/// Registers the JS function for the lifetime of the guard (cleared on drop).
struct UuidFnGuard;

impl Drop for UuidFnGuard {
    fn drop(&mut self) {
        JS_UUID_FN.with(|f| *f.borrow_mut() = None);
        JS_UUID_ERR.with(|e| *e.borrow_mut() = None);
    }
}

fn install_uuid_fn(f: Option<js_sys::Function>) -> (Option<fn() -> String>, Option<UuidFnGuard>) {
    match f {
        Some(f) => {
            JS_UUID_FN.with(|slot| *slot.borrow_mut() = Some(f));
            (Some(js_uuid_gen as fn() -> String), Some(UuidFnGuard))
        }
        None => (None, None),
    }
}

fn js_err<E: std::fmt::Display>(err: E) -> JsValue {
    js_sys::Error::new(&err.to_string()).into()
}

#[wasm_bindgen]
pub fn html_to_rdfa(html: &str, base: &str, well_known_prefix: &str) -> String {
    utils::set_panic_hook();
    let wkp = {
        let wkp = well_known_prefix.trim();
        if wkp.is_empty() { None } else { Some(wkp) }
    };
    RdfaGraph::parse_str(html, base, wkp).unwrap()
}

#[wasm_bindgen]
pub fn rdfa_to_turtle(rdfa_graph: &str, uuid_fn: Option<js_sys::Function>) -> String {
    utils::set_panic_hook();
    let (uuid_fn, _guard) = install_uuid_fn(uuid_fn);

    let res = TurtleDoc::try_from((rdfa_graph, None, uuid_fn));
    take_uuid_err().unwrap();
    let turtle_doc = res.map_err(js_err).unwrap();
    turtle_doc.as_turtle().unwrap()
}

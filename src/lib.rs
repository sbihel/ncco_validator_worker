mod utils;

use cfg_if::cfg_if;
use js_sys::Reflect;
use serde_json::Value;
use valico::json_schema;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, Response, ResponseInit};

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub async fn wasm_entry(req: Request) -> Result<Response, JsValue> {
    let mut res = ResponseInit::new();
    if req.method() != "POST" {
        res.status(405);
        return Response::new_with_opt_str_and_init(None, &res);
    }

    let body = match JsFuture::from(req.json()?).await {
        Ok(b) => b,
        Err(e) => {
            res.status(400);
            res.status_text(
                &Reflect::get(&e, &JsValue::from_str("message"))?
                    .as_string()
                    .unwrap(),
            );
            return Response::new_with_opt_str_and_init(None, &res);
        }
    };
    let ncco = match body.into_serde() {
        Ok(n) => n,
        Err(e) => {
            res.status(400);
            res.status_text(&format!("{}", e));
            return Response::new_with_opt_str_and_init(None, &res);
        }
    };

    // Need the schema at compile time because we cannot access the filesystem.
    let file = include_str!("../ncco.schema");
    let json_v4_schema: Value = serde_json::from_str(file).unwrap();
    let mut scope = json_schema::Scope::new();
    let schema = scope
        .compile_and_return(json_v4_schema.clone(), false)
        .unwrap();

    let validation = schema.validate(&ncco);
    if validation.is_valid() {
        res.status(200);
    } else {
        res.status(400);
        res.status_text(&format!("{:?}", validation.errors));
    }

    Response::new_with_opt_str_and_init(None, &res)
}

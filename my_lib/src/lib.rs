use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Example async function returning Promise
use wasm_bindgen_futures::future_to_promise;
use js_sys::Promise;

#[wasm_bindgen]
pub fn async_compute(x: i32) -> Promise {
    let fut = async move {
        // simulate async work
        let res = x * 2;
        Ok(JsValue::from(res))
    };
    future_to_promise(fut)
}



// brew redis
// first tries to find the tech stack, then scan their tech stacks
// 'grype
// to search for vulnerabilities
// grype redis::latest
// some are being high [cve-2025-6080]

// google this: cve-2025-6080 from us webpage, which wil =l give you similar kind of src code
// [vulnerabilities management]
// you can find there, how to re-create that vulnerabilities [also there will recent reports available]

// you can build security report for your product
// grype cgr.dev/chainguard/redis

// sbom [software biil of material]
// sygt redis:latest [important] -> to find fixed version

// nvd - open source database
// trivi, grype
// wolfi-dev OS


// hardend image -> removing unused dependencies
// shift, grype are playing on docker

// melange - os


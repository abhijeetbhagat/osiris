extern crate either;
extern crate memmap;
extern crate serde;
extern crate serde_derive;

mod parsing;
mod utils;

use neon::prelude::*;
use parsing::parser::Parser;

fn parse(mut cx: FunctionContext) -> JsResult<JsValue> {
    let path = cx.argument::<JsString>(0)?.value();

    if let Ok(info) = Parser::parse(&path) {
        let value = neon_serde::to_value(&mut cx, &info)?;
        Ok(value)
    } else {
        cx.throw_error("Could not parse file")
    }
}

register_module!(mut cx, { cx.export_function("parse", parse) });

extern crate lava_rs;
#[macro_use]
extern crate neon;

use neon::prelude::*;
use lava_rs::player;

fn hello_world(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello world!"))
}

fn init_player(mut cx: FunctionContext) -> JsResult<JsString> {
    player::init_player();
    Ok(cx.string("hello world!"))
}

fn load_file(mut cx: FunctionContext) -> JsResult<JsString> {
    let file_path = cx.argument::<JsString>(0)?.value();
    player::load_file(file_path.to_string());
    Ok(cx.string("hello world!"))
}

register_module!(mut m, {
    m.export_function("helloWorld", hello_world);
    m.export_function("init_player", init_player);
    m.export_function("load_file", load_file)
});

extern crate lava_rs;
#[macro_use]
extern crate neon;

use neon::prelude::*;
use lava_rs::player;

fn init_player(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    player::init_player();
    Ok(cx.undefined())
}

fn load_file(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let file_path = cx.argument::<JsString>(0)?.value();
    player::load_file(file_path.to_string());
    Ok(cx.undefined())
}

fn pause(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    player::pause();
    Ok(cx.undefined())
}

fn stop(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    player::stop();
    Ok(cx.undefined())
}

fn set_volume(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let volume = cx.argument::<JsNumber>(0)?.value() as i32;
    player::set_volume(volume);
    Ok(cx.undefined())
}

fn seek_by_percent(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let percent = cx.argument::<JsNumber>(0)?.value();
    player::seek_by_percent(percent);
    Ok(cx.undefined())
}

fn seek_by_second(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let sec = cx.argument::<JsNumber>(0)?.value() as i32;
    player::seek_by_second(sec);
    Ok(cx.undefined())
}

fn seek_by_position(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let sec = cx.argument::<JsNumber>(0)?.value() as i32;
    player::seek_by_position(sec);
    Ok(cx.undefined())
}

fn get_current_time_length(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let length = player::get_current_time_length();
    Ok(cx.number(length))
}

fn get_current_time_position(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let pos = player::get_current_time_position();
    Ok(cx.number(pos))
}

fn is_stopping(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let is_stopping = player::is_stopping();
    Ok(cx.boolean(is_stopping))
}

register_module!(mut m, {
    m.export_function("init_player", init_player);
    m.export_function("load_file", load_file);
    m.export_function("pause", pause);
    m.export_function("stop", stop);
    m.export_function("set_volume", set_volume);
    m.export_function("seek_by_percent", seek_by_percent);
    m.export_function("seek_by_second", seek_by_second);
    m.export_function("seek_by_position", seek_by_position);
    m.export_function("get_current_time_length", get_current_time_length);
    m.export_function("get_current_time_position", get_current_time_position);
    m.export_function("is_stopping", is_stopping)
});

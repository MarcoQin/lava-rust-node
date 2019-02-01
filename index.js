"use strict";
Object.defineProperty(exports, "__esModule", { value: true });
var core = require("./native/index.node");
core.init_player();
function loadFile(filePath) {
    core.load_file(filePath);
}
exports.loadFile = loadFile;
function pause() {
    core.pause();
}
exports.pause = pause;
function stop() {
    core.stop();
}
exports.stop = stop;
function setVolume(volume) {
    core.set_volume(volume);
}
exports.setVolume = setVolume;
function isStopping() {
    return core.is_stopping();
}
exports.isStopping = isStopping;
function seekByPercent(percent) {
    core.seek_by_percent(percent);
}
exports.seekByPercent = seekByPercent;
function seekBySec(sec) {
    core.seek_by_second(sec);
}
exports.seekBySec = seekBySec;
function seekByPosition(pos) {
    core.seek_by_position(pos);
}
exports.seekByPosition = seekByPosition;
function getTimeLength() {
    return core.get_current_time_length();
}
exports.getTimeLength = getTimeLength;
function getCurrentTimePos() {
    return core.get_current_time_position();
}
exports.getCurrentTimePos = getCurrentTimePos;

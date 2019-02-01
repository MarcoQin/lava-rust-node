import * as core from "./native/index.node";

core.init_player();

export function loadFile(filePath: string) {
  core.load_file(filePath);
}

export function pause() {
  core.pause();
}

export function stop() {
  core.stop();
}

export function setVolume(volume: number) {
  core.set_volume(volume);
}

export function isStopping(): boolean {
  return core.is_stopping()
}

export function seekByPercent(percent: number) {
  core.seek_by_percent(percent);
}

export function seekBySec(sec: number) {
  core.seek_by_second(sec);
}

export function seekByPosition(pos: number) {
  core.seek_by_position(pos);
}

export function getTimeLength(): number {
  return core.get_current_time_length();
}

export function getCurrentTimePos(): number {
  return core.get_current_time_position();
}

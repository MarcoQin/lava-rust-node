const player = require('./native/index.node');


console.log(player);
player.init_player();
player.load_file("/Users/marcoqin/Music/网易云音乐/帆足圭吾 - 遊園施設.mp3");

setTimeout(function () {
  console.log("exit");
}, 13000);

console.log("hahaha");

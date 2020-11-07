const debug = true;

import { Game, Turn } from "flipout";

function event_to_grid(e) {
  const dimension = 80;
  const offset = 40;
  return {
    x: Math.floor((e.offsetX - offset) / dimension),
    y: Math.floor((e.offsetY - offset) / dimension)
  }
}

////////////////////////////////////////////////////////////////
/// Mouse event handlers
////////////////////////////////////////////////////////////////

let current_mouse_pointer = 'default';

function handle_mousemove(ev) {
  let grid = event_to_grid(ev);
  let mouse_pointer = 'default';

  if (false) { // XXX: TODO: change if mouse is over the HINT grid
    mouse_pointer = 'pointer';
  }

  if (mouse_pointer != current_mouse_pointer) {
    current_mouse_pointer = mouse_pointer;
    document.getElementById("canvas").style.cursor = current_mouse_pointer;
  }
}

function handle_pass(ev, game) {
  if (debug) {
    console.log("Pass clicked");
  }

  if (game.ui_pass(Turn.Black)) {
    setTimeout(() => ai_action(), 2000);
  }
}

function handle_click(ev, game) {
  let grid = event_to_grid(ev);

  if (debug) {
    console.log("Grid clicked: ", grid.x, grid.y);
  }

  if (game.ui_move(Turn.Black, grid.x, grid.y)) {
    setTimeout(() => ai_action(), 2000);
  }
}

function ai_action() {
  if (!game.ai_action(Turn.White)) {
    setTimeout(() => ai_action(Turn.White), 2000);
  }
}

////////////////////////////////////////////////////////////////
/// main
////////////////////////////////////////////////////////////////

let game = Game.new(); // WASM
let board = document.getElementById('canvas');
let pass = document.getElementById('pass');

board.addEventListener('click', ev => handle_click(ev, game));
pass.addEventListener('click', ev => handle_pass(ev, game));
// board.addEventListener('mousemove', e => handle_mousemove(e));

setTimeout(() => game.update_screen(), 1000);

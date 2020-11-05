const debug = false;

////////////////////////////////////////////////////////////////
/// Sprite
////////////////////////////////////////////////////////////////

const dimension = 80;
const offset = 40;

class Sprite {
  constructor(image_path) {
    this.image = new Image();
    this.image.src = image_path;
  }

  draw_on(ctx, x, y, action = 0) {
    ctx.drawImage(this.image,
                  dimension * action, 0,
                  dimension, dimension,
                  x, y,
                  dimension, dimension);
  }
}

/// clear rectangle
function screen_clear_rect(x, y, width, height) {
  ctx.clearRect(x, y, width, height);
}

/// put sprite on x, y
function screen_put_sprite(x, y, class_id, action = 0) {
  ctx.drawImage(sprites.image,
                dimension * action, dimension * class_id,
                dimension, dimension,
                x, y,
                dimension, dimension);
}

let sprites = new Sprite('assets/sprites.png');
let ctx = document.getElementById('canvas').getContext('2d');

////////////////////////////////////////////////////////////////
/// Grid/Point converters
////////////////////////////////////////////////////////////////

function grid_to_point(x, y) {
  return {
    x: x * dimension + offset,
    y: y * dimension + offset
  }
}

////////////////////////////////////////////////////////////////
/// export functions to WASM
////////////////////////////////////////////////////////////////

export function screen_update_grid(opcode, color, x, y) {
  switch (opcode) {
  case "put":
    screen_put_stone(color, x, y);
    break;
  case "remove":
    screen_remove_stone(x, y);
    break;
  case "flip":
    screen_flip_to(color, x, y, 0);
    break;
  case "hint":
    screen_put_hint(color, x, y);
    break;
  }
}

export function screen_put_stone(color, x, y) {
  let frame = (color == "black" ? 0 : 15);
  let point = grid_to_point(x, y);
  screen_put_sprite(point.x, point.y, 0, frame);
}

export function screen_put_hint(color, x, y) {
  let frame = (color == "black" ? 0 : 15);
  let point = grid_to_point(x, y);
  screen_put_sprite(point.x, point.y, 1, frame);
}

export function screen_remove_stone(x, y) {
  let point = grid_to_point(x, y);
  screen_clear_rect(point.x, point.y, dimension, dimension);
}

export function screen_flip_to(color, x, y, delay) {
  let i = 0 - delay;
  let frame = (color == "white" ? 0 : 15);
  let direc = (color == "white" ? 1 : -1);
  let point = grid_to_point(x, y);

  var id = setInterval(function () {
    if (i >= 0) {
      screen_remove_stone(x, y);
      screen_put_sprite(point.x, point.y, 0, frame);
      frame += direc;
    }
    if (++i === 16) {
      clearInterval(id);
    }
  }, 33.3);
}

export function screen_show_message(id, message) {
  let msg = document.getElementById(id);
  msg.innerText = message;
}

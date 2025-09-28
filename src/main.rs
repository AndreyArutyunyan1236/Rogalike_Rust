use tcod::colors::*;
use tcod::console::*;
use tcod::input::Key;
use tcod::input::KeyCode::*;

fn main() {
    // VARIABLES
    let screen_w: i32 = 80;
    let screen_h: i32 = 50;

    const MAP_W: i32 = 80;
    const MAP_H: i32 = 45;

    const COLOR_DARK_WALL: Color = Color {r: 50, g: 100, b: 50};
    const COLOR_DARK_GROUND: Color = Color {r: 100, g: 100, b: 150};

    let fps_limit = 20;

    let player_x = screen_w /2 - 1;
    let player_y = screen_h /2 - 5;

    let root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(screen_w, screen_h)
        .title("Rusty Game")
        .init();

    let con = Offscreen::new(MAP_W, MAP_H);

    let player = Object::new(player_x, player_y, 'P', BLUE);
    let npc = Object::new(player_x, player_y + 5, 'E', YELLOW);

    let mut objects = [player, npc];

    let game = Game {
        map: make_map(MAP_W, MAP_H),
    };

    let mut tcod: Tcod = Tcod{ root, con };


    // FPS LIMITING
    tcod::system::set_fps(fps_limit);

    // GAME LOOP
    while !tcod.root.window_closed() {
        tcod.con.clear();
        render_all(&mut tcod, &game, &objects, MAP_W, MAP_H, COLOR_DARK_WALL, COLOR_DARK_GROUND, screen_w, screen_h);
        tcod.root.flush();

        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player, &game);

        if exit {
            break;
        }
    }
}

// STRUCTS
struct Tcod {
    root: Root,
    con: Offscreen,
}

#[derive(Debug)]
struct Object {
    x: i32,
    y: i32,
    ch: char,
    color: Color,
}

impl Object {
    pub fn new(x: i32, y: i32, ch: char, color: Color) -> Self {
        Object{ x, y, ch, color }
    }

    pub fn move_to(&mut self, dx: i32, dy: i32, game: &Game) {
        let new_x = self.x + dx;
        let new_y = self.y + dy;

        // Ensure we're within map bounds and not blocked
        if new_x >= 0
            && new_y >= 0
            && (new_x as usize) < game.map.len()
            && (new_y as usize) < game.map[0].len()
            && !game.map[new_x as usize][new_y as usize].blocked
        {
            self.x = new_x;
            self.y = new_y;
        }
    }

    pub fn draw(&self, con: &mut dyn Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.ch, BackgroundFlag::None);
    }
}

#[derive(Copy, Clone, Debug)]
struct Tile {
    blocked: bool,
    block_sight: bool,
}

impl Tile {
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false
        }
    }

    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}

type Map = Vec<Vec<Tile>>;

struct Game {
    map: Map,
}

// FUNCTIONS
fn make_map(map_w: i32, map_h: i32) -> Map {
    let map = vec![vec![Tile::empty(); map_h as usize]; map_w as usize];

    map
}

fn render_all(tcod: &mut Tcod, game: &Game, objects: &[Object; 2],
              w: i32, h: i32, wall_color: Color, ground_color: Color, screen_w: i32, screen_h: i32) {

    for y in 0..h {
        for x in 0..w {
            let wall = game.map[x as usize][y as usize].block_sight;

            if wall {
                tcod.con.set_char_background(x, y, wall_color, BackgroundFlag::Set);
            }
            else {
                tcod.con.set_char_background(x, y, ground_color, BackgroundFlag::Set);
            }
        }
    }

    for obj in objects {
        obj.draw(&mut tcod.con);
    }

    // PASTING CONTENT OF THE CON (CONSOLE) TO ROOT (MAIN CONSOLE)
    blit(
        &tcod.con,
        (0,0),
        (screen_w, screen_h),
        &mut tcod.root,
        (0,0),
        1.0,
        1.0,
    );
}

fn handle_keys(tcod: &mut Tcod, player: &mut Object, game: &Game) -> bool {

    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, ..} =>return true,
        Key { code: Up, .. } => player.move_to(0, -1, game),
        Key { code: Down, .. } => player.move_to(0, 1, game),
        Key { code: Left, .. } => player.move_to(-1, 0, game),
        Key { code: Right, .. } => player.move_to(1, 0, game),
        _ => {},
    }
    false
}
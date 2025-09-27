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

    const COLOR_DARK_WALL: Color = Color {r: 0, g: 0, b: 100};
    const COLOR_DARK_GROUND: Color = Color {r: 50, g: 50, b: 150};

    let fps_limit = 20;

    let player_x = screen_w /2 - 1;
    let player_y = screen_h /2 - 5;

    let root: Root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(screen_w, screen_h)
        .title("Rusty Game")
        .init();

    let con = Offscreen::new(screen_w, screen_h);

    let player = Object::new(player_x, player_y, 'P', BLUE);
    let npc = Object::new(player_x, player_y + 5, 'E', YELLOW);

    let mut objects = [player, npc];

    let mut tcod: Tcod = Tcod{ root, con };


    // FPS LIMITING
    tcod::system::set_fps(fps_limit);

    // GAME LOOP
    while !tcod.root.window_closed() {
        tcod.con.clear();

        for obj in &objects {
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

        tcod.root.flush();

        let player = &mut objects[0];
        let exit = handle_keys(&mut tcod, player);

        if exit {
            break;
        }
    }
}

// FUNCTIONS FOR PLAYER MOVEMENT AND KEY READING

fn handle_keys(tcod: &mut Tcod, player: &mut Object) -> bool {

    let key = tcod.root.wait_for_keypress(true);

    match key {
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = tcod.root.is_fullscreen();
            tcod.root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, ..} =>return true,
        Key { code: Up, .. } => player.move_to(0, -1),
        Key { code: Down, .. } => player.move_to(0, 1),
        Key { code: Left, .. } => player.move_to(-1, 0),
        Key { code: Right, .. } => player.move_to(1, 0),
        _ => {},
    }
    false
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

    pub fn move_to(&mut self, dx:i32, dy: i32) {
        self.x += dx;
        self.y += dy;
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

fn make_map(map_h: i32, map_w: i32) -> Map {
    let mut map = vec![vec![Tile::empty(); map_h as usize]; map_w as usize];

    map
}




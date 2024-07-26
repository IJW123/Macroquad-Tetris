use macroquad::{prelude::*, texture};
use ::rand::prelude::*;

const SCREEN_SIZE: (f32, f32) = (1200., 1200.);
const MAIN_RECTANGLE: (f32, f32) = (500., 1000.);
const NEXT_BLOCK_SQUARE: f32 = 150.;
const OFFSET_X: f32 = SCREEN_SIZE.0 / 2. - MAIN_RECTANGLE.0 / 2.;
const OFFSET_Y: f32 = SCREEN_SIZE.1 / 2. - MAIN_RECTANGLE.1 / 2.;
const SUB_BLOCK_SIZE: f32 = 50.;

type Point = (usize, usize);
type Block = (usize, Texture2D);
type BlockRow = Vec<Block>;
type TetrisField = Vec<BlockRow>;

// Press rotate button -> loops through rotation Vector -> loops through rotation //
// create a Tetris field type that is a Vec filled with row types (which are also vecs) filled with Block types (which are tuples of x 
// coordinates and Textur2D types)

struct TetrisGame {
    grid: TetrisField,
    grid_width: usize,
    grid_height: usize,
}

impl TetrisGame {
    fn new(grid_width: usize, grid_height: usize) -> Self {
        // Initialize the grid with None, indicating all spaces are initially empty
        let grid = vec![vec![None; grid_width]; grid_height];
    }
}

struct Iblock {
    block_coords: Vec<Point>, // Use the type alias for clarity
    direction: Point,
}

impl Iblock {
    fn new() -> Self {
        Iblock {
            block_coords: vec![(0, 0), (1, 0), (2, 0), (3, 0)],
            direction: (0, 1),
        }
    }

    fn rotate(&mut self) {
        // Check alignment by comparing the x-coordinates of the first two blocks
        if self.block_coords[0].0 == self.block_coords[1].0 {
            // Currently vertical, rotate to horizontal
            self.block_coords[0].0 += 1;
            self.block_coords[0].1 -= 1;
            self.block_coords[2].0 -= 1;
            self.block_coords[2].1 += 1;
            self.block_coords[3].0 -= 2;
            self.block_coords[3].1 += 2;
        } else {
            // Currently horizontal, rotate to vertical
            self.block_coords[0].0 -= 1;
            self.block_coords[0].1 += 1;
            self.block_coords[2].0 += 1;
            self.block_coords[2].1 -= 1;
            self.block_coords[3].0 += 2;
            self.block_coords[3].1 -= 2;
        }
    }
}

// block needs to move down at set speed and left or right based on keys pressed, so 3 directions
// collision detection with other blocks and walls by creating a Vec<Vec< of all blocks and checking if the next position is occupied

// I = 0000, O = 00, T = 0 , S = 00, Z = 00 , J = 0  , L =   0, 
//               00     000     00        00      000      000       
// Screen should be 10 blocks wide and 20 blocks tall. so sub_block size of 50 pixels
#[derive(Copy, Clone, Debug)]
enum BlockType {
    I,O,T,S,Z,J,L,
}

struct BlockTextures {
    I: Texture2D,
    O: Texture2D,
    T: Texture2D,
    S: Texture2D,
    Z: Texture2D,
    J: Texture2D,
    L: Texture2D,
}

#[derive(Clone, Debug)]
struct Block {
    top_left: Point,
    block_type: BlockType,
    texture: Texture2D,
    direction: Point,
    rotation: i32,    
}

fn hor_block_size (block: &Block) -> Vec2 {
    match block.block_type {
        BlockType::I => Vec2::new(SUB_BLOCK_SIZE * 4.,SUB_BLOCK_SIZE * 1.),
        BlockType::O => Vec2::new(SUB_BLOCK_SIZE * 2.,SUB_BLOCK_SIZE * 2.),
        BlockType::T => Vec2::new(SUB_BLOCK_SIZE * 3.,SUB_BLOCK_SIZE * 2.),
        BlockType::S => Vec2::new(SUB_BLOCK_SIZE * 3.,SUB_BLOCK_SIZE * 2.),
        BlockType::Z => Vec2::new(SUB_BLOCK_SIZE * 3.,SUB_BLOCK_SIZE * 2.),
        BlockType::J => Vec2::new(SUB_BLOCK_SIZE * 3.,SUB_BLOCK_SIZE * 2.),
        BlockType::L => Vec2::new(SUB_BLOCK_SIZE * 3.,SUB_BLOCK_SIZE * 2.),
    }
}

fn get_texture_for_block_type(textures: &BlockTextures, block_type: BlockType) -> Texture2D {
    match block_type {
        BlockType::I => textures.I.clone(),
        BlockType::O => textures.O.clone(),
        BlockType::T => textures.T.clone(),
        BlockType::S => textures.S.clone(),
        BlockType::Z => textures.Z.clone(),
        BlockType::J => textures.J.clone(),
        BlockType::L => textures.L.clone(),
    }
}

fn pick_block() -> BlockType {
    let number = ::rand::thread_rng().gen_range(1..8);
    match number {
        1 => BlockType::I,
        2 => BlockType::O,
        3 => BlockType::T,
        4 => BlockType::S,
        5 => BlockType::Z,
        6 => BlockType::J,
        7 => BlockType::L,
        _ => unreachable!("number should be within 1..7"), 
    }
}

// fn game_logic(
//     new_block: bool, 
//     last_update: f64, 
//     speed: f64, 
//     block: Block) {
//     if get_time() - last_update > speed {
//         last_update = get_time();
//         draw_texture_ex(
//             &block.texture,
//             300.0,  // X coordinate
//             150.0,  // Y coordinate
//             WHITE,  // Color tint (no tint)
//             DrawTextureParams {
//                 flip_x: false, // Do not flip horizontally
//                 flip_y: true,  // Flip vertically
//                 ..Default::default()
//             }
//         );
//     }

// }
 
#[macroquad::main(window_conf)]
async fn main() {
    let mut score = 0;
    let mut speed = 0.4;
    let mut last_update = get_time();
    let mut navigation_lock = false;
    let mut game_over = false;
    let mut new_block = true;
    let block_type = BlockType::O;
    let block_textures = BlockTextures {
        I: load_texture("assets/I.png").await.unwrap(),
        O: load_texture("assets/O.png").await.unwrap(),
        T: load_texture("assets/T.png").await.unwrap(),
        S: load_texture("assets/S.png").await.unwrap(),
        Z: load_texture("assets/Z.png").await.unwrap(),
        J: load_texture("assets/J.png").await.unwrap(),
        L: load_texture("assets/L.png").await.unwrap(),
    };
    let mut block = Block {
        top_left: (4, 0),
        block_type: block_type,
        texture: Texture2D::empty(),
        direction: (0,1),
        rotation: 0,
    };

    let down = (0, 1);
    let right = (1, 0);
    let left = (-1, 0);
    
    loop {
        clear_background(BLACK);
        // Draw bakcground
        if !game_over {
            clear_background(LIGHTGRAY);

            draw_rectangle(OFFSET_X, OFFSET_Y, MAIN_RECTANGLE.0, MAIN_RECTANGLE.1, BLACK);
            draw_rectangle(950., 100., NEXT_BLOCK_SQUARE, NEXT_BLOCK_SQUARE, BLACK);

            draw_text(format!("SCORE: {score}").as_str(), 10., 20., 20., DARKGRAY);
        } else {
            clear_background(WHITE);
            let text = "Game Over. Press [enter] to play again.";
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);

            draw_text(
                text,
                screen_width() / 2. - text_size.width / 2.,
                screen_height() / 2. + text_size.height / 2.,
                font_size,
                DARKGRAY,
            );
        }

        // draw the game elements
        if !game_over {
            if new_block {
                block.block_type = pick_block();
                block.texture = get_texture_for_block_type(&block_textures, block.block_type);
                println!("type: {:?}, texture {:?}", block.block_type, block.texture);
                block.top_left = (4, 0); // Reset position when a new block is created
                new_block = false;
            }
            let block_size = hor_block_size(&block);
            draw_texture_ex(
                &block.texture,
                block.top_left.0 as f32 * SUB_BLOCK_SIZE + OFFSET_X,  // X coordinate
                block.top_left.1 as f32 * SUB_BLOCK_SIZE + OFFSET_Y,  // Y coordinate
                WHITE,  // Color tint (no tint)
                DrawTextureParams {
                    dest_size: Some(block_size), // Flip vertically
                    ..Default::default()
                }
            );

            if get_time() - last_update > speed {
                last_update = get_time(); 
                block.top_left.1 += 1; 
                println!("{:?}", block.top_left);   
            }

            if block.top_left.1 == 20 {
                new_block = true;
            }                                   
        }
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris".to_owned(),
        window_width: SCREEN_SIZE.0 as i32,
        window_height: SCREEN_SIZE.1 as i32,
        ..Default::default()
    }
}
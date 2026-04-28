use rand::prelude::*;

#[derive(Debug)]
struct Board<T> {
    x: usize,
    y: usize,
    tiles: Vec<Vec<T>>,
}

#[derive(Copy, Clone, Debug)]
struct Tile{
    x: u32,
    y: u32,
    walls: [bool; 4],
    on: u32,
}

fn emptyBoardAssemble(x: i32, y: i32) -> Vec<Vec<Tile>>{
    let mut res: Vec<Vec<Tile>> = vec![];
    for i in (0..y){
        let mut row: Vec<Tile> = vec![];
        for j in (0..x){
            row.push(Tile {x: j as u32, y: i as u32, walls: [false; 4], on: 0})
        }
        res.push(row);
    }
    res
}

//generate random walls on n tiles
fn generateBoard(n: usize, x:i32, y:i32) -> Board<Tile>{

    let mut rng = rand::rng();

    // Generate and shuffle a sequence:
    let mut nums: Vec<i32> = (0..(x*y)).collect();
    nums.shuffle(&mut rng);
    //only n changed tiles
    nums.truncate(n);

    let mut all_tiles: Vec<Vec<Tile>> = emptyBoardAssemble(x,y);

    //change number to position
    for number in nums.iter() {
        let board_x = number / x;
        let board_y = (number - (&board_x * x)) % y;
        
        //
        //println!("{},{},{}", number, board_x, board_y);

        //generate walls
        let custom_distr: [i32; 10] = [0,0,0,0, 1,1,1, 2,2, 3];
        let walls_amount: Option<&i32> = custom_distr.choose(&mut rng);
        

        let mut walls_layout: Vec<i32> = [0,1,2,3].to_vec();
        //what walls will be chosen
        walls_layout.shuffle(&mut rng);
        //layout array
        walls_layout.truncate(*walls_amount.unwrap() as usize);

        //walls placement
        let mut walls: [bool; 4] = [false; 4];
        for wall in walls_layout.iter(){
            walls[*wall as usize] = true;
        }

        //make tile
        let tile: Tile = Tile {
            x: board_x as u32,
            y: board_y as u32,
            walls: walls,
            on: 0
        };

        //replace null tiles with walled tiles
        all_tiles[board_x as usize][board_y as usize] = tile;


    }


    Board {x: x as usize, y: y as usize, tiles: all_tiles}
}

fn main() {

    //making game board
    let mut board: Board<Tile> = generateBoard(12, 6, 7);
    println!("{:?}", board);

    //fixing game board
    //skip for now ig
}

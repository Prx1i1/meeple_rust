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

#[derive(Copy, Clone, Debug)]
struct Position{
    x: i32,
    y: i32,
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

//return array of positions
fn get_adjacent(pos: Position, x: i32, y: i32, tiles: Vec<Vec<Tile>>)
-> Vec<Position>{

    let mut results: Vec<Position> = vec![];

    //in x axis
    for i in [pos.x - 1, pos.x + 1]{
        //basic edge cases
        if i >= 0 && i < x{

            //candidate position and tiles
            let candidate: Position = Position {x: i, y: pos.y};
            let current_tile: Tile = tiles[pos.y as usize][pos.x as usize];
            let checked_tile: Tile = tiles[pos.y as usize][i as usize];
            
            //check walls
            let mut walls_blocked: bool = false;
            if( i > pos.x){
                walls_blocked = current_tile.walls[1] || checked_tile.walls[3];
            }
            if (i < pos.x){
                walls_blocked = current_tile.walls[3] || checked_tile.walls[1];
            }

            //walls ok
            if !walls_blocked{
                results.push(Position {x: i, y: pos.y});
            }

        }
    }

    // for j in (pos.y - 1 .. pos.y + 2){
    //     if j >= 0 && j < y{
    //         results.push(Position { x: pos.x, y: j, value: pos.value + 1});
    //     }
    // }
    results

}

//fix board w/ bfs or dfs pathing
fn fix_board(board: Board<Tile>) /* -> Board<Tile>*/{

    let mut board_nums: Vec<Vec<i32>> = vec![vec![0; board.x];board.y]; 

    //starting pos
    let mut pointer: Position= Position { x: (0), y: (0)};

    //update board numbers
    let mut tiles: Vec<Position> = vec![];
    tiles.push(pointer);
    while tiles.len() > 0{
        let curr_tile: Position= tiles.pop().expect("err");
        //adjacent tiles of unknown value
        let adjacents: Vec<Position> = get_adjacent(
        curr_tile,
        board.x as i32,
        board.y as i32,
        board.tiles.clone());

        for pos in adjacents.iter(){
            
            //adjacent tile has lower value
            if board_nums[curr_tile.x as usize][curr_tile.y as usize]
                > board_nums[pos.x as usize][pos.y as usize]{
                    tiles.push(*pos);
            }
        }

    }

    //debug
    println!("{:?}", board_nums);


    //after getting distances start fixing map by removing walls #todo


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

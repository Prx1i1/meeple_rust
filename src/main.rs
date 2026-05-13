use std::collections::VecDeque;

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

struct Meep{
    id: i32,
    spec_move: Box< dyn Fn() -> Vec<Tile>>
}

fn red_spec_move(board: Board<Tile>, pos: Position) -> Vec<Position>{
    let x = pos.x;
    let y = pos.y;

    let mut tiles: Vec<Position> = [].to_vec(); 

    if x - 1 >= 0{
        tiles.push(Position {x: x - 1, y: pos.y});
    }
    if x + 1 < board.x as i32 {
        tiles.push(Position { x: x + 1, y : pos.y });
    }

    if y - 1 >= 0{
        tiles.push(Position {x: pos.x, y: y - 1});
    }
    if y + 1 < board.y as i32 {
        tiles.push(Position { x: pos.x, y : y + 1 });
    }

    tiles

}

/// Prints a Vec<Vec<i32>> as a matrix
fn print_matrix(matrix: &Vec<Vec<i32>>) {
    if matrix.is_empty() {
        println!("(empty matrix)");
        return;
    }

    for (row_index, row) in matrix.iter().enumerate() {
        if row.is_empty() {
            println!("Row {}: (empty)", row_index);
            continue;
        }

        // Print each element in the row separated by spaces
        for (col_index, value) in row.iter().enumerate() {
            if col_index > 0 {
                print!(" ");
            }
            print!("{}", value);
        }
        println!(); // Newline after each row
    }
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
fn get_adjacent(pos: Position, x: i32, y: i32, tiles: &Vec<Vec<Tile>>)
-> Vec<Position>{

    let mut results: Vec<Position> = vec![];

    //in x axis
    for i in [pos.x - 1, pos.x + 1]{
        //basic edge cases
        if i >= 0 && i < x{

            //candidate position and tiles
            let current_tile: Tile = tiles[pos.y as usize][pos.x as usize];
            let checked_tile: Tile = tiles[pos.y as usize][i as usize];
            
            //check walls
            let mut walls_blocked: bool = false;
            if i > pos.x{
                walls_blocked = current_tile.walls[1] || checked_tile.walls[3];
            }
            if i < pos.x{
                walls_blocked = current_tile.walls[3] || checked_tile.walls[1];
            }

            //walls ok
            if !walls_blocked{
                results.push(Position {x: i, y: pos.y});
            }

        }
    }

    for j in [pos.y - 1, pos.y + 1]{
        if j >= 0 && j < y {
            //candidate position and tiles
            let current_tile: Tile = tiles[pos.y as usize][pos.x as usize];
            let checked_tile: Tile = tiles[j as usize][pos.x as usize];
        
                        //check walls
            let mut walls_blocked: bool = false;
            if j > pos.y{
                walls_blocked = current_tile.walls[2] || checked_tile.walls[0];
            }
            if j < pos.y{
                walls_blocked = current_tile.walls[0] || checked_tile.walls[2];
            }

            //walls ok
            if !walls_blocked{
                results.push(Position {x: pos.x, y: j});
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
fn fix_board(board: &mut Board<Tile>){

    let mut board_nums: Vec<Vec<i32>> = vec![vec![-1; board.x];board.y]; 

    //starting pos
    let mut pointer: Position= Position { x: (0), y: (0)};
    board_nums[0][0] = 0;

    //update board numbers
    let mut tiles: VecDeque<_> = VecDeque::from(vec![]);
    tiles.push_back(pointer);
    while tiles.len() > 0{

        //println!("{:?}", tiles);

        let curr_tile: Position= tiles.pop_front().expect("err");
        //adjacent tiles of unknown value
        let adjacents: Vec<Position> = get_adjacent(
        curr_tile,
        board.x as i32,
        board.y as i32,
        &board.tiles);

        for pos in adjacents.iter(){
            
            //adjacent tile has lower value
            let curr_value = &board_nums[curr_tile.y as usize][curr_tile.x as usize];
            let adjacent_value = &board_nums[pos.y as usize][pos.x as usize];

            if *adjacent_value == -1 {
                    tiles.push_back(*pos);
                    board_nums[pos.y as usize][pos.x as usize] = board_nums[curr_tile.y as usize][curr_tile.x as usize] + 1
            }
        }

    }

    //debug
    //println!("{:#?}", board_nums);
    print_matrix(&mut board_nums);


    //fixing: remove wall to lowest != -1
    for (y, row) in board_nums.clone().iter().enumerate(){
        for (x, elem) in row.iter().enumerate(){
            //if element is -1
            if *elem == -1{
                //adjacent min that is more than -1

                let mut max: i32 = -1;
                let mut adjacentPosition: Position = Position { x: 0, y: 0 };
                let mut direction: i32 = -1;

                //adjacency
                for new_x in [x as i32 - 1, x as i32 + 1]{

                    if new_x as i32 >= 0 && new_x < row.len() as i32{

                        let candidate: i32 = board_nums[y][new_x as usize];

                        if candidate != -1 && candidate > max{
                            max = candidate;
                            adjacentPosition = Position { x: new_x as i32, y: y as i32 };

                            if(new_x - x as i32 > 0){
                                direction = 1;
                            }else{
                                direction = 3;
                            }

                        }

                    }

                }

                for new_y in [y as i32 - 1, y as i32 + 1]{
                    if new_y as i32 >= 0 && new_y < board_nums.len() as i32{
                        let candidate: i32 = board_nums[new_y as usize][x];

                        if candidate != -1 && candidate > max{
                            max = candidate;
                            adjacentPosition = Position { x: x as i32, y: new_y as i32 };
                        
                        
                            if(new_y - y as i32> 0){
                                direction = 2;
                            }else{
                                direction = 0;
                            }
                        }
                    }
                }

                //any adjacent found, changing walls
                if max != -1 && direction != -1{

                    

                    
                    let reverse_direction = ((direction + 2) % 4) as usize;
                    //println!("{}", reverse_direction);

                    board.tiles[y][x].walls[direction as usize] = false;
                    board.tiles[adjacentPosition.y as usize][adjacentPosition.x as usize]
                    .walls[reverse_direction] = false;

                    board_nums[y][x] = max + 1;
                    

                }


            }
        }
    }

    //if any -1 is still present fix again
    if board_nums.iter().flatten().any(|&x| x == -1){
        fix_board(board);
        println!("loop")
    }
    
    print_matrix(&board_nums);

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
        let custom_distr: [i32; 10] = [0,0,0,0, 1,1,1, 2, 2, 3];
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


    let board: Board<Tile> = Board {x: x as usize, y: y as usize, tiles: all_tiles};


    board


}

//return the list of positions possible to move in one move
fn make_moves_list(board: Board<Tile>, pos: Position, piece: i32) -> Vec<Position>{

    let mut result: Vec<Position> = [].to_vec();

    let mut x: i32 = pos.x;

    //right
    while (x < board.x as i32){
        let currentTile: Tile = board.tiles[pos.y as usize][x as usize];
        x+=1;
        if(x == board.x as i32){
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }
        let nextTile: Tile = board.tiles[pos.y as usize][x as usize];

        //check if wall or border
        if(currentTile.walls[1] == true || nextTile.walls[3] == true){
            //res+ currentTile
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }

    }

    
    //left
    x = pos.x;
    while (x > 0){
        let currentTile: Tile = board.tiles[pos.y as usize][x as usize];
        x-=1;
        if x == 0{
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }
        let nextTile: Tile = board.tiles[pos.y as usize][x as usize];

        //check if wall or border
        if(currentTile.walls[3] == true || nextTile.walls[1] == true || x == 0){
            //res+ currentTile
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }
    }


    let mut y: i32 = pos.y;
    //bottom
    while (y < board.y as i32){
        let currentTile: Tile = board.tiles[y as usize][pos.x as usize];
        y+=1;
        if y == board.y as i32{
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }
        let nextTile: Tile = board.tiles[y as usize][pos.x as usize];

        //check if wall or border
        if(currentTile.walls[2] == true || nextTile.walls[0] == true){
            //res+ currentTile
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }

    }

    
    //top
    y = pos.y;
    while (y > 0){
        let currentTile: Tile = board.tiles[y as usize][pos.x as usize];
        y-=1;
        if y == 0{
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }
        let nextTile: Tile = board.tiles[y as usize][pos.x as usize];

        //check if wall or border
        if(currentTile.walls[0] == true || nextTile.walls[2] == true){
            //res+ currentTile
            result.push(Position { x: currentTile.x as i32, y: currentTile.y as i32 });
            break;
        }
    }


    //additional tiles for skil;

    result


}

fn main() {

    //making game board
    let mut board: Board<Tile> = generateBoard(12, 6, 7);
    //println!("{:?}", board);

    let mut fixed_board = fix_board(&mut board);


    println!("{:?}", make_moves_list(board, Position {x: 3, y:3}, 0))
}

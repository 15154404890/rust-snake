use std::collections::LinkedList;
use ncurses::*;
fn main() {
    initscr();
    noecho();
    cbreak();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let mut snake = Snake {
        body: LinkedList::from_iter(vec![(10, 10), (10, 9), (10, 8)]),
        direction: Direction::Right,
    };
    let mut food = vec![(13, 13)];
    let mut d =Food {
        body: LinkedList::from_iter(vec![(13, 13), (16, 8), (5, 20), (9, 30), (11, 40)]),
        direction: Direction::Up,
    };

    loop {
        clear();
        draw_snake(&snake);
        draw_food(&d);
        handle_input(&mut snake);
        snake.move_forward();
        snake.eat_food(&mut food);
        refresh();
        napms(10);
    }
    
}
#[derive(Clone,Copy,PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
    
}
struct Snake{
    body: LinkedList<(i32, i32)>,
    direction: Direction,
}
struct Food{
    body: LinkedList<(i32, i32)>,
    direction: Direction,
}
fn draw_food(food: &Food) {
    for &(x, y) in &food.body {
        mvaddch(x, y, '*' as u32);
    }
}
fn draw_snake(snake: &Snake){
    for &(x,y)in &snake.body{
        mvaddch(x, y, 'g' as u32);
    }
}
fn handle_input(snake: &mut Snake){
    let input =getch();
    match  input{
        97=>{
            snake.direction=Direction::Left
        },
        100=>{
            snake.direction=Direction::Right
        },
        119=>{
            snake.direction=Direction::Up
        },
        115=>{
            snake.direction=Direction::Down
        },
        _=>(),  
    }
}

impl Snake{
    fn move_forward(&mut self){
        let (mut x,mut y)=*self.body.front().unwrap();
        match  self.direction {
            Direction::Left =>y -=1,
            Direction::Right =>y +=1,
            Direction::Up => x-=1,
            Direction::Down =>x+=1,
        }
        self.body.push_front((x,y));
        self.body.pop_back();
    }
}
impl Snake {
    fn eat_food(&mut self, food: &mut Vec<(i32, i32)>) {
        for (i, v) in food.clone().iter().enumerate() {
            let head = *self.body.front().unwrap();
            if head.0 == v.0 && head.1 == v.1 {
                self.body.push_back(head);
                food.remove(i);
                break;
            }
        }
    } 
}

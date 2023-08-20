mod pathfinding {
    use std::usize;

    pub fn dijkstra(grid: &Grid, start: Point, end: Point) -> Option<Vec<Point>> {
        println!(
            "Starting Dijkstra with start: {:?} and end: {:?}",
            start, end
        );

        let mut open_set = BinaryHeap::new();
        open_set.push(State {
            cost: 0,
            position: start,
            heuristic: 0,
        });

        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(start, 0);

        while let Some(current) = open_set.pop() {
            if current.position == end {
                println!("End found!");
                let mut path = vec![end];
                while let Some(next) = came_from.get(&path[path.len() - 1]) {
                    path.push(*next);
                }
                path.reverse();
                return Some(path);
            }

            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = current.position;
                let next = ((x as isize + dx) as usize, (y as isize + dy) as usize);

                if next.0 >= grid.len() || next.1 >= grid[0].len() {
                    println!("Skipped: Out of bounds");
                    continue;
                }

                if grid[next.0][next.1] == Cell::Barrier {
                    println!("Skipped: Cell is a barrier");
                    continue;
                }

                let tentative_g_score = g_score.get(&current.position).unwrap() + 1;
                if tentative_g_score < *g_score.get(&next).unwrap_or(&usize::MAX) {
                    println!(
                        "Cell ({:?}) added to open set with score {}",
                        next, tentative_g_score
                    );
                    came_from.insert(next, current.position);
                    g_score.insert(next, tentative_g_score);
                    open_set.push(State {
                        cost: tentative_g_score,
                        position: next,
                        heuristic: 0, // No heuristic for Dijkstra
                    });
                }
            }
        }

        println!("No path found");
        None
    }
    use serde::Deserialize;

    pub type Grid = Vec<Vec<Cell>>;
    pub type Point = (usize, usize);

    #[derive(Clone, PartialEq, Deserialize)]
    pub enum Cell {
        Empty,
        Barrier,
        Start,
        End,
    }

    use std::cmp::Ordering;
    use std::collections::BinaryHeap;
    use std::collections::HashMap;

    fn heuristic(a: Point, b: Point) -> usize {
        (b.0 as isize - a.0 as isize).abs() as usize + (b.1 as isize - a.1 as isize).abs() as usize
    }

    pub fn a_star(grid: &Grid, start: Point, end: Point) -> Option<Vec<Point>> {
        println!("Starting A* with start: {:?} and end: {:?}", start, end);

        let mut open_set = BinaryHeap::new();
        open_set.push(State {
            cost: 0,
            position: start,
            heuristic: heuristic(start, end),
        });

        let mut came_from = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(start, 0);

        while let Some(current) = open_set.pop() {
            if current.position == end {
                println!("End found!");
                let mut path = vec![end];
                while let Some(next) = came_from.get(&path[path.len() - 1]) {
                    path.push(*next);
                }
                path.reverse();
                return Some(path);
            }

            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = current.position;
                let next = ((x as isize + dx) as usize, (y as isize + dy) as usize);

                if next.0 >= grid.len() || next.1 >= grid[0].len() {
                    println!("Skipped: Out of bounds");
                    continue;
                }

                if grid[next.0][next.1] == Cell::Barrier {
                    println!("Skipped: Cell is a barrier");
                    continue;
                }

                let tentative_g_score = g_score.get(&current.position).unwrap() + 1;
                if tentative_g_score < *g_score.get(&next).unwrap_or(&usize::MAX) {
                    println!(
                        "Cell ({:?}) added to open set with score {}",
                        next, tentative_g_score
                    );
                    came_from.insert(next, current.position);
                    g_score.insert(next, tentative_g_score);
                    open_set.push(State {
                        cost: tentative_g_score,
                        position: next,
                        heuristic: heuristic(next, end),
                    });
                }
            }
        }

        println!("No path found");
        None
    }

    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        position: Point,
        heuristic: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| other.heuristic.cmp(&self.heuristic))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    use std::collections::VecDeque;

    pub fn bfs(grid: &Grid, start: Point, end: Point) -> Option<Vec<Point>> {
        println!("Starting BFS with start: {:?} and end: {:?}", start, end);

        let mut queue = VecDeque::new();
        queue.push_back(start);

        let mut came_from = HashMap::new();
        let mut visited = HashMap::new();
        visited.insert(start, true);

        while let Some(current) = queue.pop_front() {
            if current == end {
                println!("End found!");
                let mut path = vec![end];
                while let Some(next) = came_from.get(&path[path.len() - 1]) {
                    path.push(*next);
                }
                path.reverse();
                return Some(path);
            }

            for &(dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = current;
                let next = ((x as isize + dx) as usize, (y as isize + dy) as usize);

                if next.0 >= grid.len() || next.1 >= grid[0].len() {
                    println!("Skipped: Out of bounds");
                    continue;
                }

                if grid[next.0][next.1] == Cell::Barrier {
                    println!("Skipped: Cell is a barrier");
                    continue;
                }

                if !visited.contains_key(&next) {
                    came_from.insert(next, current);
                    visited.insert(next, true);
                    queue.push_back(next);
                }
            }
        }

        println!("No path found");
        None
    }
}
use actix_web::{post, web, App, HttpResponse, HttpServer};
use pathfinding::{a_star, bfs, dijkstra, Grid, Point};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PathfindingRequest {
    grid: Grid,
    start: Point,
    end: Point,
}

pub async fn a_star_route(info: web::Json<PathfindingRequest>) -> HttpResponse {
    match a_star(&info.grid, info.start, info.end) {
        Some(path) => HttpResponse::Ok().json(path),
        None => HttpResponse::NotFound().body("Path not found"),
    }
}

pub async fn dijkstra_route(info: web::Json<PathfindingRequest>) -> HttpResponse {
    match dijkstra(&info.grid, info.start, info.end) {
        Some(path) => HttpResponse::Ok().json(path),
        None => HttpResponse::NotFound().finish(),
    }
}

pub async fn bfs_route(info: web::Json<PathfindingRequest>) -> HttpResponse {
    match bfs(&info.grid, info.start, info.end) {
        Some(path) => HttpResponse::Ok().json(path),
        None => HttpResponse::NotFound().finish(),
    }
}

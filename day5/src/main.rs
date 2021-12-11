#![feature(array_chunks)]
#![feature(int_abs_diff)]
const WIDTH: usize = 1000;
type Point = (u32, u32);
struct Grid {
    g: Vec<u32>,
}

impl Grid {
    fn paint_line(&mut self, a: Point, b: Point) {
        let points = Self::gen_points_on_line(a, b);

        for (x, y) in points {
            let i = y as usize * WIDTH + x as usize;
            self.g[i] += 1;
        }
    }

    fn gen_points_on_line(a: Point, b: Point) -> impl Iterator<Item = Point> {
        let ((lx, ly), (rx, ry)) = if a.0 < b.0 { (a, b) } else { (b, a) };
        let xdiff = lx.abs_diff(rx);
        let ydiff = ly.abs_diff(ry);
        let steps = xdiff.max(ydiff) + 1;

        let y_inc = ly < ry;

        let x_points = (lx..=rx).cycle();
        let y_points: Box<dyn Iterator<Item = u32>> = if y_inc {
            Box::new((ly..=ry).cycle())
        } else {
            Box::new((ry..=ly).rev().cycle())
        };

        x_points.zip(y_points).take(steps as usize)
    }
}

fn main() {
    let mut grid = Grid {
        g: vec![0u32; WIDTH * WIDTH],
    };
    let (lines, diagonals): (Vec<_>, _) = include_str!("../input.txt")
        .lines()
        .map(|s| s.split_once(" -> ").unwrap())
        .map(|(lxy, rxy)| (parse_coord(lxy), parse_coord(rxy)))
        .partition(|((x1, y1), (x2, y2))| x1 == x2 || y1 == y2);

    for (a, b) in lines {
        grid.paint_line(a, b);
    }

    let count = grid.g.iter().filter(|&&v| v >= 2).count();
    println!("Day 5.1: {}", count);

    for (a, b) in diagonals {
        grid.paint_line(a, b);
    }

    let count = grid.g.iter().filter(|&&v| v >= 2).count();
    println!("Day 5.2: {}", count);
}

fn parse_coord(s: &str) -> (u32, u32) {
    let (x, y) = s.split_once(',').unwrap();
    (x.parse().unwrap(), y.parse().unwrap())
}

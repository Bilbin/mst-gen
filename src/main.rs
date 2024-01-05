use ggez::conf::*;
use ggez::event::{self, MouseButton};
use ggez::glam::*;
use ggez::graphics::{self, Color, Mesh};
use ggez::mint::Point2;
use ggez::{Context, GameError, GameResult};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

struct Circle {
    mesh: Mesh,
    pos: Vec2,
}

impl Circle {
    fn new(pos: Vec2, ctx: &Context) -> Circle {
        let mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2::new(pos.x, pos.y),
            15.0,
            2.0,
            Color::from_rgb(0xff, 0x66, 0x00),
        )
        .unwrap();

        Circle { mesh, pos }
    }
}

struct MainState {
    circles: Vec<Circle>,
    lines: Vec<Mesh>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            circles: Vec::new(),
            lines: Vec::new(),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), ggez::GameError> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);
        for circle in &self.circles {
            canvas.draw(&circle.mesh, Vec2::new(0.0, 0.0));
        }
        for line in &self.lines {
            canvas.draw(line, Vec2::new(0.0, 0.0))
        }

        canvas.finish(ctx)?;
        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        ctx: &mut Context,
        _button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), GameError> {
        match _button {
            MouseButton::Left => {
                self.circles.push(Circle::new(Vec2::new(x, y), ctx));
                self.lines = create_mst(&self.circles, ctx);
            }
            _ => {}
        }
        Ok(())
    }
}

struct Edge {
    v_ind1: usize,
    v_ind2: usize,
    cost: f32,
}

impl Edge {
    fn new(v_ind1: usize, v_ind2: usize, circles: &Vec<Circle>) -> Edge {
        Edge {
            v_ind1,
            v_ind2,
            cost: circles[v_ind1].pos.distance(circles[v_ind2].pos),
        }
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.partial_cmp(&self.cost).unwrap()
    }
}

impl PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Edge {}

fn create_mst(circles: &Vec<Circle>, ctx: &Context) -> Vec<Mesh> {
    let mut lines = Vec::new();
    let mut visited = vec![false; circles.len()];
    let mut pq = BinaryHeap::new();
    pq.push(Edge::new(0, 0, circles));

    while !pq.is_empty() {
        let edge = pq.pop().unwrap();
        let origin = edge.v_ind1;
        let current = edge.v_ind2;
        if visited[current] {
            continue;
        }

        visited[current] = true;
        let line = graphics::Mesh::new_line(
            ctx,
            &[
                Point2 {
                    x: circles[origin].pos.x,
                    y: circles[origin].pos.y,
                },
                Point2 {
                    x: circles[current].pos.x,
                    y: circles[current].pos.y,
                },
            ],
            2.0,
            Color::from_rgb(0xff, 0x66, 0x00),
        )
        .unwrap();
        lines.push(line);

        for ind in 0..circles.len() {
            if !visited[ind] && ind != current {
                pq.push(Edge::new(current, ind, circles));
            }
        }
    }

    lines
}

pub fn main() -> GameResult {
    let mut cb = ggez::ContextBuilder::new("MST Gen", "Bilbin");
    let window = WindowSetup {
        title: "MST Gen".to_owned(),
        samples: NumSamples::One,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };
    cb = cb.window_setup(window);
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;

    event::run(ctx, event_loop, state)
}

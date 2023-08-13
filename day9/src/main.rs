use std::collections::VecDeque;

use egui::{Color32, Sense, Stroke};
use nom::{combinator::all_consuming, Finish};
use parse::{Direction, GridPos, Instruction};

mod parse;

use eframe::{egui, epaint::ahash::HashSet};

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        vsync: false,
        ..Default::default()
    };
    eframe::run_native(
        "AoC 2022 — Day 9",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    );
}

struct MyApp {
    instructions: VecDeque<Instruction>,
    knots: [GridPos; 10],
    tail_visited: HashSet<GridPos>,
}

impl MyApp {
    fn new() -> Self {
        let instructions = include_str!("input.txt")
            .lines()
            .map(|l| all_consuming(Instruction::parse)(l).finish().unwrap().1)
            .collect();

        Self {
            instructions,
            knots: [GridPos { x: 0, y: 0 }; 10],
            tail_visited: Default::default(),
        }
    }

    fn update_state(&mut self) {
        let instruction = match self.instructions.front_mut() {
            Some(instruction) => instruction,
            None => return,
        };
        self.knots[0] += instruction.dir.delta();

        for i in 1..self.knots.len() {
            let diff = self.knots[i - 1] - self.knots[i];
            let (dx, dy) = match (diff.x, diff.y) {
                // overlapping
                (0, 0) => (0, 0),
                // touching up/left/down/right
                (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
                // touching diagonally
                (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
                // need to move up/left/down/right
                (0, 2) => (0, 1),
                (0, -2) => (0, -1),
                (2, 0) => (1, 0),
                (-2, 0) => (-1, 0),
                // need to move to the right diagonally
                (2, 1) => (1, 1),
                (2, -1) => (1, -1),
                // need to move to the left diagonally
                (-2, 1) => (-1, 1),
                (-2, -1) => (-1, -1),
                // need to move up/down diagonally
                (1, 2) => (1, 1),
                (-1, 2) => (-1, 1),
                (1, -2) => (1, -1),
                (-1, -2) => (-1, -1),
                // 🆕 need to move diagonally
                (-2, -2) => (-1, -1),
                (-2, 2) => (-1, 1),
                (2, -2) => (1, -1),
                (2, 2) => (1, 1),
                _ => panic!("unhandled case: tail - head = {diff:?}"),
            };
            self.knots[i].x += dx;
            self.knots[i].y += dy;

            if i == self.knots.len() - 1 {
                self.tail_visited.insert(self.knots[i]);
            }
        }

        instruction.dist -= 1;
        if instruction.dist == 0 {
            self.instructions.pop_front();
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_state();

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.label(format!("{} instructions left", self.instructions.len()));
            ui.label(format!("{} places visited", self.tail_visited.len()));
            egui::ScrollArea::new([false, true]).show(ui, |ui| {
                for ins in &self.instructions {
                    let arrow = match ins.dir {
                        Direction::Up => "⬆",
                        Direction::Down => "⬇",
                        Direction::Right => "➡",
                        Direction::Left => "⬅",
                    };
                    ui.label(arrow.repeat(ins.dist as _));
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            const CANVAS_WIDTH: f32 = 900.0;
            const CANVAS_HEIGHT: f32 = 700.0;
            const SIDE: f32 = 5.0;

            let painter_size = egui::vec2(CANVAS_WIDTH, CANVAS_HEIGHT);
            let (res, painter) = ui.allocate_painter(painter_size, Sense::hover());
            let center = res.rect.center().to_vec2();

            let to_panel_pos = |pos: GridPos| {
                (egui::vec2(pos.x as f32 * SIDE, pos.y as f32 * SIDE) + center).to_pos2()
            };

            let half_width = (CANVAS_WIDTH / SIDE).floor() as i32;
            let half_height = (CANVAS_HEIGHT / SIDE).floor() as i32;

            for x in -half_width..half_width {
                for y in -half_height..half_height {
                    let dot = GridPos { x, y };
                    if !self.tail_visited.contains(&dot) {
                        continue;
                    }
                    let color = Color32::DARK_RED;

                    let dot_pos = to_panel_pos(dot);
                    painter.circle_stroke(dot_pos, 1.0, Stroke::new(2.0, color));
                }
            }

            let num_knots = self.knots.len();

            for (i, knot_pos) in self.knots.iter().copied().enumerate() {
                let knot_pos = to_panel_pos(knot_pos);
                if i > 0 {
                    // paint an arrow from the previous knot to this one
                    let prev_pos = to_panel_pos(self.knots[i - 1]);
                    painter.arrow(
                        prev_pos,
                        knot_pos - prev_pos,
                        Stroke::new(1.0, Color32::YELLOW),
                    )
                }
            }

            for (i, knot_pos) in self.knots.iter().copied().enumerate() {
                let knot_pos = to_panel_pos(knot_pos);
                painter.circle_filled(
                    knot_pos,
                    2.0,
                    Color32::from_rgb(
                        20,
                        60 + ((255.0 - 60.0) * (num_knots as f32 - i as f32) / num_knots as f32)
                            as u8,
                        20,
                    ),
                );
            }
        });

        ctx.request_repaint();
    }
}
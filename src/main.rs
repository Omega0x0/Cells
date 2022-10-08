use cells::{world::*, cell::{Gen, Cell}, limit};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const WIDTH_SCREEN: u32 = 650;
const HEIGHT_SCREEN: u32 = 650;

fn main() {
    nannou::app(create_window).update(update).run();
}

struct Game {
    world: World,
    egui: Egui
}

fn create_window(app: &App) -> Game {
    let window_id = app
        .new_window()
        .title("Cells")
        .view(view)
        .raw_event(raw_window_event)
        .size(WIDTH_SCREEN, HEIGHT_SCREEN)
        .resizable(false)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Game {
        world: World::new(),
        egui 
    }
}

fn raw_window_event(_app: &App, game: &mut Game, event: &nannou::winit::event::WindowEvent) {
    match event {
        nannou::winit::event::WindowEvent::CloseRequested => {
            std::process::exit(0x0000);
        }
        _ => {}
    }
    game.egui.handle_raw_event(event);
}

fn update(_app: &App, game: &mut Game, update: Update) {
    let mut new_buf_cells: Vec<Cell> = vec![];
    for cell in game.world.cells.1.iter_mut() {
        let grid = &mut game.world.cells.0;
    
        match cell.genome[cell.step] {
            Gen::SetDirection(d) => cell.to_rotate(d),
            Gen::Reproduce => {
                let (left, right, up, down) = (
                    limit(0, 49, cell.position.0 as i64 - 1) as usize,
                    limit(0, 49, cell.position.0 as i64 + 1) as usize,
                    limit(0, 49, cell.position.1 as i64 + 1) as usize,
                    limit(0, 49, cell.position.1 as i64 - 1) as usize,
                );

                match cell.direction {
                    0 => {
                        if grid[right][cell.position.1] == 0 {
                            cell.step = 0;
                            cell.time_life = 0;
                            let mut new_cell = cell.clone(); 
                            new_cell.position.0 = right;

                            grid[right][new_cell.position.1] = 1;
                            new_buf_cells.push(new_cell);
                        }
                    },
                    1 => {
                        if grid[cell.position.0][up] == 0 {
                            cell.step = 0;
                            cell.time_life = 0;
                            let mut new_cell = cell.clone();
                            new_cell.position.1 = up;

                            grid[cell.position.0][up] = 1;
                            new_buf_cells.push(new_cell);
                        }
                    },
                    2 => {
                        if grid[left][cell.position.1] == 0 {
                            cell.step = 0;
                            cell.time_life = 0;
                            let mut new_cell = cell.clone();
                            new_cell.position.0 = left;

                            grid[left][new_cell.position.1] = 1;
                            new_buf_cells.push(new_cell);
                        }
                    },
                    3 => {
                        if grid[cell.position.0][down] == 0 {
                            cell.step = 0;
                            cell.time_life = 0;
                            let mut new_cell = cell.clone();
                            new_cell.position.1 = down;

                            grid[cell.position.0][down] = 1;
                            new_buf_cells.push(new_cell);
                        }
                    },
                    _ => {}
                }
            },
        }

        cell.time_life += 1;
        cell.step += 1;
        if cell.step >= cell.genome.len() { cell.step = 0; }
    }
    game.world.cells.1.append(&mut new_buf_cells);

    for i in 0..game.world.cells.1.len() {
        if i < game.world.cells.1.len() && 
        game.world.cells.1[i].time_life > game.world.cells.1[i].max_time_life {
            let (x, y) = (
                game.world.cells.1[i].position.0, 
                game.world.cells.1[i].position.1
            );

            game.world.cells.0[x][y] = 0;
            game.world.cells.1.remove(i);
        }
    }


    let egui = &mut game.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("Info").show(&ctx, |ui| {
        ui.label(format!("Cells: {}", game.world.cells.1.len()));
    });
}

fn view(app: &App, game: &Game, frame: Frame) {
    let size_cell = (
        WIDTH_SCREEN as f32 / SIZE_MAP.0 as f32, 
        HEIGHT_SCREEN as f32 / SIZE_MAP.1 as f32
    );
    let draw = app.draw();
    draw.background().rgb(15./255., 15./255., 25./255.);

    for cell in game.world.cells.1.iter() {
        draw.rect()
            .w(size_cell.0)
            .h(size_cell.1)
            .x(
                cell.position.0 as f32 * 
                size_cell.0 - 
                WIDTH_SCREEN as f32 / 2.0 + 
                size_cell.0 / 2.0
            )
            .y(
                cell.position.1 as f32 *
                size_cell.1 -
                HEIGHT_SCREEN as f32 / 2.0 +
                size_cell.1 / 2.0
            )
            .rgb(cell.color.r, cell.color.g, cell.color.b);
    }

    draw.to_frame(app, &frame).unwrap();
    game.egui.draw_to_frame(&frame).unwrap();
}
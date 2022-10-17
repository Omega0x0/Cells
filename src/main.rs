use cells::{world::*, cell::{Gen, Cell}, limit, info::Info, filters::Filters};
use nannou::prelude::*;
use nannou_egui::{self, egui, Egui};

const WIDTH_SCREEN: u32 = 1280;
const HEIGHT_SCREEN: u32 = 720;

fn main() {
    nannou::app(create_window).update(update).run();
}

struct Game {
    world: World,
    info: Info,
    filters: Filters,
    egui: Egui
}

fn create_window(app: &App) -> Game {
    let window_id = app
        .new_window()
        .title("Cells")
        .view(view)
        .raw_event(raw_window_event)
        .size(WIDTH_SCREEN, HEIGHT_SCREEN)
        //.resizable(false)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Game {
        world: World::new(),
        info: Info::new(),
        filters: Filters::new(),
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
    let egui = &mut game.egui;
    egui.set_elapsed_time(update.since_start);

    let ctx = egui.begin_frame();

    egui::Window::new("World").show(&ctx, |ui| {
        ui.label(format!("Cells: {}", game.world.cells.1.len()));
        ui.label(format!("FPS: {:.1}", 1000.0 / update.since_last.as_millis() as f32));

        ui.label("Nutrient medium:");
        ui.add(egui::Slider::new(&mut game.world.nutrient_medium, 0.0..=10.0));
    });

    egui::Window::new("Info").show(&ctx, |ui| {
        ui.label(format!("Average max lifetime: {:.1}", game.info.ave_max_lifetime));
        ui.label(format!("Average min mass: {:.1}", game.info.ave_min_mass));
        ui.label(format!("Average max mass: {:.1}", game.info.ave_max_mass));
        ui.label(format!("Average min mass division: {:.1}", game.info.ave_min_mass_division));
    });

    egui::Window::new("Filters").show(&ctx, |ui| {
        ui.checkbox(&mut game.filters.max_lifetime, "Max time of time.");
        ui.checkbox(&mut game.filters.max_mass, "Max mass.");
        ui.checkbox(&mut game.filters.min_mass, "Min mass.");
        ui.checkbox(&mut game.filters.min_mass_division, "Min mass of division.");
    });



    let mut new_buf_cells: Vec<Cell> = vec![];
    for i in 0..game.world.cells.1.len() {
        if i >= game.world.cells.1.len() { break; }
        
        let mut cell = &mut game.world.cells.1[i];
        let grid = &mut game.world.cells.0;
    
        match cell.genome[cell.step] {
            Gen::SetDirection(d) => cell.to_rotate(d),
            Gen::Reproduce => {
                let mut new_cell = cell.clone();
                let mut is_rprdc = false;
                let (left, right, up, down) = (
                    limit(0, (SIZE_MAP.0 - 1) as i64, cell.position.0 as i64 - 1) as usize,
                    limit(0, (SIZE_MAP.0 - 1) as i64, cell.position.0 as i64 + 1) as usize,
                    limit(0, (SIZE_MAP.1 - 1) as i64, cell.position.1 as i64 + 1) as usize,
                    limit(0, (SIZE_MAP.1 - 1) as i64, cell.position.1 as i64 - 1) as usize,
                );

                match cell.direction {
                    0 => {
                        if grid[right][cell.position.1] == 0 {
                            is_rprdc = true;
                            new_cell.position.0 = right;
                        }
                    },
                    1 => {
                        if grid[cell.position.0][up] == 0 {
                            is_rprdc = true;
                            new_cell.position.1 = up;
                        }
                    },
                    2 => {
                        if grid[left][cell.position.1] == 0 {
                            is_rprdc = true;
                            new_cell.position.0 = left;
                        }
                    },
                    3 => {
                        if grid[cell.position.0][down] == 0 {
                            is_rprdc = true;
                            new_cell.position.1 = down;
                        }
                    },
                    _ => {}
                }

                if is_rprdc && cell.mass > cell.min_mass_division {
                    new_cell.step = 0;
                    grid[new_cell.position.0][new_cell.position.1] = 1;
                    (cell.time_life, new_cell.time_life) = (0, 0);
                    (cell.mass, new_cell.mass) = (cell.mass / 2.0, new_cell.mass / 2.0);
                    new_cell.mutate();
                    new_buf_cells.push(new_cell);
                }
            },
        }

        cell.time_life += 1;
        cell.step += 1;
        cell.mass += game.world.nutrient_medium * 
        (1.0 - cell.position.1 as f32 / SIZE_MAP.1 as f32) - cell.consume();

        if cell.step >= cell.genome.len() { cell.step = 0; }
        if cell.mass > cell.max_mass { cell.mass = cell.max_mass; }

        game.info.ave_max_lifetime += cell.max_time_life as f32;
        game.info.ave_min_mass += cell.min_mass as f32;
        game.info.ave_max_mass += cell.max_mass as f32;
        game.info.ave_min_mass_division += cell.min_mass_division as f32;

        if i < game.world.cells.1.len() && 
        (game.world.cells.1[i].time_life > game.world.cells.1[i].max_time_life ||
        game.world.cells.1[i].mass < game.world.cells.1[i].min_mass) {
            let (x, y) = (
                game.world.cells.1[i].position.0, 
                game.world.cells.1[i].position.1
            );

            game.world.cells.0[x][y] = 0;
            game.world.cells.1.remove(i);
        }
    }
    game.info.ave_max_lifetime /= game.world.cells.1.len() as f32;
    game.info.ave_min_mass /= game.world.cells.1.len() as f32;
    game.info.ave_max_mass /= game.world.cells.1.len() as f32;
    game.info.ave_min_mass_division /= game.world.cells.1.len() as f32;

    game.world.cells.1.append(&mut new_buf_cells);
}

fn view(app: &App, game: &Game, frame: Frame) {
    let size_cell = (
        WIDTH_SCREEN as f32 / SIZE_MAP.0 as f32, 
        HEIGHT_SCREEN as f32 * (WIDTH_SCREEN as f32 / HEIGHT_SCREEN as f32) / SIZE_MAP.1 as f32
    );
    let draw = app.draw();
    draw.background().rgb(15./255., 15./255., 25./255.);

    for cell in game.world.cells.1.iter() {
        let rect = draw.rect()
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
            );

        if game.filters.max_lifetime {
            rect.rgb(
                cell.max_time_life as f32 / game.info.ave_max_lifetime - 0.8, 
                cell.max_time_life as f32 / game.info.ave_max_lifetime - 0.8, 
                cell.max_time_life as f32 / game.info.ave_max_lifetime - 0.8
            );
            continue;
        } else if game.filters.max_mass {
            rect.rgb(
                cell.max_mass as f32 / game.info.ave_max_mass - 0.8, 
                cell.max_mass as f32 / game.info.ave_max_mass - 0.8, 
                cell.max_mass as f32 / game.info.ave_max_mass - 0.8
            );
            continue;
        } else if game.filters.min_mass {
            rect.rgb(
                cell.min_mass as f32 / game.info.ave_min_mass - 0.8, 
                cell.min_mass as f32 / game.info.ave_min_mass - 0.8, 
                cell.min_mass as f32 / game.info.ave_min_mass - 0.8
            );
            continue;
        } else if game.filters.min_mass_division {
            rect.rgb(
                cell.min_mass_division as f32 / game.info.ave_min_mass_division - 0.8, 
                cell.min_mass_division as f32 / game.info.ave_min_mass_division - 0.8, 
                cell.min_mass_division as f32 / game.info.ave_min_mass_division - 0.8
            );
            continue;
        }

        rect.rgb(cell.color.r, cell.color.g, cell.color.b);
    }

    draw.to_frame(app, &frame).unwrap();
    game.egui.draw_to_frame(&frame).unwrap();
}
use cells::{
    cell::{Cell, Gen},
    filters::Filters,
    info::Info,
    limit,
    world::*, settings::Settings,
};
use nannou::{prelude::*, event::ElementState};
use nannou::winit::event::WindowEvent;
use nannou_egui::{
    self,
    egui,
    Egui,
};

const WIDTH_SCREEN: u32 = 1280;
const HEIGHT_SCREEN: u32 = 720;

fn main() {
    nannou::app(create_window).update(update).run();
}

struct Game {
    world: World,
    info: Info,
    filters: Filters,
    settings: Settings,
    egui: Egui,
}

fn create_window(app: &App) -> Game {
    let window_id = app
        .new_window()
        .title("Cells")
        .view(view)
        .raw_event(raw_window_event)
        .size(WIDTH_SCREEN, HEIGHT_SCREEN)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    Game {
        world: World::new(),
        info: Info::new(),
        filters: Filters::Default,
        settings: Settings::new(),
        egui,
    }
}

fn raw_window_event(_app: &App, game: &mut Game, event: &nannou::winit::event::WindowEvent) {
    match event {
        WindowEvent::CloseRequested => {
            std::process::exit(0x0000);
        }
        WindowEvent::MouseWheel { delta, .. } => {
            game.settings.scale += if let MouseScrollDelta::LineDelta(_, y) = delta { 
                if game.settings.scale <= 0.5 && *y / 5.0 < 0.0 {
                    0.0
                } else { *y / 5.0 }
            } else { 
                0.0 
            };
        }
        WindowEvent::MouseInput { state, ..} => {
            if let ElementState::Pressed = state { 
                game.settings.mouse.pressed = true; 
            } else { 
                game.settings.mouse.pressed = false; 
            }
        }
        WindowEvent::CursorMoved { position, ..} => {
            if game.settings.mouse.pressed {
                let direc = vec2(
                    (position.x - game.settings.mouse.last_pos.0) as f32,
                    (game.settings.mouse.last_pos.1 - position.y) as f32
                );

                game.settings.position += direc;
            }
            game.settings.mouse.last_pos = (position.x, position.y);
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
        ui.label(format!(
            "Cells: {}", game.world.cells.1.len()
        ));

        ui.label(format!(
            "FPS: {:.1}",
            1000.0 / update.since_last.as_millis() as f32
        ));

        ui.label("Nutrient medium:");
        ui.add(egui::Slider::new(
            &mut game.world.nutrient_medium,
            0.0..=10.0,
        ));

        ui.label("Speed:");
        ui.add(egui::Slider::new(
            &mut game.world.speed,
            0..=50,
        ));
    });

    egui::Window::new("Info").show(&ctx, |ui| {
        ui.label(format!(
            "Average max lifetime: {:.1}",
            game.info.ave_max_lifetime
        ));

        ui.label(format!(
            "Average min mass: {:.1}", 
            game.info.ave_min_mass
        ));

        ui.label(format!(
            "Average max mass: {:.1}", 
            game.info.ave_max_mass
        ));

        ui.label(format!(
            "Average min mass division: {:.1}",
            game.info.ave_min_mass_division
        ));

        ui.label(format!(
            "Average damage: {:.3}",
            game.info.ave_damage
        ));

        ui.label(format!(
            "Average resistance: {:.3}",
            game.info.ave_resistance
        ));
    });

    egui::Window::new("Filters").show(&ctx, |ui| {
        ui.radio_value(
            &mut game.filters, 
            Filters::Default, 
            "Default."
        );

        ui.radio_value(
            &mut game.filters, 
            Filters::MaxLifeTime, 
            "Max time of life"
        );

        ui.radio_value(
            &mut game.filters, 
            Filters::MaxMass, 
            "Max mass."
        );

        ui.radio_value(
            &mut game.filters, 
            Filters::MinMass, 
            "Min mass."
        );

        ui.radio_value(
            &mut game.filters,
            Filters::MinMassDivision,
            "Min mass of division.",
        );

        ui.radio_value(
            &mut game.filters,
            Filters::Damage,
            "Damage.",
        );

        ui.radio_value(
            &mut game.filters,
            Filters::Resistance,
            "Resistance.",
        );
    });

    for _ in 0..game.world.speed {

        let mut new_buf_cells: Vec<Cell> = vec![];
        for i in 0..game.world.cells.1.len() {
        if i >= game.world.cells.1.len() {
            break;
        }

        if game.world.cells.1[i].step >= game.world.cells.1[i].genome.len() {
            game.world.cells.1[i].step = 0;
        }
        match game.world.cells.1[i].genome[game.world.cells.1[i].step] {
            Gen::SetDirection(d) => game.world.cells.1[i].to_rotate(d),
            Gen::Reproduce => {
                let mut cell = &mut game.world.cells.1[i];
                let grid = &mut game.world.cells.0;

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
                        if grid[right][cell.position.1] < 0 {
                            is_rprdc = true;
                            new_cell.position.0 = right;
                        }
                    }
                    1 => {
                        if grid[cell.position.0][up] < 0 {
                            is_rprdc = true;
                            new_cell.position.1 = up;
                        }
                    }
                    2 => {
                        if grid[left][cell.position.1] < 0 {
                            is_rprdc = true;
                            new_cell.position.0 = left;
                        }
                    }
                    3 => {
                        if grid[cell.position.0][down] < 0 {
                            is_rprdc = true;
                            new_cell.position.1 = down;
                        }
                    }
                    _ => {}
                }

                if is_rprdc && cell.mass > cell.min_mass_division {
                    new_cell.step = 0;
                    grid[new_cell.position.0][new_cell.position.1] = i as i32;
                    (cell.time_life, new_cell.time_life) = (0, 0);
                    (cell.mass, new_cell.mass) = (cell.mass / 2.0, new_cell.mass / 2.0);
                    new_cell.mutate();
                    new_buf_cells.push(new_cell);
                }
            }
            Gen::Attack => {
                let (left, right, up, down) = (
                    limit(
                        0, (SIZE_MAP.0 - 1) as i64, 
                        game.world.cells.1[i].position.0 as i64 - 1
                    ) as usize,
                    limit(
                        0, (SIZE_MAP.0 - 1) as i64, 
                        game.world.cells.1[i].position.0 as i64 + 1
                    ) as usize,
                    limit(
                        0, (SIZE_MAP.1 - 1) as i64, 
                        game.world.cells.1[i].position.1 as i64 + 1
                    ) as usize,
                    limit(
                        0, (SIZE_MAP.1 - 1) as i64, 
                        game.world.cells.1[i].position.1 as i64 - 1
                    ) as usize,
                );
                
                let grid = game.world.cells.0;
                let mut i_neighbor_cell = grid[right][game.world.cells.1[i].position.1] as usize;
                match game.world.cells.1[i].direction {
                    0 => {
                        if grid[right][game.world.cells.1[i].position.1] > -1 {
                            i_neighbor_cell = grid[right][game.world.cells.1[i].position.1] as usize;
                        }
                    }
                    1 => {
                        if grid[game.world.cells.1[i].position.0][up] > -1 {
                            i_neighbor_cell = grid[game.world.cells.1[i].position.0][up] as usize;
                        }
                    }
                    2 => {
                        if grid[left][game.world.cells.1[i].position.1] > -1 {
                            i_neighbor_cell = grid[left][game.world.cells.1[i].position.1]  as usize;
                        }
                    }
                    3 => {
                        if grid[game.world.cells.1[i].position.0][down] > -1 {
                            i_neighbor_cell = grid[game.world.cells.1[i].position.0][down] as usize;
                        }
                    }
                    _ => {}
                }

                if i_neighbor_cell < game.world.cells.1.len() && 
                game.world.cells.1[i].species != game.world.cells.1[i_neighbor_cell].species {
                    let dmg = game.world.cells.1[i_neighbor_cell].resistance - game.world.cells.1[i].damage;
                    
                    if dmg < 0.0 {
                        game.world.cells.1[i_neighbor_cell].mass += dmg;
                    }

                    game.world.cells.1[i].mass += game.world.cells.1[i].damage - game.world.cells.1[i_neighbor_cell].resistance;
                }
            }
        }

        {
            let mut cell = &mut game.world.cells.1[i];

            cell.time_life += 1;
            cell.step += 1;
            cell.mass += game.world.nutrient_medium
                * (1.0 - cell.position.1 as f32 / SIZE_MAP.1 as f32)
                - cell.consume();

            if cell.step >= cell.genome.len() {
                cell.step = 0;
            }
            if cell.mass > cell.max_mass {
                cell.mass = cell.max_mass;
            }

            game.info.ave_max_lifetime += cell.max_time_life as f32;
            game.info.ave_min_mass += cell.min_mass;
            game.info.ave_max_mass += cell.max_mass;
            game.info.ave_min_mass_division += cell.min_mass_division;
            game.info.ave_damage += cell.damage;
            game.info.ave_resistance += cell.resistance;
        }

        if i < game.world.cells.1.len()
            && (game.world.cells.1[i].time_life > game.world.cells.1[i].max_time_life
                || game.world.cells.1[i].mass < game.world.cells.1[i].min_mass)
        {
            let (x, y) = (
                game.world.cells.1[i].position.0,
                game.world.cells.1[i].position.1,
            );

            game.world.cells.0[x][y] = -1;
            game.world.cells.1.remove(i);
        }
    }
        game.info.ave_max_lifetime /= game.world.cells.1.len() as f32;
        game.info.ave_min_mass /= game.world.cells.1.len() as f32;
        game.info.ave_max_mass /= game.world.cells.1.len() as f32;
        game.info.ave_min_mass_division /= game.world.cells.1.len() as f32;
        game.info.ave_damage /= game.world.cells.1.len() as f32;
        game.info.ave_resistance /= game.world.cells.1.len() as f32;

        game.world.cells.1.append(&mut new_buf_cells);

    }
}

fn view(app: &App, game: &Game, frame: Frame) {
    let settings = &game.settings;
    let size_cell = (5.0 * settings.scale, 5.0 * settings.scale);
    let draw = app.draw();

    draw.background().rgb(15. / 255., 15. / 255., 25. / 255.);
    draw.rect()
        .stroke_color(GRAY)
        .stroke_weight(size_cell.0)
        .color(rgba(0.0, 0.0, 0.0, 0.0))
        .w_h(
            SIZE_MAP.0 as f32 * size_cell.0 + size_cell.0, 
            SIZE_MAP.1 as f32 * size_cell.1 + size_cell.1
        )
        .x_y(
            SIZE_MAP.0 as f32 * size_cell.0 / 2.0 + settings.position.x as f32 - size_cell.0 / 2.0, 
            SIZE_MAP.1 as f32 * size_cell.1 / 2.0 + settings.position.y as f32 - size_cell.1 / 2.0
        );

    for cell in game.world.cells.1.iter() {
        let rect = draw
            .rect()
            .w(size_cell.0)
            .h(size_cell.1)
            .x(cell.position.0 as f32 * size_cell.0 + settings.position.x as f32)
            .y(cell.position.1 as f32 * size_cell.1 + settings.position.y as f32);

        match game.filters {
            Filters::MaxLifeTime => {
                rect.rgb(
                    cell.max_time_life as f32 / game.info.ave_max_lifetime - 0.8,
                    cell.max_time_life as f32 / game.info.ave_max_lifetime - 0.8,
                    cell.max_time_life as f32 / game.info.ave_max_lifetime - 0.8,
                );
            }
            Filters::MaxMass => {
                rect.rgb(
                    cell.max_mass as f32 / game.info.ave_max_mass - 0.8,
                    cell.max_mass as f32 / game.info.ave_max_mass - 0.8,
                    cell.max_mass as f32 / game.info.ave_max_mass - 0.8,
                );
            }
            Filters::MinMass => {
                rect.rgb(
                    cell.min_mass as f32 / game.info.ave_min_mass - 0.8,
                    cell.min_mass as f32 / game.info.ave_min_mass - 0.8,
                    cell.min_mass as f32 / game.info.ave_min_mass - 0.8,
                );
            }
            Filters::MinMassDivision => {
                rect.rgb(
                    cell.min_mass_division as f32 / game.info.ave_min_mass_division - 0.8,
                    cell.min_mass_division as f32 / game.info.ave_min_mass_division - 0.8,
                    cell.min_mass_division as f32 / game.info.ave_min_mass_division - 0.8,
                );
            }
            Filters::Damage => {
                rect.rgb(
                    cell.damage as f32 / game.info.ave_damage - 0.8,
                    cell.damage as f32 / game.info.ave_damage - 0.8,
                    cell.damage as f32 / game.info.ave_damage - 0.8,
                );
            }
            Filters::Resistance => {
                rect.rgb(
                    cell.resistance as f32 / game.info.ave_resistance - 0.8,
                    cell.resistance as f32 / game.info.ave_resistance - 0.8,
                    cell.resistance as f32 / game.info.ave_resistance - 0.8,
                );
            }
            _ => {
                rect.rgb(cell.color.r, cell.color.g, cell.color.b);
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
    game.egui.draw_to_frame(&frame).unwrap();
}
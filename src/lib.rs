use colorsys::{ColorTransform, Hsl, Rgb};
use extism_pdk::*;
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};

const HUE_STEP: f64 = 2.5;

lazy_static! {
    static ref BASE_COLOR: Arc<Mutex<Hsl>> = Arc::new(Mutex::new(Hsl::new(0.0, 100.0, 50.0, None)));
}

#[host_fn]
extern "ExtismHost" {
    fn matricks_debug(msg: &str);
    fn matricks_info(msg: &str);
    fn matricks_warn(msg: &str);
    fn matricks_error(msg: &str);
}

#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    // Setup your plugin here

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
    // Grab values from the config
    let width: usize = config::get("width").unwrap().parse().unwrap();
    let height: usize = config::get("height").unwrap().parse().unwrap();

    // Grab the base color object
    let mut base_color = BASE_COLOR.lock().unwrap();

    // Tweak the base value
    base_color.adjust_hue(HUE_STEP);

    // Create hues for all coordinates
    let mut next_state_hsl: Vec<Vec<Hsl>> = vec![];
    for y in 0..height {
        next_state_hsl.push(vec![]);
        for x in 0..width {
            // Create a new color for this coordinate using the base color
            let mut coordinate_color = base_color.clone();
            coordinate_color.adjust_hue(HUE_STEP * (x + y) as f64);
            next_state_hsl[y].push(coordinate_color);
        }
    }

    // Convert the state to BGRA values
    let mut next_state: Vec<Vec<[u8; 4]>> = vec![];
    for y in 0..height {
        next_state.push(vec![]);
        for x in 0..width {
            let color_as_rgb = Rgb::from(&next_state_hsl[y][x]);
            next_state[y].push([
                color_as_rgb.blue().round() as u8,
                color_as_rgb.green().round() as u8,
                color_as_rgb.red().round() as u8,
                255,
            ])
        }
    }

    Ok(Json(Some(next_state)))
}


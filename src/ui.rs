use crate::algos::Algo;
use crate::draw_context::DrawContext;
use minifb::{Key, KeyRepeat, Window, WindowOptions};

fn get_next_index(window: &Window, index: usize, max_index: usize) -> usize {
    if window.is_key_pressed(Key::Right, KeyRepeat::Yes) {
        std::cmp::min(index + 1, max_index)
    } else if window.is_key_pressed(Key::Left, KeyRepeat::Yes) {
        index.saturating_sub(1)
    } else if window.is_key_pressed(Key::Home, KeyRepeat::No) || index == std::usize::MAX {
        0
    } else if window.is_key_pressed(Key::End, KeyRepeat::No) {
        max_index
    } else {
        index
    }
}

pub fn show<TAlgo>(
    states: &[TAlgo::State],
    actions: &[TAlgo::Action],
    window_size: usize,
    draw_width: f32,
) where
    TAlgo: Algo,
{
    let title = "Geometry Algorithms Visualization";
    let mut window =
        Window::new(title, window_size, window_size, WindowOptions::default()).unwrap();
    let mut index = std::usize::MAX;
    let size = window.get_size();
    let mut dc = DrawContext::new(size, draw_width);

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    window.set_key_repeat_rate(0.01);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let new_index = get_next_index(&window, index, actions.len() * 2);
        if new_index != index {
            index = new_index;
            dc.clear();
            if index % 2 == 0 {
                TAlgo::draw_state(&mut dc, &states[index / 2]);
            } else {
                TAlgo::draw_state(&mut dc, &states[index / 2]);
                TAlgo::draw_action(&mut dc, &actions[index / 2]);
            }
            window
                .update_with_buffer(dc.get_data(), size.0, size.1)
                .unwrap();
        } else {
            window.update();
        }
    }
}

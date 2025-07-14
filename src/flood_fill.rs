use std::collections::VecDeque;
use raylib::prelude::*;
use crate::framebuffers::Framebuffer;


pub fn flood_fill(fb: &mut Framebuffer, start_x: i32, start_y: i32, new_color: Color) {
    if start_x < 0 || start_y < 0 || start_x >= fb.width as i32 || start_y >= fb.height as i32 {
        return;
    }
    
    let start_x = start_x as usize;
    let start_y = start_y as usize;
    let original_color = fb.get_pixel(start_x, start_y);

    if colors_equal(original_color, new_color) {
        return;
    }

    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    
    while let Some((x, y)) = queue.pop_front() {
        if x >= fb.width || y >= fb.height {
            continue;
        }

        if !colors_equal(fb.get_pixel(x, y), original_color) {
            continue;
        }
        fb.set_pixel(x, y, new_color);

        if x > 0 {
            queue.push_back((x - 1, y));
        }
        if x + 1 < fb.width {
            queue.push_back((x + 1, y));
        }
        if y > 0 {
            queue.push_back((x, y - 1));
        }
        if y + 1 < fb.height {
            queue.push_back((x, y + 1));
        }
    }
}

pub fn flood_fill_8_connected(fb: &mut Framebuffer, start_x: i32, start_y: i32, new_color: Color) {
    if start_x < 0 || start_y < 0 || start_x >= fb.width as i32 || start_y >= fb.height as i32 {
        return;
    }
    
    let start_x = start_x as usize;
    let start_y = start_y as usize;
    
    let original_color = fb.get_pixel(start_x, start_y);
    
    if colors_equal(original_color, new_color) {
        return;
    }
    
    let mut queue = VecDeque::new();
    queue.push_back((start_x, start_y));
    
    while let Some((x, y)) = queue.pop_front() {
        if x >= fb.width || y >= fb.height {
            continue;
        }
        
        if !colors_equal(fb.get_pixel(x, y), original_color) {
            continue;
        }
        
        fb.set_pixel(x, y, new_color);
        let directions = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1),           (0, 1),
            (1, -1),  (1, 0),  (1, 1),
        ];
        
        for (dx, dy) in directions.iter() {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            
            if new_x >= 0 && new_y >= 0 {
                queue.push_back((new_x as usize, new_y as usize));
            }
        }
    }
}

fn colors_equal(c1: Color, c2: Color) -> bool {
    c1.r == c2.r && c1.g == c2.g && c1.b == c2.b && c1.a == c2.a
}

pub fn flood_fill_vector(fb: &mut Framebuffer, start_pos: Vector2, new_color: Color) {
    flood_fill(fb, start_pos.x as i32, start_pos.y as i32, new_color);
}

pub fn flood_fill_8_connected_vector(fb: &mut Framebuffer, start_pos: Vector2, new_color: Color) {
    flood_fill_8_connected(fb, start_pos.x as i32, start_pos.y as i32, new_color);
}

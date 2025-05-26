use godot::builtin::Vector2;

pub fn get_click_col_row(click_position: Vector2, square_size: f32) -> (u16, u16) {
    let row = (click_position.y / square_size) as u16;
    let col = (click_position.x / square_size) as u16;
    (col, row)
}

pub fn get_vec_from_col_row(col: u16, row: u16, square_size: f32) -> Vector2 {
    Vector2::new(col as f32 * square_size, row as f32 * square_size)
}

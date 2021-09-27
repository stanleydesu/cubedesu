use draw::*;

fn main() {
    // cube rendering layout (where each letter corresponds to a face):
    //  U
    // LFRB
    //  D
    let sticker_len = 15u32; // length of each sticker in pixels
    let canvas = Canvas::new(13 * sticker_len, 10 * sticker_len);
    // let cube = Cube::default_facelet();
    // let draw_face = |index: u32, row: u32, col: u32| {
    //     for i in row..=(row + 2) {
    //         for j in row..=(col + 2) {
    //             // let color = cubecube[index]
    //             let sticker = Drawing::new()
    //                 .with_shape(Shape::Rectangle {
    //                     width: sticker_len,
    //                     height: sticker_len,
    //                 })
    //                 .with_xy((i * sticker_len) as f32, (j * sticker_len) as f32)
    //                 .with_style(Style::filled(Color::random()));
    //             canvas.display_list.add(sticker);
    //         }
    //     }
    // };

    render::save(&canvas, "images/cube.svg", SvgRenderer::new()).expect("Failed to save");
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {}
}

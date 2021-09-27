use cubedesu::*;
use draw::*;

fn face_to_color(face: Face) -> RGB {
    match face {
        Face::F => RGB {
            r: 0,
            g: 155,
            b: 72,
        },
        Face::U => RGB {
            r: 255,
            g: 255,
            b: 255,
        },
        Face::R => RGB {
            r: 183,
            g: 18,
            b: 52,
        },
        Face::D => RGB {
            r: 255,
            g: 213,
            b: 0,
        },
        Face::B => RGB {
            r: 0,
            g: 70,
            b: 173,
        },
        Face::L => RGB {
            r: 255,
            g: 88,
            b: 0,
        },
    }
}

fn main() {
    // cube rendering layout (where each letter corresponds to a face):
    //  U
    // LFRB
    //  D
    let sticker_len = 15u32; // length of each sticker in pixels
    let mut canvas = Canvas::new(13 * sticker_len, 10 * sticker_len);
    let cube = FaceletModel::default_facelet();
    let mut draw_face = |mut index: usize, row: u32, col: u32| {
        for i in row..=(row + 2) {
            for j in col..=(col + 2) {
                let color = face_to_color(cube[index]);
                index += 1;
                let sticker = Drawing::new()
                    .with_shape(Shape::Rectangle {
                        width: sticker_len,
                        height: sticker_len,
                    })
                    .with_xy((j * sticker_len) as f32, (i * sticker_len) as f32)
                    .with_style(Style::filled(color));
                let border = Drawing::new()
                    .with_shape(Shape::Rectangle {
                        width: sticker_len,
                        height: sticker_len,
                    })
                    .with_xy((j * sticker_len) as f32, (i * sticker_len) as f32)
                    .with_style(Style::stroked(2, Color::black()));
                canvas.display_list.add(sticker);
                canvas.display_list.add(border);
            }
        }
    };

    // draw the faces
    draw_face(0, 0, 3); // U
    draw_face(9, 3, 6); // R
    draw_face(18, 3, 3); // F
    draw_face(27, 6, 3); // D
    draw_face(36, 3, 0); // L
    draw_face(45, 3, 9); // B

    render::save(&canvas, "images/cube.svg", SvgRenderer::new()).expect("Failed to save");
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test() {}
}

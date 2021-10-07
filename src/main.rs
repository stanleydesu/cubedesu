use cubedesu::*;
use macroquad::prelude::*;

#[macroquad::main("Desu")]
async fn main() {
    let mut gcube = GCube::new();
    gcube.apply_movements(&scramble_to_movements("").unwrap());
    const F_LEN: f32 = 1.7; // side length of each facelet
    const F_DEPTH: f32 = 0.01; // thickness/depth of each facelet
                               // returns the size vec3 of a facelet

    // returns facelet dimensions/orientation for a specific face
    let face_to_dimensions = |face| match face {
        Face::U | Face::D => vec3(F_LEN, F_DEPTH, F_LEN),
        Face::L | Face::R => vec3(F_DEPTH, F_LEN, F_LEN),
        Face::F | Face::B => vec3(F_LEN, F_LEN, F_DEPTH),
        _ => vec3(0.0, 0.0, 0.0),
    };

    let face_to_color = |face| match face {
        Face::U => WHITE,
        Face::R => RED,
        Face::L => ORANGE,
        Face::B => BLUE,
        Face::D => YELLOW,
        Face::F => GREEN,
        _ => BLACK,
    };

    let point3_to_vec3 = |p: Point3| vec3(p.x as f32, p.y as f32, p.z as f32);

    loop {
        clear_background(GRAY);
        set_camera(&Camera3D {
            position: vec3(0., 10., 12.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });
        let GCube(stickers) = gcube;
        for sticker in stickers {
            draw_cube(
                point3_to_vec3(sticker.current),
                face_to_dimensions(get_face(sticker.current)),
                None,
                face_to_color(get_face(sticker.initial)),
            );
        }

        set_default_camera();

        next_frame().await
    }
}

// fn get_key{
//     match
//     "I": "R",
//     "K": "R'",
//     "W": "B",
//     "O": "B'",
//     "S": "D",
//     "L": "D'",
//     "D": "L",
//     "E": "L'",
//     "J": "U",
//     "F": "U'",
//     "H": "F",
//     "G": "F'",
//     ";": "y",
//     "A": "y'",
//     "U": "r",
//     "R": "l'",
//     "M": "r'",
//     "V": "l",
//     "T": "x",
//     "Y": "x",
//     "N": "x'",
//     "B": "x'",
//     ".": "M'",
//     "X": "M'",
//     "5": "M",
//     "6": "M",
//     "P": "z",
//     "Q": "z'",
//     "Z": "d",
//     "C": "u'",
//     ",": "u",
//     "/": "d'"
//                })

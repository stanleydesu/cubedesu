use std::str::FromStr;

use cubedesu::*;
use macroquad::{input::KeyCode, prelude::*};

const F_LEN: f32 = 1.5; // side length of each facelet
const F_DEPTH: f32 = 0.01; // thickness/depth of each facelet

#[macroquad::main("cubedesu")]
async fn main() {
    let mut gcube = GCube::new();
    gcube.apply_movements(&scramble_to_movements("").unwrap());

    let mut camera = Camera3D {
        position: vec3(0., 10., 12.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    set_camera(&camera);

    let camera_x_displacement = vec3(0.2, 0., 0.);
    let camera_y_displacement = vec3(0., 0.2, 0.);

    loop {
        clear_background(GRAY);
        if let Some(key) = get_last_key_pressed() {
            if let Some(movement) = key_to_movement(key) {
                gcube.apply_movement(&movement);
            }
        }

        if is_key_down(KeyCode::Left) {
            camera.position -= camera_x_displacement
        }
        if is_key_down(KeyCode::Right) {
            camera.position += camera_x_displacement
        }
        if is_key_down(KeyCode::Up) {
            camera.position += camera_y_displacement
        }
        if is_key_down(KeyCode::Down) {
            camera.position -= camera_y_displacement
        }

        set_camera(&camera);

        let GCube(stickers) = gcube;
        for sticker in stickers {
            draw_cube(
                point3_to_vec3(sticker.current),
                face_to_dimensions(get_face(sticker.current)),
                None,
                face_to_color(get_face(sticker.initial)),
            );
            draw_cube_wires(
                point3_to_vec3(sticker.current),
                face_to_dimensions(get_face(sticker.current)),
                BLACK,
            );
        }

        next_frame().await
    }
}

// returns facelet dimensions/orientation for a specific face
fn face_to_dimensions(face: Face) -> Vec3 {
    match face {
        Face::U | Face::D => vec3(F_LEN, F_DEPTH, F_LEN),
        Face::L | Face::R => vec3(F_DEPTH, F_LEN, F_LEN),
        Face::F | Face::B => vec3(F_LEN, F_LEN, F_DEPTH),
        _ => vec3(0.0, 0.0, 0.0),
    }
}

fn point3_to_vec3(p: Point3) -> Vec3 {
    vec3(p.x as f32, p.y as f32, p.z as f32)
}

fn face_to_color(face: Face) -> Color {
    match face {
        Face::U => WHITE,
        Face::R => RED,
        Face::L => ORANGE,
        Face::B => BLUE,
        Face::D => YELLOW,
        Face::F => GREEN,
        _ => BLACK,
    }
}

fn key_to_movement(key: KeyCode) -> Option<Movement> {
    let movement_str = match key {
        KeyCode::I => "R",
        KeyCode::K => "R'",
        KeyCode::W => "B",
        KeyCode::O => "B'",
        KeyCode::S => "D",
        KeyCode::L => "D'",
        KeyCode::D => "L",
        KeyCode::E => "L'",
        KeyCode::J => "U",
        KeyCode::F => "U'",
        KeyCode::H => "F",
        KeyCode::G => "F'",
        KeyCode::Semicolon => "y",
        KeyCode::A => "y'",
        KeyCode::U => "r",
        KeyCode::R => "l'",
        KeyCode::M => "r'",
        KeyCode::V => "l",
        KeyCode::T => "x",
        KeyCode::Y => "x",
        KeyCode::N => "x'",
        KeyCode::B => "x'",
        KeyCode::Period => "M'",
        KeyCode::X => "M",
        KeyCode::Key5 => "M",
        KeyCode::Key6 => "M",
        KeyCode::P => "z",
        KeyCode::Q => "z'",
        KeyCode::Z => "d",
        KeyCode::C => "u'",
        KeyCode::Comma => "u",
        KeyCode::Slash => "d'",
        _ => "",
    };
    Movement::from_str(movement_str).ok()
}

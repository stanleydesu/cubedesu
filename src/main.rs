use std::str::FromStr;

use cubedesu::*;
use macroquad::{input::KeyCode, math::Quat, prelude::*};

const F_LEN: f32 = 1.8; // side length of each facelet
const F_DEPTH: f32 = 0.; // thickness/depth of each facelet

#[macroquad::main("cubedesu")]
async fn main() {
    let mut gcube = GCube::new(3);
    let mut size_f = gcube.size as f32;
    let mut is_stickered = true;
    let mut camera = Camera3D {
        position: vec3(0., size_f * 3.5, size_f * 5.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    let desu_gray = Color::new(35. / 255., 39. / 255., 42. / 255., 1.);

    loop {
        if let Some(key) = get_last_key_pressed() {
            if key == KeyCode::Minus { gcube.shrink() } 
            else if key == KeyCode::Equal { gcube.grow() }
            else if key == KeyCode::Key1 { is_stickered = !is_stickered }
            else if let Some(movement) = key_to_movement(key) {
                gcube.apply_movement(&movement);
            }
            if size_f != gcube.size as f32 {
                camera.position *= gcube.size as f32 / size_f;
                size_f = gcube.size as f32;
            }
        }
        if is_key_down(KeyCode::Up) { camera.position.y += size_f / 7.; }
        if is_key_down(KeyCode::Down) { camera.position.y -= size_f / 7.; }
        let mut angle = 0.0;
        if is_key_down(KeyCode::Left) { angle = 0.05; }
        if is_key_down(KeyCode::Right) { angle = -0.05; }
        camera.position.y = clamp(camera.position.y, size_f * -3.5, size_f * 3.5);
        if angle != 0.0 {
            camera.position = Quat::from_rotation_y(angle).mul_vec3(camera.position);
        }
        set_camera(&camera);

        clear_background(desu_gray);
        for sticker in gcube.stickers.iter() {
            let curr = point3_to_vec3(sticker.current);
            draw_cube(
                curr,
                face_to_dimensions(gcube.get_curr_face(*sticker)),
                None,
                face_to_color(gcube.get_initial_face(*sticker)),
            );
            let mut mirr = curr;
            if mirr.x.abs() == size_f { mirr.x *= 2.4 }
            else if mirr.y.abs() == size_f { mirr.y *= 2.4 }
            else { mirr.z *= 2.4 }
            // only draw the inside face of the mirrored facelet
            let mirr_vec = curr - mirr;
            if (mirr - camera.position).dot(mirr_vec) < 0. {
                draw_cube(
                    mirr,
                    face_to_dimensions(gcube.get_curr_face(*sticker)),
                    None,
                    face_to_color(gcube.get_initial_face(*sticker)),
                );
            }
        }
        if is_stickered {
            let scale = if size_f > 20. { 1.96 } else { 1.98 };
            draw_cube(vec3(0., 0., 0.), 
                vec3(size_f * scale, size_f * scale, size_f * scale), 
                None, 
                desu_gray);
        }
        next_frame().await
    }
}

// returns the 3 closest faces on a cube to a Vec3
// fn closest_faces(p: Vec3) -> [Face; 3] {
//     let face_centers = vec![
//         // vec3(0.0)
//     ];
// }

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

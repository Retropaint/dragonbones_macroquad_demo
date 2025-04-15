use std::{
    cmp,
    f64::consts::PI,
    fs::File,
    io::{BufReader, Read},
    ops::Deref,
    thread,
    time::Duration,
};

use image::{imageops::crop_imm, ImageBuffer};
use macroquad::prelude::{animation::AnimatedSprite, *};
use rusty_dragonbones::runtime::{self, animate, load_dragonbones, prep_tex_for_rot, Prop, Vec2};
use serde_json::Value;
use structs::Model;
use zip::{read::ZipFile, ZipArchive};

mod structs;

macro_rules! f32 {
    ($thingy:expr) => {
        $thingy as f32
    };
}

#[macroquad::main("CBGM")]
async fn main() {
    // load *ske.json and *tex.json via the zip
    let path = File::open("./dragon.zip").unwrap();
    let mut zip = ZipArchive::new(&path).unwrap();
    let mut json1 = String::new();
    let mut json2 = String::new();
    zip.by_index(0).unwrap().read_to_string(&mut json1).unwrap();
    zip.by_index(2).unwrap().read_to_string(&mut json2).unwrap();
    let (mut dbroot, dbtex) = load_dragonbones(&mut json1, &mut json2).unwrap();

    // load texture
    let img = &mut vec![];
    zip.by_index(1).unwrap().read_to_end(img).unwrap();
    let tex = Texture2D::from_file_with_format(img, Some(ImageFormat::Png));

    let mut frame: i32 = 0;
    let mut anim_frame: i32 = 0;
    let mut offset = Vec2::default();
    //offset.x += 150.;
    //offset.y += 150.;
    let offset_speed = 10.;
    let col = Color::new(1., 1., 1., 1.);

    let gop = Texture2D::from_file_with_format(
        include_bytes!("/Users/o/downloads/gopher.png"),
        Some(ImageFormat::Png),
    );

    let mut img = image::load_from_memory_with_format(img, image::ImageFormat::Png).unwrap();
    let mut anim_idx = 0;
    offset.x = macroquad::window::screen_width() as f64 / 2.;
    offset.y = macroquad::window::screen_height() as f64 / 2.;
    loop {
        clear_background(WHITE);
        if is_mouse_button_down(MouseButton::Left) {
            anim_idx += 1;
            if anim_idx > dbroot.armature[0].animation.len()-1 {
                anim_idx = 0;
            }
        }

        let anim_idx = 0;
        let dur = dbroot.armature[0].animation[anim_idx].duration;
        let mut props: Vec<Prop> = animate(&mut dbroot, &dbtex, anim_idx, (frame) % (dur * 3), 3);
        props.sort_by(|a, b| (a.z as f64).total_cmp(&(b.z as f64)));

        if is_key_down(KeyCode::Right) {
            //anim_frame += 1;
            offset.x += offset_speed;
        }
        if is_key_down(KeyCode::Left) {
            //anim_frame -= 1;
            offset.x -= offset_speed;
        }
        if is_key_down(KeyCode::Up) {
            offset.y -= offset_speed;
        }
        if is_key_down(KeyCode::Down) {
            offset.y += offset_speed;
        }

        for mut p in props {
            prep_tex_for_rot(&mut p);

            draw_texture_ex(
                &tex,
                f32!(p.pos.x) + f32!(p.tex_pos.x) - (f32!(p.tex_size.x) / 2.) + f32!(offset.x),
                f32!(p.pos.y) + f32!(p.tex_pos.y) - (f32!(p.tex_size.y) / 2.) + f32!(offset.y),
                col,
                DrawTextureParams {
                    dest_size: Some(vec2(
                        f32!(p.tex_size.x * p.scale.x),
                        f32!(p.tex_size.y * p.scale.y),
                    )),
                    rotation: f32!((p.rot + p.tex_rot) * PI / 180.),
                    source: Some(Rect {
                        x: dbtex.sub_texture[p.tex_idx as usize].x as f32,
                        y: dbtex.sub_texture[p.tex_idx as usize].y as f32,
                        w: dbtex.sub_texture[p.tex_idx as usize].width as f32,
                        h: dbtex.sub_texture[p.tex_idx as usize].height as f32,
                    }),
                    pivot: Some(vec2(
                        f32!(p.pos.x) + f32!(offset.x),
                        f32!(p.pos.y) + f32!(offset.y),
                    )),
                    ..Default::default()
                },
            );

            // debug circles
            
            //draw_circle(
            //    p.pos.x as f32 + offset.x as f32,
            //    p.pos.y as f32 + offset.y as f32,
            //    5.,
            //    RED,
            //);
        }

        frame += 1;
        thread::sleep(Duration::from_millis(10));
        next_frame().await;
    }
}

// use egui::IconData;
use eframe::egui;
use egui::{Color32, Pos2, Vec2};
use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::time::Instant;
use egui::IconData;

struct TextEffect {
    text: String,
    position: Pos2,
    start_time: Instant,
    duration: f32,
}

impl TextEffect {
    fn new(text: String, position: Pos2) -> Self {
        Self {
            text,
            position,
            start_time: Instant::now(),
            duration: 2.0, // 2秒
        }
    }

    fn is_alive(&self) -> bool {
        self.start_time.elapsed().as_secs_f32() < self.duration
    }

    fn alpha(&self) -> f32 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        1.0 - (elapsed / self.duration).min(1.0)
    }

    fn current_position(&self) -> Pos2 {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        Pos2::new(
            self.position.x,
            self.position.y - elapsed * 50.0,
        )
    }
}

struct EnqingApp {
    person_offset: Vec2,
    jump_velocity: f32,
    gravity: f32,
    is_jumping: bool,
    jump_start_time: Option<Instant>,
    text_effects: Vec<TextEffect>,
    enqing_count: u32,
    jump_height: f32,
}

impl Default for EnqingApp {
    fn default() -> Self {
        Self {
            person_offset: Vec2::ZERO,
            jump_velocity: 0.0,
            gravity: 800.0,
            is_jumping: false,
            jump_start_time: None,
            text_effects: Vec::new(),
            enqing_count: 0,
            jump_height: 150.0,
        }
    }
}

impl EnqingApp {
    fn start_jump(&mut self) {
        if !self.is_jumping {
            self.is_jumping = true;
            self.jump_velocity = 500.0;
            self.jump_start_time = Some(Instant::now());
            self.enqing_count += 1;
        }
    }

    fn update_jump(&mut self, delta_time: f32) {
        if self.is_jumping {
            self.jump_velocity -= self.gravity * delta_time;//zhongli
            self.person_offset.y -= self.jump_velocity * delta_time;

            if self.person_offset.y >= 0.0 {
                self.person_offset.y = 0.0;
                self.is_jumping = false;
                self.jump_velocity = 0.0;
            }
        }
    }
}

impl eframe::App for EnqingApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let delta_time = ctx.input(|i| i.unstable_dt).min(1.0 / 30.0);

        self.update_jump(delta_time);

        self.text_effects.retain(|effect| effect.is_alive());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.ctx().set_visuals(egui::Visuals::dark());

            let (response, painter) = ui.allocate_painter(
                ui.available_size(),
                egui::Sense::click(),
            );

            painter.rect_filled(
                response.rect,
                0.0,
                Color32::from_rgb(30, 30, 40),
            );

            let center = response.rect.center();
            let person_pos = center + self.person_offset;

            let person_radius = 30.0;

            painter.circle_filled(
                person_pos,
                person_radius,
                Color32::from_rgb(100, 150, 255),
            );
///以下这个脸是ai写的，我不会画画。字符“```ಥ```”在微软雅黑无法正常显示
            // 绘制人物的脸////
            /*
            painter.circle_filled(
                person_pos + Vec2::new(-10.0, -5.0),
                5.0,
                Color32::BLACK,
            );
            painter.circle_filled(
                person_pos + Vec2::new(10.0, -5.0),
                5.0,
                Color32::BLACK,
            );
            painter.line_segment(
                [
                    person_pos + Vec2::new(-8.0, 10.0),
                    person_pos + Vec2::new(8.0, 10.0),
                ],
                (2.0, Color32::BLACK),
            );*/
////
            // 精确调整版：使用字符绘制流泪人脸和举手
            // 绘制眼睛（使用ಥ字符 - 这是一个表示悲伤的Unicode字符）
            let eye_char = "T";
            let eye_font = egui::FontId::proportional(18.0);

            // 左眼
            painter.text(
                person_pos + Vec2::new(-8.0, -3.0),
                egui::Align2::CENTER_CENTER,
                eye_char,
                eye_font.clone(),
                Color32::BLACK,
            );

            // 右眼
            painter.text(
                person_pos + Vec2::new(8.0, -3.0),
                egui::Align2::CENTER_CENTER,
                eye_char,
                eye_font,
                Color32::BLACK,
            );

            // 绘制眼泪
            //painter.line_segment(
            //    [
            //        person_pos + Vec2::new(-8.0, 8.0),
            //        person_pos + Vec2::new(-8.0, 18.0),
            //    ],
            //    (1.5, Color32::BLUE),
            //);
            //painter.line_segment(
            //    [
            //        person_pos + Vec2::new(8.0, 8.0),
            //        person_pos + Vec2::new(8.0, 18.0),
            //    ],
            //    (1.5, Color32::BLUE),
            //);

            // 绘制嘴巴（使用O字符表示惊讶/哭泣的嘴）
            let mouth_char = "O";
            let mouth_font = egui::FontId::proportional(14.0);

            painter.text(
                person_pos + Vec2::new(0.0, 12.0),
                egui::Align2::CENTER_CENTER,
                mouth_char,
                mouth_font,
                Color32::BLACK,
            );

            // 绘制举着的双手
            painter.line_segment(
                [
                    person_pos + Vec2::new(-25.0, -7.0),
                    person_pos + Vec2::new(-38.0, -65.0),
                ],
                (3.0, Color32::from_rgb(100, 150, 255)),
            );
            painter.line_segment(
                [
                    person_pos + Vec2::new(25.0, -7.0),
                    person_pos + Vec2::new(38.0, -65.0),
                ],
                (3.0, Color32::from_rgb(100, 150, 255)),
            );
            // 绘制文字效果
            for effect in &self.text_effects {
                let alpha = effect.alpha();
                let pos = effect.current_position();

                painter.text(
                    pos,
                    egui::Align2::CENTER_CENTER,
                    &effect.text,
                    egui::FontId::proportional(24.0),
                    Color32::from_rgba_premultiplied(255, 215, 0, (alpha * 255.0) as u8),
                );
            }

            painter.text(
                Pos2::new(response.rect.width() / 2.0, 50.0),
                egui::Align2::CENTER_CENTER,
                format!("恩情: {}", self.enqing_count),
                egui::FontId::proportional(36.0),
                Color32::from_rgb(255, 215, 0),
            );

            if response.clicked() {
                let click_pos = response.interact_pointer_pos();
                if let Some(pos) = click_pos {
                    let distance = (pos - person_pos).length();
                    if distance <= person_radius {
                        self.start_jump();

                        self.text_effects.push(TextEffect::new(
                            "恩情 +1 \\o/".to_string(),
                            pos,
                        ));
                    }
                }
            }
        });

        ctx.request_repaint();
    }
}
//////
///中文无法正常显示，需要添加字体。
fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "chinese_font".to_owned(),
        Arc::from(egui::FontData::from_static(include_bytes!("./fonts/msyh.ttc"))),
    );

    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "chinese_font".to_owned());

    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "chinese_font".to_owned());

    ctx.set_fonts(fonts);
}
///以下是ai写的，需要系统1又```Microsoft YaHei```字体
//    fn setup_fonts(ctx: &egui::Context) {
//        let mut fonts = egui::FontDefinitions::default();

        // 在 Windows 上，添加常见的中文字体名称
//        fonts
//            .families
//            .get_mut(&egui::FontFamily::Proportional)
//            .unwrap()
//            .insert(0, "Microsoft YaHei".to_owned());

//        fonts
//            .families
//            .get_mut(&egui::FontFamily::Monospace)
//            .unwrap()
//            .insert(0, "Microsoft YaHei".to_owned());

//        ctx.set_fonts(fonts);
//    }
///目前音乐能找到但是无法播放。
fn play_background_music() -> Result<(), Box<dyn std::error::Error>> {
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        if let Ok(file) = File::open("music.m4a") {
            let source = Decoder::new(BufReader::new(file))?;
            let _ = stream_handle.play_raw(source.convert_samples());
            println!("音乐开始播放");
        } else {
            println!("未找到音乐文件，请将音乐文件命名为 'music.mp3' 并放在程序同目录下");
        }
    } else {
        println!("无法初始化音频输出");
    }

    Ok(())
}

fn load_icon_from_file(path: &str) -> Option<IconData> {
    let image = image::open(path).ok()?;
    let image = image.into_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Some(IconData {
        rgba,
        width,
        height,
    })
}

fn main() -> Result<(), eframe::Error> {
    let icon_data = load_icon_from_file("../../assets/icon.jpg").unwrap_or_else(|| {
        eprintln!("无法加载图标文件，使用默认图标");
        IconData::default() // 使用默认的 IconData
    });

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 600.0])
            .with_title("恩情软件")
            .with_icon(icon_data) // 添加图标
            .with_resizable(false),
        ..Default::default()
    };

    std::thread::spawn(|| {
        let _ = play_background_music();
    });

    eframe::run_native(
        "恩情软件",
        options,
        Box::new(|cc| {
            setup_fonts(&cc.egui_ctx);
            Ok(Box::<EnqingApp>::default())
        }),
    )
}

use bevy::prelude::*;
use bevy::diagnostic::*;

const FONT_TTF: &str = "FiraCode-Regular.ttf";

const FONT_SIZE: f32 = 20.0;
const FONT_COLOR: Color = Color::YELLOW;

const INFO_TOP: f32 = 8.0;
const INFO_RIGHT: f32 = 8.0;


#[derive(Component, Clone)]
pub struct FPS;


#[derive(Bundle, Clone)]
pub struct Info {
    pub fps: FPS,

    #[bundle]
    pub text_bundle: TextBundle,
}

impl Info {
    pub fn new(font_handle: &Handle<Font>) -> Self {

        let text_style = TextStyle {
            font: font_handle.clone(),
            font_size: FONT_SIZE,
            color: FONT_COLOR,
        };

        Self {
            fps: FPS,

            text_bundle: TextBundle::from_sections([
                TextSection::new("\nfps: ", text_style.clone()),    // 0
                TextSection::from_style(text_style.clone()),        // 1
            ])
            .with_style(Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(INFO_TOP),
                    right: Val::Px(INFO_RIGHT),
                    ..default()
                },
                ..default()
            }),
        }

    }
}


pub fn create_info(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    print!("make fps text...");

    let font_handle:Handle<Font> = asset_server.load(FONT_TTF);

    commands.spawn(Info::new(&font_handle));

    println!("...done.");
}

pub fn update_info(
    mut query: Query<
        &mut Text,
        With<FPS>
    >,
    diagnostics: Res<Diagnostics>,
) {
    if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.value() {
            let mut text = query.single_mut();
            text.sections[1].value =  format!("{value:.0}");
        }
    }
}
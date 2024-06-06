mod quad;

use bevy::{prelude::Color as BevyColor, prelude::*, sprite::*};
use bevy_iced::iced::widget::button;
use bevy_iced::iced::widget::container::Appearance;
use bevy_iced::iced::{
    widget::{column, container, horizontal_rule, row, slider, text},
    Alignment, Background, Font, Radians, Shadow, Vector,
};
use bevy_iced::iced::{Color, Element};
use bevy_iced::{IcedContext, IcedPlugin};
use rand::random as rng;

// const ALPHAPROTA_FONT: Font = Font::with_name("Alpha Prota");
// const ALPHAPROTA_FONT_BYTES: &'static [u8] = include_bytes!("../assets/fonts/AlphaProta.ttf");

#[derive(Clone, Event)]
pub enum UiMessage {
    StartChanged(Color),
    EndChanged(Color),
    AngleChanged(Radians),
    ShadowColorChanged(Color),
    BackgroundOpacityChanged(f32),
    ShadowOffsetXChanged(f32),
    ShadowOffsetYChanged(f32),
    QuadColorChanged(Color),
    ObjectSpawned,
    // EventHappened(Event),
}

#[derive(Component)]
struct ColorText;

#[derive(Component, Resource)]
struct IcedData {
    start: Color,
    end: Color,
    angle: Radians,
    shadow: Shadow,
    background_opacity: f32,
    text_color: Color,
    // radius: [f32; 4],
    quad_color: Color,
    // events: EventsWidget,
}

#[derive(Component)]
struct BevyData;

pub fn main() {
    let quad_color = [0.0, 1.0, 1.0, 1.0];

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(IcedPlugin::default())
        .add_event::<UiMessage>()
        .insert_resource(IcedData {
            start: Color::new(1.0, 0.5, 1.0, 1.0),
            end: Color::new(0.0, 0.0, 1.0, 1.0),
            angle: Radians(0.0),
            shadow: Shadow {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.8),
                offset: Vector::new(0.0, 8.0),
                blur_radius: 16.0,
            },
            background_opacity: 1.0,
            text_color: Color::from_rgb(0.0, 0.0, 0.0),
            // radius: [50.0; 4],
            quad_color: Color::from_rgba(
                quad_color[0],
                quad_color[1],
                quad_color[2],
                quad_color[3],
            ),
            // events: EventsWidget {
            //     ..Default::default()
            // },
        })
        .add_systems(Startup, setup)
        .add_systems(Update, ui_system)
        .add_systems(Update, message_system)
        .run();
}

fn message_system(
    mut commands: Commands,
    mut messages: EventReader<UiMessage>,
    mut data: ResMut<IcedData>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut quad_query: Query<(Entity, &mut Handle<ColorMaterial>), With<BevyData>>,
) {
    for msg in messages.read() {
        match msg {
            UiMessage::StartChanged(color) => data.start = *color,
            UiMessage::EndChanged(color) => data.end = *color,
            UiMessage::AngleChanged(angle) => data.angle = *angle,
            UiMessage::ShadowColorChanged(color) => data.shadow.color = *color,
            UiMessage::BackgroundOpacityChanged(opacity) => {
                if opacity < &0.05 {
                    data.text_color = Color::from_rgb(1.0, 1.0, 1.0);
                } else {
                    data.text_color = Color::from_rgb(0.0, 0.0, 0.0);
                }
                data.background_opacity = *opacity;
            }
            UiMessage::QuadColorChanged(color) => {
                for (x, y) in quad_query.iter() {
                    // let color_material = materials.get_mut(y).unwrap();
                    commands
                        .entity(x)
                        .insert(materials.add(ColorMaterial::from(BevyColor::rgba(
                            color.r, color.g, color.b, color.a,
                        ))));
                }
                // data.quad_color = *color;
            }
            UiMessage::ShadowOffsetXChanged(x) => {
                data.shadow.offset.x = *x;
            }
            UiMessage::ShadowOffsetYChanged(y) => {
                data.shadow.offset.y = *y;
            }
            // Message::EventHappened(event) => {
            //     data.events.last.push(event);
            //
            //     if data.events.last.len() > 2 {
            //         let _ = data.events.last.remove(0);
            //     }
            // }
            UiMessage::ObjectSpawned => {
                let pos = (Vec3::new(rng(), rng(), 0.0) - Vec3::new(0.5, 0.5, 0.0)) * 300.0;
                commands.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: BevyColor::rgba_u8(rng(), rng(), rng(), rng()),
                        custom_size: Some(Vec2::new(50.0, 50.0)),
                        ..Default::default()
                    },
                    transform: Transform::from_translation(pos),
                    ..Default::default()
                });
            }
        }
    }
}

fn ui_system(mut _commands: Commands, mut ctx: IcedContext<UiMessage>, data: ResMut<IcedData>) {
    ctx.display(column!(container(column!(
        row!(
            text("Background Opacity"),
            slider(
                0.0..=1.0,
                data.background_opacity,
                UiMessage::BackgroundOpacityChanged
            )
            .step(0.01),
            text(format!(" {:.2}", data.background_opacity))
        )
        .padding(8)
        .spacing(8),
        container(horizontal_rule(0)).padding(8),
        // row!(column!(
        //     color_picker("Start", data.start).map(UiMessage::StartChanged),
        //     color_picker("End", data.end).map(UiMessage::EndChanged),
        //     // angle_picker,
        // )),
        // container(horizontal_rule(0)).padding(8),
        row!(text("The Box").font(Font {
            weight: bevy_iced::iced::font::Weight::Bold,
            ..Default::default()
        }))
        .padding(8)
        .spacing(8),
        column!(
            color_picker("Color:", data.quad_color).map(UiMessage::QuadColorChanged),
            // color_picker("Shadow:", data.shadow.color).map(UiMessage::ShadowColorChanged),
            // row![
            //     text("Offset: ").width(64),
            //     row![
            //         text("X: "),
            //         slider(
            //             -100.0..=100.0,
            //             data.shadow.offset.x,
            //             UiMessage::ShadowOffsetXChanged
            //         )
            //         .step(0.01),
            //         text(format!(" {:.2}", data.shadow.offset.x)),
            //     ],
            //     row![
            //         text("Y: "),
            //         slider(
            //             -50.0..=100.0,
            //             data.shadow.offset.y,
            //             UiMessage::ShadowOffsetYChanged
            //         )
            //         .step(0.01),
            //         text(format!(" {:.2}", data.shadow.offset.y)),
            //     ],
            // ]
            // .padding(8)
            // .spacing(8),
        ),
        container(horizontal_rule(0)).padding(8),
        column!(
            row!(text("Spawn an Object (In Bevy)").font(Font {
                weight: bevy_iced::iced::font::Weight::Bold,
                ..Default::default()
            }))
            .padding(8)
            .spacing(8),
            // row![text(format!("{:?}", self.events.last[0]))],
            row!(button("Spawn: ").on_press(UiMessage::ObjectSpawned),)
                .padding(8)
                .spacing(8),
        ),
    ),)
    .style(Appearance {
        text_color: Some(data.text_color),
        background: Some(Background::Color(Color::from_rgba(
            1.0,
            1.0,
            1.0,
            data.background_opacity
        ))),
        ..Default::default()
    }),));
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    data: Res<IcedData>,
) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        BevyData,
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::default()).into(),
            transform: Transform::default()
                .with_scale(Vec3::splat(128.))
                .with_translation(Vec3::new(0.0, -150.0, 0.0)),
            material: materials.add(BevyColor::rgba(
                data.quad_color.r,
                data.quad_color.g,
                data.quad_color.b,
                data.quad_color.a,
            )),
            ..default()
        },
    ));
    commands.spawn((
        TextBundle::from_section(
            "This: is inside Bevy.",
            TextStyle {
                font_size: 100.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        ColorText,
    ));
}

fn color_picker(label: &str, color: Color) -> Element<'_, Color> {
    row![
        text(label).width(64),
        row![
            text("R: "),
            slider(0.0..=1.0, color.r, move |r| { Color { r, ..color } }).step(0.01),
            text(format!(" {:.2}", color.r)),
        ],
        row![
            text("G: "),
            slider(0.0..=1.0, color.g, move |g| { Color { g, ..color } }).step(0.01),
            text(format!(" {:.2}", color.g)),
        ],
        row![
            text("B: "),
            slider(0.0..=1.0, color.b, move |b| { Color { b, ..color } }).step(0.01),
            text(format!(" {:.2}", color.b)),
        ],
        row![
            text("A: "),
            slider(0.0..=1.0, color.a, move |a| { Color { a, ..color } }).step(0.01),
            text(format!(" {:.2}", color.a)),
        ],
        quad::CustomQuad::new(
            Color::from_rgba(color.r, color.g, color.b, color.a),
            20.0,
            [4.0, 4.0, 4.0, 4.0],
            Shadow::default(),
        ),
    ]
    .padding(8)
    .spacing(8)
    .align_items(Alignment::Center)
    .into()
}

mod quiz;
use quiz::*;

use bevy::prelude::*;

struct TextObj;
struct BoxObj;
struct ClueBox;
struct ClueText;
struct ReadingClue(bool);

struct Geometry {
    title: Rect<f32>,
    categories: [Rect<f32>; 6],
    clues: [[Rect<f32>; 5]; 6],
    //cluebox: Rect<f32>,
}

impl Default for Geometry {
    fn default() -> Self {
        let title = Rect {
            top: 1.0,
            bottom: 1.0 - 0.15,
            left: 0.0,
            right: 1.0,
        };

        let mut categories: [Rect<f32>; 6] = Default::default();
        let ncategories = categories.len() as f32;
        for (i, c) in categories.iter_mut().enumerate() {
            *c = Rect {
                top: 1.0 - 0.15,
                bottom: 1.0 - 0.3,
                left: i as f32 / ncategories,
                right: (i + 1) as f32 / ncategories,
            };
        }

        let mut clues: [[Rect<f32>; 5]; 6] = Default::default();
        for (i, col) in clues.iter_mut().enumerate() {
            for (j, clue) in col.iter_mut().enumerate() {
                *clue = Rect {
                    top: 1.0 - 0.3 - 0.7 * j as f32 / ncategories,
                    bottom: 1.0 - 0.3 - 0.7 * (j + 1) as f32 / ncategories,
                    left: i as f32 / ncategories,
                    right: (i + 1) as f32 / ncategories,
                };
            }
        }

        /*
        let cluebox = Rect {
            top: 1.0 - 0.4,
            bottom: 1.0 - 0.8,
            left: 0.2,
            right: 0.8,
        };
        */

        Self {
            title,
            categories,
            clues,
            //cluebox,
        }
    }
}

fn main() {
    let quiz = Quiz::new("assets/quiz.xml").unwrap();
    let window_width = 1800.0f32;
    let window_height = (window_width / 1.8).floor();
    let geometry = Geometry::default();
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Jeopardy".to_string(),
            width: window_width,
            height: window_height,
            scale_factor_override: Some(1.0),
            ..Default::default()
        })
        .insert_resource(ReadingClue(false))
        .insert_resource(quiz)
        .insert_resource(geometry)
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(user_click.system())
        .run();
}

fn make_box(size: &Size<f32>, raw: &Rect<f32>) -> Rect<f32> {
    Rect {
        top: raw.top * size.height,
        bottom: raw.bottom * size.height,
        left: raw.left * size.width,
        right: raw.right * size.width,
    }
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    quiz: Res<Quiz>,
    geometry: Res<Geometry>,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // Window setup
    let window = windows.get_primary_mut().unwrap();
    let size = Size::new(window.width(), window.height());
    let font = asset_server.load("korinan.ttf");

    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Make the title
    gen_text(
        &mut commands,
        &mut materials,
        (&quiz.name).as_deref().unwrap_or("Quiz!"),
        &size,
        make_box(&size, &geometry.title),
        font.clone(),
        100.0,
        Color::YELLOW,
        Color::NONE,
    );

    let amounts: Vec<i32> = vec![200, 400, 600, 800, 1000];
    for (i, col) in quiz.category.iter().enumerate() {
        gen_text(
            &mut commands,
            &mut materials,
            &col.name,
            &size,
            make_box(&size, &geometry.categories[i]),
            font.clone(),
            50.,
            Color::WHITE,
            Color::BLUE,
        );

        for (j, _) in col.clue.iter().enumerate() {
            let tbox = make_box(&size, &geometry.clues[i][j]);

            let text = format!("${}", amounts[j]);
            gen_text(
                &mut commands,
                &mut materials,
                &text,
                &size,
                tbox,
                font.clone(),
                50.,
                Color::ORANGE,
                Color::BLUE,
            );

/*
            let mut new_box: SpriteBundle = blue_box.clone();
            new_box.transform = Transform {
                translation: Vec3::new(
                    x - (window.width() / 1.9), // idk why 1.9, just seems to work
                    y - (window.height() / 2.),
                    10.,
                ),
                ..Default::default()
            };
            commands.spawn_bundle(new_box).insert(BoxObj);
*/
        }
    }
}

// https://www.reddit.com/r/bevy/comments/kowbxa/comment/ghvbp2j/
fn gen_text(
    command: &mut Commands,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    s: &str,
    size: &Size<f32>,
    position: Rect<f32>,
    font: Handle<Font>,
    font_size: f32,
    text_color: Color,
    box_color: Color,
) {
    let box_margin = 0.005 * f32::max(size.width, size.height);

    let style = Style {
        align_items: AlignItems::FlexEnd,
        justify_content: JustifyContent::Center,
        position: Rect {
            bottom: Val::Px(position.bottom + box_margin),
            left: Val::Px(position.left + box_margin),
            ..Default::default()
        },
        position_type: PositionType::Absolute,
        size: Size::new(
            Val::Px(position.right - position.left - 2.0 * box_margin),
            Val::Px(position.top - position.bottom - 2.0 * box_margin),
        ),
        ..Default::default()
    };

    let material = materials.add(box_color.into());

    let node_bundle = NodeBundle {
        style,
        material,
        ..Default::default()
    };

    let text = Text::with_section(
        s,
        TextStyle {
            font,
            font_size,
            color: text_color,
        },
        TextAlignment {
            horizontal: HorizontalAlign::Center,
            vertical: VerticalAlign::Center,
        },
    );

    let style = Style {
        align_self: AlignSelf::Center,
/*
        align_items: AlignItems::Center,
        flex_wrap: FlexWrap::Wrap,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        position: Rect {
            bottom: Val::Percent(0.0),
            left: Val::Percent(0.0),
            ..Default::default()
        },
        position_type: PositionType::Absolute,
        size: Size::new(
            Val::Percent(100.0),
            Val::Percent(100.0),
        ),
*/
        ..Default::default()
    };

    let text_bundle = TextBundle {
        text,
        style,
        ..Default::default()
    };

    command
        .spawn_bundle(node_bundle)
        .with_children(|parent| {
            parent.spawn_bundle(text_bundle);
        }).insert(TextObj);
}

#[allow(clippy::too_many_arguments)]
fn user_click(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    mut box_query: Query<(Entity, &mut Transform, &Sprite, With<BoxObj>)>,
    mut text_query: Query<(Entity, &mut Style, With<TextObj>)>,
    mut clue_box_query: Query<(Entity, With<ClueBox>)>,
    mut clue_text_query: Query<(Entity, With<ClueText>)>,
    windows: Res<Windows>,
    quiz: Res<Quiz>,
    geometry: Res<Geometry>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut reading: ResMut<ReadingClue>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        if reading.0 {
            for (clue_text_entity, _) in clue_text_query.iter_mut() {
                commands.entity(clue_text_entity).despawn();
            }
            for (clue_box_entity, _) in clue_box_query.iter_mut() {
                commands.entity(clue_box_entity).despawn();
            }
            let mut text_iter: i32 = 0;
            for (_, mut text_style, _) in text_query.iter_mut() {
                if text_iter < 7 {
                    // To keep categories + title unmoved
                    text_iter += 1;
                    continue;
                }
                let new_bottom: Val = text_style.position.bottom + (-5000.);
                let new_right: Val = text_style.position.right + (-5000.);
                text_style.position = Rect {
                    bottom: new_bottom,
                    right: new_right,
                    ..Default::default()
                }
            }
            reading.0 = !reading.0;
        } else {
            let win = windows.get_primary().expect("No Window");
            let mouse_pos: Vec2 = win.cursor_position().expect("No Mouse Pos");
            let size = Size::new(win.width(), win.height());
            let font = asset_server.load("korinan.ttf");
            let mut i: i32 = 0;
            for (_, mut box_tf, box_sprite, _) in box_query.iter_mut() {
                //println!("Box: {}", box_tf.translation);
                if (i % 6) != 0
                    && mouse_pos.x < box_tf.translation.x + (box_sprite.size.x / 2.)
                    && mouse_pos.x > box_tf.translation.x - (box_sprite.size.x / 2.)
                    && mouse_pos.y < box_tf.translation.y + (box_sprite.size.y / 2.)
                    && mouse_pos.y > box_tf.translation.y - (box_sprite.size.y / 2.)
                {
                    // Move out of way rather than despawn because of future iteration
                    box_tf.translation = Vec3::new(9000., 9000., 15.);

                    for (j, (_, mut text_style, _)) in text_query.iter_mut().enumerate() {
                        //println!("j{}", j);
                        if Some(i) == text_to_box_coords(j as i32 - 1) {
                            // Move out of way rather than despawn because of future iteration
                            let new_bottom: Val = text_style.position.bottom + 5000.;
                            let new_right: Val = text_style.position.right + 5000.;
                            text_style.position = Rect {
                                bottom: new_bottom,
                                right: new_right,
                                ..Default::default()
                            };
                            break;
                        }
                    }

                    let mut clue_box = SpriteBundle {
                        material: materials.add((Color::MIDNIGHT_BLUE).into()),
                        sprite: Sprite::new(Vec2::new(800., 320.)),
                        ..Default::default()
                    };
                    clue_box.transform = Transform {
                        translation: Vec3::new(0., -10., 15.),
                        ..Default::default()
                    };
                    commands.spawn_bundle(clue_box).insert(ClueBox);

                    let clue_text: &str = quiz.get_clue(i as usize);
                    let (ic, jc) = clue_coords(i as usize);
                    let clue_box = make_box(&size, &geometry.clues[ic][jc]);
                    gen_text(
                        &mut commands,
                        &mut materials,
                        clue_text,
                        &size,
                        clue_box,
                        font.clone(),
                        50.,
                        Color::WHITE,
                        Color::BLUE,
                    );
                    let mut text_iter: i32 = 0;
                    for (_, mut text_style, _) in text_query.iter_mut() {
                        if text_iter < 7 {
                            // To keep categories + title unmoved:
                            // genuinely optional, but I like it
                            text_iter += 1;
                            continue;
                        }
                        let new_bottom: Val = text_style.position.bottom + 5000.;
                        let new_right: Val = text_style.position.right + 5000.;
                        text_style.position = Rect {
                            bottom: new_bottom,
                            right: new_right,
                            ..Default::default()
                        };
                    }

                    reading.0 = !reading.0;

                    break;
                }
                i += 1;
            }
        }
    }
}

fn text_to_box_coords(n: i32) -> Option<i32> {
    if (0..=35).contains(&n) {
        Some(6 * (5 - n % 6) + n / 6)
    } else {
        None
    }
}

use bevy::prelude::*;

pub fn spawn_ability_modal(parent: &mut ChildSpawnerCommands) {
    parent
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                padding: UiRect {
                    left: Val::Px(15.0),
                    bottom: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|container| {
            container
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Row,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        column_gap: Val::Px(14.0),
                        padding: UiRect::all(Val::Px(8.0)),
                        border_radius: BorderRadius::all(Val::Px(12.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.08, 0.09, 0.12, 0.65)),
                ))
                .with_children(|row| {
                    let abilities = [
                        (
                            "ALL",
                            Color::srgb(0.2, 0.6, 0.9),
                            Color::srgb(0.1, 0.25, 0.5),
                        ),
                        (
                            "NONE",
                            Color::srgb(0.9, 0.5, 0.2),
                            Color::srgb(0.5, 0.25, 0.1),
                        ),
                        (
                            "NONE",
                            Color::srgb(0.7, 0.3, 0.9),
                            Color::srgb(0.4, 0.15, 0.5),
                        ),
                    ];

                    for (label, border_col, bg_col) in abilities {
                        row.spawn((
                            Button,
                            Node {
                                width: Val::Px(60.0),
                                height: Val::Px(60.0),
                                border: UiRect::all(Val::Px(2.0)),
                                justify_content: JustifyContent::Center,
                                align_items: AlignItems::Center,
                                border_radius: BorderRadius::all(Val::Px(10.0)),
                                ..default()
                            },
                            Interaction::None,
                            BackgroundColor(bg_col),
                            BorderColor::all(border_col),
                        ))
                        .with_children(|btn| {
                            btn.spawn((
                                Text::new(label),
                                TextFont {
                                    font_size: FontSize::Px(12.0),
                                    ..default()
                                },
                                TextColor(Color::WHITE),
                            ));
                        });
                    }
                });
        });
}

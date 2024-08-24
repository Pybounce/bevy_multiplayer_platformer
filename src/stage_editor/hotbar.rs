
/*
    commands.spawn(NodeBundle {
        background_color: BackgroundColor(Color::srgb(0.0, 0.0, 1.0)),
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    })
    .insert(Interaction::None);

    commands.spawn(NodeBundle {
        style: Style {
            flex_direction: FlexDirection::Row,
            align_self: AlignSelf::Stretch,
            width: Val::Percent(50.),
            height: Val::Percent(5.),
            overflow: Overflow::clip_x(),
            ..default()
        },
        background_color: Color::srgb(0.10, 0.10, 0.10).into(),
        ..default()
    })
    .insert(Interaction::None)
    .insert(FocusPolicy::Block)
    .with_children(|parent| {
        // Moving panel
        parent
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        top: Val::Px(0.0),
                        ..default()
                    },
                    ..default()
                },
                //ScrollingList::default(),
            ))
            .with_children(|parent| {
                // List items
                for i in 0..30 {
                    parent.spawn((
                        TextBundle::from_section(
                            format!("Item {i}"),
                            TextStyle {
                                ..default()
                            },
                        ),
                        Label,
                    ));
                }
            });
    });

fn hovering(
    mut query: Query<(&mut BackgroundColor, &Interaction)>
) {
    for (mut bg, i) in &mut query {
        match i {
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.0, 1.0, 0.0);
            },
            Interaction::None => {
                bg.0 = Color::srgb(1.0, 0.0, 0.0);
            },
            _ => ()
        }
    }
}

//ok so you have a box with overflow on clip y
// then you have the list container inside that
// the list container itself contains a list of elements (list items)
// but the bottom of the list container is overflowed so we don't see it
// then we move the list container (ie every item in the list) up a bit, and that creates scrolling
    // since it is now clipping the top y and bottom y etc


*/
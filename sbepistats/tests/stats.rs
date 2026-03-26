use bevy::prelude::*;
use sbepistats::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatsPlugin))
        .add_stat_type::<Speed>()
        .add_stat_modifier_add::<Speed, SpeedBoost>(|_| 0.2)
        .add_stat_modifier_add::<Speed, SpeedBooster>(|_| 0.3)
        .add_stat_type::<PowerLevel>()
        .add_stat_modifier_add::<PowerLevel, PowerUp>(|_| 1);
    app
}

#[derive(StatType)]
struct Speed;

#[derive(Component)]
struct SpeedBoost;

#[derive(Component)]
struct SpeedBooster;

#[derive(StatType)]
#[stat_type(u32)]
struct PowerLevel;

#[derive(Component)]
struct PowerUp;

#[test]
fn f32_stat_without_modifier() {
    let mut app = app();
    app.world_mut().spawn(Stat::<Speed>::new(1.0));
    app.update();
    assert_eq!(
        1.0,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn f32_stat_with_modifier() {
    let mut app = app();
    app.world_mut().spawn((Stat::<Speed>::new(1.0), SpeedBoost));
    app.update();
    assert_eq!(
        1.2,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn f32_stat_with_two_modifier() {
    let mut app = app();
    app.world_mut()
        .spawn((Stat::<Speed>::new(1.0), SpeedBoost, SpeedBooster));
    app.update();
    assert_eq!(
        1.5,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn u32_stat_without_modifier() {
    let mut app = app();
    app.world_mut().spawn(Stat::<PowerLevel>::new(1));
    app.update();
    assert_eq!(
        1,
        app.world_mut()
            .query::<&Stat::<PowerLevel>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn u32_stat_with_modifier() {
    let mut app = app();
    app.world_mut().spawn((Stat::<PowerLevel>::new(1), PowerUp));
    app.update();
    assert_eq!(
        2,
        app.world_mut()
            .query::<&Stat::<PowerLevel>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

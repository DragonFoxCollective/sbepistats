use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use sbepistats::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, TestPlugin));
    app
}

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(StatType)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = StatTypeHook)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = ConfigureStatTypeAddHook)]
struct Speed;

#[derive(Component)]
struct Accelerate;

#[auto_system(plugin = TestPlugin, schedule = PreUpdate, config(
    in_set = StatSystems::<Speed>::Op(DataTypeOp::Add),
))]
fn accelerate(mut query: Query<(&mut Stat<Speed>, &Stat<Acceleration>), With<Accelerate>>) {
    for (mut stat, boost) in query.iter_mut() {
        stat.add_modifier(boost.total());
    }
}

#[derive(StatType)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = StatTypeHook)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = OrderStatBeforeHook::<Speed>::default())]
struct Acceleration;

#[derive(Component)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = StatModifierAddHook::<Acceleration>::default())]
struct AccelerationBoost;

impl StatModifierAdd<Acceleration> for AccelerationBoost {
    fn add(&self) -> f32 {
        0.2
    }
}

#[derive(StatType)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = StatTypeHook)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = OrderStatAfterHook::<Speed>::default())]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = ConfigureStatTypeAddHook)]
struct Position;

#[derive(Component)]
struct Move;

#[auto_system(plugin = TestPlugin, schedule = PreUpdate, config(
    in_set = StatSystems::<Position>::Op(DataTypeOp::Add),
))]
fn r#move(mut query: Query<(&mut Stat<Position>, &Stat<Speed>), With<Move>>) {
    for (mut stat, boost) in query.iter_mut() {
        stat.add_modifier(boost.total());
    }
}

#[test]
fn auto_plugin_f32_stat_without_modifier() {
    let mut app = app();
    app.world_mut().spawn(Stat::<Acceleration>::new(1.0));
    app.update();
    assert_eq!(
        1.0,
        app.world_mut()
            .query::<&Stat::<Acceleration>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn auto_plugin_f32_stat_with_modifier() {
    let mut app = app();
    app.world_mut()
        .spawn((Stat::<Acceleration>::new(1.0), AccelerationBoost));
    app.update();
    assert_eq!(
        1.2,
        app.world_mut()
            .query::<&Stat::<Acceleration>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn auto_plugin_order_stat_before() {
    let mut app = app();
    app.world_mut().spawn((
        Stat::<Speed>::new(1.0),
        Accelerate,
        Stat::<Acceleration>::new(1.0),
        AccelerationBoost,
    ));
    app.update();
    assert_eq!(
        2.2,
        app.world_mut()
            .query::<&Stat::<Speed>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

#[test]
fn auto_plugin_order_stat_after() {
    let mut app = app();
    app.world_mut().spawn((
        Stat::<Position>::new(1.0),
        Move,
        Stat::<Speed>::new(1.0),
        Accelerate,
        Stat::<Acceleration>::new(1.0),
        AccelerationBoost,
    ));
    app.update();
    assert_eq!(
        3.2,
        app.world_mut()
            .query::<&Stat::<Position>>()
            .single(app.world())
            .unwrap()
            .total()
    )
}

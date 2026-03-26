use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use sbepistats::*;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatsPlugin, TestPlugin));
    app
}

#[derive(AutoPlugin)]
#[auto_plugin(impl_plugin_trait)]
struct TestPlugin;

#[derive(StatType)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = StatTypeHook)]
struct Speed;

#[derive(Component)]
#[auto_plugin_build_hook(plugin = TestPlugin, hook = StatModifierAddHook::<Speed>::default())]
struct SpeedBoost;

impl StatModifierAdd<Speed> for SpeedBoost {
    fn add(&self) -> f32 {
        0.2
    }
}

#[test]
fn auto_plugin_f32_stat_without_modifier() {
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
fn auto_plugin_f32_stat_with_modifier() {
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

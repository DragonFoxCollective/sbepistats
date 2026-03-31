//! Compatibility hooks for `bevy_auto_plugin`.

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;
use derive_where::derive_where;

use crate::{Add, AppExt, Mul, StatModifierAdd, StatModifierMul, StatType};

/// [`bevy_auto_plugin`] build hook for [`AppExt::add_stat_type`].
///
/// ```rust
/// # use bevy_auto_plugin::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(AutoPlugin)]
/// # #[auto_plugin(impl_plugin_trait)]
/// # struct MyPlugin;
/// #
/// #[derive(StatType)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatTypeHook)]
/// struct MyStat;
/// ```
pub struct StatTypeHook;

impl<T: StatType<DataType: Clone + Send + Sync + 'static> + Send + Sync + 'static>
    AutoPluginBuildHook<T> for StatTypeHook
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_type::<T>();
    }
}

/// [`bevy_auto_plugin`] build hook for [`AppExt::configure_stat_type_add`].
///
/// ```rust
/// # use bevy_auto_plugin::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(AutoPlugin)]
/// # #[auto_plugin(impl_plugin_trait)]
/// # struct MyPlugin;
/// #
/// #[derive(StatType)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatTypeHook)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = ConfigureStatTypeAddHook)]
/// struct MyStat;
/// ```
pub struct ConfigureStatTypeAddHook;

impl<T: StatType<DataType: Add + Clone + Send + Sync + 'static> + Send + Sync + 'static>
    AutoPluginBuildHook<T> for ConfigureStatTypeAddHook
{
    fn on_build(&self, app: &mut App) {
        app.configure_stat_type_add::<T>();
    }
}

/// [`bevy_auto_plugin`] build hook for [`AppExt::add_stat_modifier_add`].
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_auto_plugin::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(AutoPlugin)]
/// # #[auto_plugin(impl_plugin_trait)]
/// # struct MyPlugin;
/// #
/// # #[derive(StatType)]
/// # #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatTypeHook)]
/// # struct MyStat;
/// #
/// #[derive(Component)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatModifierAddHook::<MyStat>::default())]
/// struct MyStatModifier;
///
/// impl StatModifierAdd<MyStat> for MyStatModifier {
///     fn add(&self) -> f32 {
///         1.0
///     }
/// }
/// ```
#[derive_where(Default)]
pub struct StatModifierAddHook<T>(PhantomData<T>);

impl<
    T: StatModifierAdd<S> + Component + 'static,
    S: StatType<DataType: Add + Clone + Send + Sync + 'static> + Send + Sync + 'static,
> AutoPluginBuildHook<T> for StatModifierAddHook<S>
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_modifier_add::<S, T>();
    }
}

/// [`bevy_auto_plugin`] build hook for [`AppExt::configure_stat_type_mul`].
///
/// ```rust
/// # use bevy_auto_plugin::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(AutoPlugin)]
/// # #[auto_plugin(impl_plugin_trait)]
/// # struct MyPlugin;
/// #
/// #[derive(StatType)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatTypeHook)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = ConfigureStatTypeMulHook)]
/// struct MyStat;
/// ```
pub struct ConfigureStatTypeMulHook;

impl<T: StatType<DataType: Mul + Add + Clone + Send + Sync + 'static> + Send + Sync + 'static>
    AutoPluginBuildHook<T> for ConfigureStatTypeMulHook
{
    fn on_build(&self, app: &mut App) {
        app.configure_stat_type_mul::<T>();
    }
}

/// [`bevy_auto_plugin`] build hook for [`AppExt::add_stat_modifier_mul`].
///
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_auto_plugin::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(AutoPlugin)]
/// # #[auto_plugin(impl_plugin_trait)]
/// # struct MyPlugin;
/// #
/// # #[derive(StatType)]
/// # #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatTypeHook)]
/// # struct MyStat;
/// #
/// #[derive(Component)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatModifierMulHook::<MyStat>::default())]
/// struct MyStatModifier;
///
/// impl StatModifierMul<MyStat> for MyStatModifier {
///     fn mul_before(&self) -> f32 {
///         1.0
///     }
/// }
/// ```
#[derive_where(Default)]
pub struct StatModifierMulHook<T>(PhantomData<T>);

impl<
    T: StatModifierMul<S> + Component + 'static,
    S: StatType<DataType: Add + Mul + Clone + Send + Sync + 'static> + Send + Sync + 'static,
> AutoPluginBuildHook<T> for StatModifierMulHook<S>
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_modifier_mul::<S, T>();
    }
}

/// [`bevy_auto_plugin`] build hook for [`AppExt::order_stats`].
///
/// ```rust
/// # use bevy_auto_plugin::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(AutoPlugin)]
/// # #[auto_plugin(impl_plugin_trait)]
/// # struct MyPlugin;
/// #
/// # #[derive(StatType)]
/// # struct StatBeforeMyStat;
/// #
/// #[derive(StatType)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatTypeHook)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = OrderStatAfterHook::<StatBeforeMyStat>::default())]
/// struct MyStat;
/// ```
#[derive_where(Default)]
pub struct OrderStatAfterHook<T>(PhantomData<T>);

impl<TBefore: StatType + Send + Sync + 'static, TAfter: StatType + Send + Sync + 'static>
    AutoPluginBuildHook<TAfter> for OrderStatAfterHook<TBefore>
{
    fn on_build(&self, app: &mut App) {
        app.order_stats::<TBefore, TAfter>();
    }
}

/// [`bevy_auto_plugin`] build hook for [`AppExt::order_stats`].
///
/// ```rust
/// # use bevy_auto_plugin::prelude::*;
/// # use sbepistats::*;
/// #
/// # #[derive(AutoPlugin)]
/// # #[auto_plugin(impl_plugin_trait)]
/// # struct MyPlugin;
/// #
/// # #[derive(StatType)]
/// # struct StatAfterMyStat;
/// #
/// #[derive(StatType)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = StatTypeHook)]
/// #[auto_plugin_build_hook(plugin = MyPlugin, hook = OrderStatBeforeHook::<StatAfterMyStat>::default())]
/// struct MyStat;
/// ```
#[derive_where(Default)]
pub struct OrderStatBeforeHook<T>(PhantomData<T>);

impl<TBefore: StatType + Send + Sync + 'static, TAfter: StatType + Send + Sync + 'static>
    AutoPluginBuildHook<TBefore> for OrderStatBeforeHook<TAfter>
{
    fn on_build(&self, app: &mut App) {
        app.order_stats::<TBefore, TAfter>();
    }
}

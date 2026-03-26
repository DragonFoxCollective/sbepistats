//! Compatibility hooks for `bevy_auto_plugin`.

use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

use crate::{Add, AppExt, Mul, StatModifierAdd, StatModifierMul, StatType};

/// [`bevy_auto_plugin`] build hook for [`StatType`].
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

/// [`bevy_auto_plugin`] build hook for [`StatModifierAdd`].
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
pub struct StatModifierAddHook<T>(PhantomData<T>);
impl<T> Default for StatModifierAddHook<T> {
    fn default() -> Self {
        Self(default())
    }
}

impl<
    T: StatModifierAdd<S> + Component + 'static,
    S: StatType<DataType: Add + Clone + Send + Sync + 'static> + Send + Sync + 'static,
> AutoPluginBuildHook<T> for StatModifierAddHook<S>
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_modifier_add::<S, T>();
    }
}

/// [`bevy_auto_plugin`] build hook for [`StatModifierMul`].
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
pub struct StatModifierMulHook<T>(PhantomData<T>);
impl<T> Default for StatModifierMulHook<T> {
    fn default() -> Self {
        Self(default())
    }
}

impl<
    T: StatModifierMul<S> + Component + 'static,
    S: StatType<DataType: Add + Mul + Clone + Send + Sync + 'static> + Send + Sync + 'static,
> AutoPluginBuildHook<T> for StatModifierMulHook<S>
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_modifier_mul::<S, T>();
    }
}

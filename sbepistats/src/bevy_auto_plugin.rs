use std::marker::PhantomData;

use bevy::prelude::*;
use bevy_auto_plugin::prelude::*;

use crate::{Add, AppExt, Mul, One, StatModifierAdd, StatModifierMul, StatType, Zero};

pub struct StatTypeHook;

impl<T: StatType<DataType: Clone + Send + Sync + 'static> + Send + Sync + 'static>
    AutoPluginBuildHook<T> for StatTypeHook
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_type::<T>();
    }
}

pub struct StatModifierAddHook<T>(PhantomData<T>);
impl<T> Default for StatModifierAddHook<T> {
    fn default() -> Self {
        Self(default())
    }
}

impl<
    T: StatModifierAdd<S> + Component + 'static,
    S: StatType<DataType: Zero + Add + Clone + Send + Sync + 'static> + Send + Sync + 'static,
> AutoPluginBuildHook<T> for StatModifierAddHook<S>
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_modifier_add::<S, T>();
    }
}

pub struct StatModifierMulHook<T>(PhantomData<T>);
impl<T> Default for StatModifierMulHook<T> {
    fn default() -> Self {
        Self(default())
    }
}

impl<
    T: StatModifierMul<S> + Component + 'static,
    S: StatType<DataType: Zero + One + Add + Mul + Clone + Send + Sync + 'static>
        + Send
        + Sync
        + 'static,
> AutoPluginBuildHook<T> for StatModifierMulHook<S>
{
    fn on_build(&self, app: &mut App) {
        app.add_stat_modifier_mul::<S, T>();
    }
}

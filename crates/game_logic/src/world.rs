use piccolo::{IntoValue, Table, Value};
use rogalik::{events::EventBus, math::vectors::Vector2i};
use std::collections::HashMap;

#[derive(Default)]
pub struct World {
    pub home: Vector2i,
    pub fish: HashMap<Vector2i, Fish>,
    pub players: Vec<Player>,
}
impl World {
    pub(crate) fn to_lua<'gc>(&self, ctx: piccolo::Context<'gc>) -> Value<'gc> {
        let t = Table::new(&ctx);
        // HOME
        let h = Table::new(&ctx);
        let _ = h.set(ctx, "x", self.home.x);
        let _ = h.set(ctx, "y", self.home.y);
        let _ = t.set(ctx, "home", h);
        // END HOME
        let _ = t.set(ctx, "players", vec_to_value(&self.players, ctx));
        let _ = t.set(ctx, "fish", hashmap_to_value(&self.fish, ctx));
        t.into_value(ctx)
    }
}

#[derive(Default, Clone, Copy)]
pub struct Player {
    pub v: Vector2i,
    pub loaded: Option<Fish>,
}
impl<'gc> IntoValue<'gc> for Player {
    fn into_value(self, ctx: piccolo::Context<'gc>) -> Value<'gc> {
        let t = Table::new(&ctx);
        let _ = t.set(ctx, "x", self.v.x);
        let _ = t.set(ctx, "y", self.v.y);
        let _ = t.set(ctx, "loaded", self.loaded);
        t.into_value(ctx)
    }
}

#[derive(Clone, Copy)]
pub struct Fish {
    pub life: u32,
    pub value: u32,
}
impl<'gc> IntoValue<'gc> for Fish {
    fn into_value(self, ctx: piccolo::Context<'gc>) -> Value<'gc> {
        let t = Table::new(&ctx);
        let _ = t.set(ctx, "life", self.life);
        let _ = t.set(ctx, "value", self.value);
        t.into_value(ctx)
    }
}

fn hashmap_to_value<'gc, T: Copy + IntoValue<'gc>>(
    hashmap: &HashMap<Vector2i, T>,
    ctx: piccolo::Context<'gc>,
) -> Value<'gc> {
    let t = Table::new(&ctx);
    for (i, (k, v)) in hashmap.iter().enumerate() {
        if let Value::Table(st) = v.into_value(ctx) {
            let _ = st.set(ctx, "x", k.x);
            let _ = st.set(ctx, "y", k.y);
            let _ = t.set(ctx, (i as u32 + 1).into_value(ctx), st);
        }
    }
    t.into()
}

fn vec_to_value<'gc, T: Copy + IntoValue<'gc>>(
    v: &Vec<T>,
    ctx: piccolo::Context<'gc>,
) -> Value<'gc> {
    let t = Table::new(&ctx);
    for (i, a) in v.iter().enumerate() {
        let _ = t.set(ctx, i as u32 + 1, *a);
    }
    t.into()
}

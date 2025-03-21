use piccolo::{IntoValue, Table, Value};
use rogalik::math::vectors::Vector2i;
use std::collections::{HashMap, HashSet};

#[derive(Default)]
pub struct World {
    pub fish: HashMap<Vector2i, Fish>,
    pub home: Vector2i,
    pub obstacles: HashSet<Vector2i>,
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
        let _ = t.set(ctx, "obstacles", hashset_to_value(&self.obstacles, ctx));
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

#[derive(Clone, Copy, Debug)]
pub struct Fish {
    pub id: u32,
    pub life: u32,
    pub value: u32,
}
impl<'gc> IntoValue<'gc> for Fish {
    fn into_value(self, ctx: piccolo::Context<'gc>) -> Value<'gc> {
        let t = Table::new(&ctx);
        let _ = t.set(ctx, "id", self.id);
        let _ = t.set(ctx, "life", self.life);
        let _ = t.set(ctx, "value", self.value);
        t.into_value(ctx)
    }
}

/// Expects an id field to use as a table key
fn hashmap_to_value<'gc, T: Copy + IntoValue<'gc>>(
    hashmap: &HashMap<Vector2i, T>,
    ctx: piccolo::Context<'gc>,
) -> Value<'gc> {
    let t = Table::new(&ctx);
    for (k, v) in hashmap.iter() {
        if let Value::Table(st) = v.into_value(ctx) {
            let _ = st.set(ctx, "x", k.x);
            let _ = st.set(ctx, "y", k.y);
            if let Value::Integer(i) = st.get(ctx, "id") {
                let _ = t.set(ctx, (i as u32).into_value(ctx), st);
            }
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

fn hashset_to_value<'gc>(hashset: &HashSet<Vector2i>, ctx: piccolo::Context<'gc>) -> Value<'gc> {
    let t = Table::new(&ctx);
    for (i, k) in hashset.iter().enumerate() {
        let st = Table::new(&ctx);
        let _ = st.set(ctx, "x", k.x);
        let _ = st.set(ctx, "y", k.y);
        let _ = t.set(ctx, (i as u32 + 1).into_value(ctx), st);
    }
    t.into()
}

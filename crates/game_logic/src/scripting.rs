use piccolo::{Callback, Closure, Executor, Function, IntoMultiValue, IntoValue, Lua, Value};

use crate::{LogicState, World};

pub(crate) fn init(code: String, state: &LogicState) -> anyhow::Result<Lua> {
    let mut lua = Lua::core();
    let executor = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let closure = Closure::load_with_env(ctx, None, code.as_bytes(), env)?;
        let ex = Executor::start(ctx, closure.into(), ());
        Ok(ctx.stash(ex))
    })?;
    lua.execute::<()>(&executor)?;

    insert_functions(&mut lua, state);

    Ok(lua)
}

pub(crate) fn get_player_action(lua: &mut Lua, idx: u32, world: &World) -> anyhow::Result<String> {
    let executor = lua.try_enter(|ctx| {
        let w = world.to_lua(ctx);
        let env = ctx.globals();
        let f = env.get(ctx, "update_player");
        match f {
            Value::Function(f) => {
                let ex = Executor::start(ctx, f.into(), (idx + 1, w));
                Ok(ctx.stash(ex))
            }
            v => Err(piccolo::Error::from_value(v)),
        }
    })?;
    Ok(lua.execute::<String>(&executor)?)
}

fn insert_functions(lua: &mut Lua, state: &LogicState) {
    // debug
    let tx = state
        .console
        .as_ref()
        .expect("Console not initialized!")
        .get_sender();
    lua.enter(|ctx| {
        let f = Callback::from_fn(&ctx, move |ctx, _, mut stack| {
            let v: Value = stack.consume(ctx)?;
            let _ = tx.send(display_value(&v));
            Ok(piccolo::CallbackReturn::Return)
        });
        let env = ctx.globals();
        let _ = env.set_value(&ctx, "debug".into_value(ctx), f.into());
    });
}

fn display_value(value: &Value) -> String {
    match value {
        Value::Table(t) => format!(
            "[{}]",
            t.iter()
                .map(|(k, v)| format!("{}: {}", display_value(&k), display_value(&v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
        _ => value.to_string(),
    }
}

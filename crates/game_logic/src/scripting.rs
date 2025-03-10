use piccolo::{Callback, Closure, Executor, Function, Lua, Value};

// const CODE: &str = "
// i = 0
// function update_player ()
//     i = (i + 1) % 4

//     if i == 0 then return 'down' end
//     if i == 1 then return 'left' end
//     if i == 2 then return 'up' end
//     return 'right'
// end
// ";

pub(crate) fn init(lua: &mut Lua, code: String) -> anyhow::Result<()> {
    let executor = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let closure = Closure::load_with_env(ctx, None, code.as_bytes(), env)?;
        let ex = Executor::start(ctx, closure.into(), ());
        Ok(ctx.stash(ex))
    })?;
    lua.execute::<()>(&executor)?;

    // lua.enter(|ctx| {
    //     let f = Callback::from_fn(&ctx, |ctx, _, _| {
    //         rogalik::engine::log::debug!("DEBUG");
    //         Ok(piccolo::CallbackReturn::Return)
    //     });
    //     let env = ctx.globals();
    //     let _ = env.set_value(
    //         &ctx,
    //         Value::String(piccolo::String::from_static(&ctx, "debug")),
    //         f.into(),
    //     );
    // });

    // for _ in 0..10 {
    //     let executor = lua
    //         .try_enter(|ctx| {
    //             let env = ctx.globals();
    //             let foo = env.get(ctx, "foo");
    //             if let Value::Function(f) = foo {
    //                 let ex = Executor::start(ctx, f.into(), "message");
    //                 Ok(ctx.stash(ex))
    //             } else {
    //                 panic!("")
    //             }
    //         })
    //         .unwrap();

    //     let result = lua.execute::<i32>(&executor);
    //     println!("{:?}", result);
    // }
    Ok(())
}

pub(crate) fn get_player_action(lua: &mut Lua, idx: u32) -> anyhow::Result<String> {
    let executor = lua.try_enter(|ctx| {
        let env = ctx.globals();
        let f = env.get(ctx, "update_player");
        match f {
            Value::Function(f) => {
                let ex = Executor::start(ctx, f.into(), idx);
                Ok(ctx.stash(ex))
            }
            v => Err(piccolo::Error::from_value(v)),
        }
    })?;
    Ok(lua.execute::<String>(&executor)?)
}

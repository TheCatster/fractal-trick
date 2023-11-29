use extism_pdk::*;

#[host_fn]
extern "ExtismHost" {
    fn matricks_debug(msg: &str);
    fn matricks_info(msg: &str);
    fn matricks_warn(msg: &str);
    fn matricks_error(msg: &str);
}

#[plugin_fn]
pub fn setup(_: ()) -> FnResult<()> {
    // Setup your plugin here

    Ok(())
}

#[plugin_fn]
pub fn update(_: ()) -> FnResult<Json<Option<Vec<Vec<[u8; 4]>>>>> {
    // Generate and return updates here

    Ok(Json(None))
}
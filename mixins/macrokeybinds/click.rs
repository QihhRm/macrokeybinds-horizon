// A click on button `button` of the mod's settings page.
fn macrokeybinds_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::port_macrokeybinds as m;
    if mod_id != m::ID {
        return;
    }
    match button {
        0 => {
            let enabled = game.native_setting(m::ID, "enabled") != "false";
            game.native_set_setting(m::ID, "enabled", if enabled { "false" } else { "true" });
        }
        1 => {
            let status = game.native_setting(m::ID, "local_messages") != "false";
            game.native_set_setting(m::ID, "local_messages", if status { "false" } else { "true" });
        }
        _ => {}
    }
}

// The mod's settings page: one button per option, labels pushed in order.
// Stands in for the Java config screen (EditMacroScreen, GlobalMacroScreen, MainMacroScreen, ServerMacroScreen, WheelOptionScreen, WheelScreen, WheelSelectIconScreen, WheelSelectMacroScreen).
fn macrokeybinds_settings(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::port_macrokeybinds as m;
    if mod_id != m::ID {
        return;
    }
    let enabled = game.native_setting(m::ID, "enabled") != "false";
    buttons.push(format!("Macro execution: {}", if enabled { "ON" } else { "OFF" }));
    buttons.push("Local macro status messages: ON".to_string());
}

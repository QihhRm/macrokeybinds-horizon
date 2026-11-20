# Java translated to Rust

Translated 17 of 417 methods (2 more are done another way in Horizon) and 55 of 65 static fields to Rust (2 of those call the game, through Horizon's helpers); 1 of 5 Fabric event handlers. It's in `runtime.rs`, under `pub mod java`; code that calls the game is in `game_side.rs`.

Each method is translated whole or not at all. A method that can't be (it uses Minecraft, a lambda, null...) is left out with its reason, and so is every method that calls it.

## Why methods were left out

| Cause | Methods | For example |
|---|---|---|
| Calls or uses other code that isn't translated | 348 | `EditMacroScreen.EditMacroScreen(3)`: its class isn't translated (it extends Minecraft's `Screen`) |
| Other Java this translator doesn't handle yet | 26 | `GlobalMacroScreen.init(0)`: its body couldn't be read: an anonymous class |
| Uses Minecraft classes (needs Horizon's game API) | 6 | `EditMacroScreen.getPressedModifierKeyCode(0)`: it calls Minecraft's `InputConstants`.isKeyDown |
| Uses Fabric API | 4 | `MacroEvent.onInputEvent(0)`: it uses Fabric's `ClientTickEvents`.END_CLIENT_TICK |
| Uses null | 4 | `Wheel.Wheel(0)`: a WheelMode with no value (null in Java) |
| Uses Java collections other than List (Map, Set...) | 3 | `MacroSessionManager.disconnectFromServer(0)`: it uses `UUID` (java.util) |
| Done another way in Horizon | 2 | `FabricMacroMod.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook |
| Uses files (java.nio / java.io) | 2 | `MacroMod.setup(0)`: it uses `File` (java.io) |
| Uses other libraries | 2 | `MacroUtil.isNumeric(1)`: it creates `BigInteger` (java.math) |
| Uses try/catch | 2 | `MacroUtil.migrateLegacyServerDirectory(3)`: a try/catch (InvalidPathException) |
| Calls the game where Horizon has no helper yet | 1 | `MacroActionExecutor.execute(2)`: it uses Minecraft's LocalPlayer.connection (no Horizon helper for it yet) |

## Fabric event handlers

| Handler | Hook | Result |
|---|---|---|
| com/thomas7520/macrokeybinds/event/MacroEvent.java:18 | `tick` | not translated: it calls MacroInputHandler.checkInputs, which isn't translated |
| com/thomas7520/macrokeybinds/event/MacroEvent.java:22 | `tick` | not translated: it calls MacroExecutor.tick, which isn't translated |
| com/thomas7520/macrokeybinds/event/MacroEvent.java:26 | `join` | not translated: it uses Minecraft's `ClientPacketListener` |
| com/thomas7520/macrokeybinds/event/MacroEvent.java:35 | `disconnect` | not translated: it calls MacroSessionManager.disconnectFromServer, which isn't translated |
| com/thomas7520/macrokeybinds/FabricMacroMod.java:13 | `init` | `on_init`, called from the hook, without: the call to `MacroMod.setup(..)`: it calls MacroMod.setup, which isn't translated; the call to `event.onInputEvent(..)`: it calls MacroEvent.onInputEvent, which isn't translated; the call to `event.onServerConnect(..)`: it calls MacroEvent.onServerConnect, which isn't translated; the call to `event.onServerDisconnect(..)`: it calls MacroEvent.onServerDisconnect, which isn't translated; the call to `event.onTick(..)`: it calls MacroEvent.onTick, which isn't translated |

## Every class


### `MacroEvent`

- [ ] `MacroEvent.onInputEvent(0)`: it uses Fabric's `ClientTickEvents`.END_CLIENT_TICK (com/thomas7520/macrokeybinds/event/MacroEvent.java:18)
- [ ] `MacroEvent.onTick(0)`: it uses Fabric's `ClientTickEvents`.END_CLIENT_TICK (com/thomas7520/macrokeybinds/event/MacroEvent.java:22)
- [ ] `MacroEvent.onServerConnect(0)`: it uses Fabric's `ClientPlayConnectionEvents`.JOIN (com/thomas7520/macrokeybinds/event/MacroEvent.java:26)
- [ ] `MacroEvent.onServerDisconnect(0)`: it uses Fabric's `ClientPlayConnectionEvents`.DISCONNECT (com/thomas7520/macrokeybinds/event/MacroEvent.java:35)

### `FabricMacroMod`: it implements Fabric's `ClientModInitializer`

- [ ] `FabricMacroMod.onInitializeClient(0)`: done another way in Horizon: it runs as `on_init`, from Horizon's init hook (com/thomas7520/macrokeybinds/FabricMacroMod.java:13)

### `EditMacroScreen`: it extends Minecraft's `Screen`

- [x] `EditMacroScreen.FORM_WIDTH` (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:38)
- [x] `EditMacroScreen.COLUMN_WIDTH` (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:39)
- [x] `EditMacroScreen.COLUMN_GAP` (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:40)
- [x] `EditMacroScreen.COMPACT_FIELD_WIDTH` (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:41)
- [x] `EditMacroScreen.WIDGET_HEIGHT` (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:42)
- [x] `EditMacroScreen.TITLE_COLOR` (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:43)
- [ ] `EditMacroScreen.EditMacroScreen(3)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:85)
- [ ] `EditMacroScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:98)
- [ ] `EditMacroScreen.tick(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:289)
- [ ] `EditMacroScreen.onEdited(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:293)
- [ ] `EditMacroScreen.onSecondEdited(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:301)
- [ ] `EditMacroScreen.cycleActionType(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:308)
- [ ] `EditMacroScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:322)
- [ ] `EditMacroScreen.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:353)
- [ ] `EditMacroScreen.keyPressed(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:388)
- [ ] `EditMacroScreen.keyReleased(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:436)
- [ ] `EditMacroScreen.mouseScrolled(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:450)
- [ ] `EditMacroScreen.resize(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:460)
- [ ] `EditMacroScreen.initDataMacro(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:465)
- [ ] `EditMacroScreen.updateTimingFields(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:516)
- [ ] `EditMacroScreen.updateMacroTypeTooltip(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:526)
- [ ] `EditMacroScreen.updateActionTypeTooltip(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:530)
- [ ] `EditMacroScreen.rebuildFormRows(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:536)
- [ ] `EditMacroScreen.isValidRepeatCount(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:579)
- [ ] `EditMacroScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:591)
- [ ] `EditMacroScreen.getKeyName(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:595)
- [ ] `EditMacroScreen.hasConflictKey(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:599)
- [ ] `EditMacroScreen.createButton(6)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:605)
- [ ] `EditMacroScreen.createPositioner(3)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:609)
- [ ] `EditMacroScreen.isKeyCodeModifier(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:615)
- [ ] `EditMacroScreen.getPressedModifierKeyCode(0)`: it calls Minecraft's `InputConstants`.isKeyDown (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java:627)

### `GlobalMacroScreen`: it extends Minecraft's `Screen`

- [x] `GlobalMacroScreen.STOP_ICON` (com/thomas7520/macrokeybinds/gui/GlobalMacroScreen.java:23)
- [ ] `GlobalMacroScreen.GlobalMacroScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/GlobalMacroScreen.java:29)
- [ ] `GlobalMacroScreen.init(0)`: its body couldn't be read: an anonymous class (com/thomas7520/macrokeybinds/gui/GlobalMacroScreen.java:34)
- [ ] `GlobalMacroScreen.tick(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/GlobalMacroScreen.java:117)
- [ ] `GlobalMacroScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/GlobalMacroScreen.java:143)

### `MainMacroScreen`: it extends Minecraft's `Screen`

- [ ] `MainMacroScreen.MainMacroScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:27)
- [ ] `MainMacroScreen.MainMacroScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:31)
- [ ] `MainMacroScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:36)
- [ ] `MainMacroScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:65)
- [ ] `MainMacroScreen.createButton(6)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:72)
- [ ] `MainMacroScreen.createUrlButton(6)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:76)
- [ ] `MainMacroScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:80)
- [ ] `MainMacroScreen.createPositioner(3)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java:88)

### `EditMacroFormList`: it extends Minecraft's `ContainerObjectSelectionList`

- [x] `EditMacroFormList.FORM_WIDTH` (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:17)
- [x] `EditMacroFormList.ROW_HEIGHT` (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:18)
- [x] `EditMacroFormList.LABEL_COLOR` (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:19)
- [ ] `EditMacroFormList.EditMacroFormList(2)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:22)
- [ ] `EditMacroFormList.clearRows(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:28)
- [ ] `EditMacroFormList.addRow(1)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:32)
- [ ] `EditMacroFormList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:36)
- [ ] `EditMacroFormList.scrollBarX(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:40)
- [ ] `EditMacroFormList.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:44)

### `EditMacroFormList.Entry`: it extends `Entry`

- [ ] `EditMacroFormList.Entry.Entry(2)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:60)
- [ ] `EditMacroFormList.Entry.extractContent(5)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:66)
- [ ] `EditMacroFormList.Entry.children(0)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:78)
- [ ] `EditMacroFormList.Entry.narratables(0)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:82)

### `EditMacroFormList.Field`: it uses Minecraft's `Component`

- [ ] `EditMacroFormList.Field.Field(3)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:87)
- [ ] `EditMacroFormList.Field.label(0)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:87)
- [ ] `EditMacroFormList.Field.widget(0)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:87)
- [ ] `EditMacroFormList.Field.xOffset(0)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java:87)

### `MacroList`: it extends Minecraft's `ContainerObjectSelectionList`

- [x] `MacroList.EDIT_ICON` (com/thomas7520/macrokeybinds/gui/other/MacroList.java:42)
- [x] `MacroList.DELETE_ICON` (com/thomas7520/macrokeybinds/gui/other/MacroList.java:43)
- [ ] `MacroList.MacroList(4)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:48)
- [ ] `MacroList.update(2)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:57)
- [ ] `MacroList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:79)
- [ ] `MacroList.updateList(1)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:83)
- [ ] `MacroList.scrollBarX(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:91)
- [ ] `MacroList.createPositioner(3)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:95)

### `MacroList.Entry`: it extends `Entry`

- [ ] `MacroList.Entry.update(0)`: it has no body (com/thomas7520/macrokeybinds/gui/other/MacroList.java:102)

### `MacroList.KeyBindingEntry`: it extends `Entry`

- [ ] `MacroList.KeyBindingEntry.KeyBindingEntry(3)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:111)
- [ ] `MacroList.KeyBindingEntry.extractContent(5)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:157)
- [ ] `MacroList.KeyBindingEntry.children(0)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:227)
- [ ] `MacroList.KeyBindingEntry.narratables(0)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:231)
- [ ] `MacroList.KeyBindingEntry.update(0)`: its class isn't translated (it extends `Entry`) (com/thomas7520/macrokeybinds/gui/other/MacroList.java:235)

### `ServerMacroScreen`: it extends Minecraft's `Screen`

- [x] `ServerMacroScreen.STOP_ICON` (com/thomas7520/macrokeybinds/gui/ServerMacroScreen.java:23)
- [ ] `ServerMacroScreen.ServerMacroScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/ServerMacroScreen.java:29)
- [ ] `ServerMacroScreen.init(0)`: its body couldn't be read: an anonymous class (com/thomas7520/macrokeybinds/gui/ServerMacroScreen.java:34)
- [ ] `ServerMacroScreen.tick(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/ServerMacroScreen.java:117)
- [ ] `ServerMacroScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/ServerMacroScreen.java:143)

### `WheelOptionScreen`: it extends Minecraft's `Screen`

- [x] `WheelOptionScreen.DELETE_ICON` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:27)
- [x] `WheelOptionScreen.MAX_PREVIEW_RADIUS` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:28)
- [x] `WheelOptionScreen.CONTROLS_DISTANCE_FROM_CIRCLE` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:29)
- [x] `WheelOptionScreen.DELETE_BUTTON_SIZE` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:30)
- [x] `WheelOptionScreen.ICON_BUTTON_SIZE` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:31)
- [x] `WheelOptionScreen.ADD_BUTTON_SIZE` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:32)
- [ ] `WheelOptionScreen.WheelOptionScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:37)
- [ ] `WheelOptionScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:44)
- [ ] `WheelOptionScreen.addSlotControls(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:69)
- [ ] `WheelOptionScreen.openMacroSelector(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:105)
- [ ] `WheelOptionScreen.removeMacro(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:112)
- [ ] `WheelOptionScreen.getWheelSlot(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:118)
- [ ] `WheelOptionScreen.setWheelSlot(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:122)
- [ ] `WheelOptionScreen.changeMode(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:130)
- [ ] `WheelOptionScreen.modeText(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:136)
- [ ] `WheelOptionScreen.changeIcon(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:141)
- [ ] `WheelOptionScreen.saveWheel(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:148)
- [ ] `WheelOptionScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:158)
- [ ] `WheelOptionScreen.iconActionText(1)`: it uses WheelSlot, which isn't translated (it uses `UUID` (java.util)) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:166)
- [ ] `WheelOptionScreen.getIcon(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:170)

### `WheelOptionScreen.SlotIconButton`: it extends Minecraft's `AbstractButton`

- [ ] `WheelOptionScreen.SlotIconButton.SlotIconButton(5)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:179)
- [ ] `WheelOptionScreen.SlotIconButton.onPress(1)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:185)
- [ ] `WheelOptionScreen.SlotIconButton.extractContents(4)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:189)
- [ ] `WheelOptionScreen.SlotIconButton.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java:201)

### `WheelScreen`: it extends Minecraft's `Screen`

- [x] `WheelScreen.HOLD_HOVER_DEAD_ZONE` (com/thomas7520/macrokeybinds/gui/wheel/WheelScreen.java:15)
- [ ] `WheelScreen.WheelScreen(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelScreen.java:19)
- [ ] `WheelScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelScreen.java:24)
- [ ] `WheelScreen.tick(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelScreen.java:44)
- [ ] `WheelScreen.confirmHoveredSlot(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelScreen.java:51)
- [ ] `WheelScreen.isPauseScreen(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelScreen.java:61)

### `WheelSelectIconScreen`: it extends Minecraft's `Screen`

- [x] `WheelSelectIconScreen.ICON_SIZE` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:29)
- [x] `WheelSelectIconScreen.ICON_GAP` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:30)
- [ ] `WheelSelectIconScreen.WheelSelectIconScreen(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:37)
- [ ] `WheelSelectIconScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:43)
- [ ] `WheelSelectIconScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:57)
- [ ] `WheelSelectIconScreen.selectIcon(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:68)
- [ ] `WheelSelectIconScreen.confirmSelection(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:73)
- [ ] `WheelSelectIconScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:78)

### `WheelSelectIconScreen.IconButton`: it extends Minecraft's `AbstractWidget`

- [ ] `WheelSelectIconScreen.IconButton.IconButton(2)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:86)
- [ ] `WheelSelectIconScreen.IconButton.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:92)
- [ ] `WheelSelectIconScreen.IconButton.onClick(2)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:104)
- [ ] `WheelSelectIconScreen.IconButton.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:108)

### `WheelSelectIconScreen.IconGrid`: it extends Minecraft's `ContainerObjectSelectionList`

- [ ] `WheelSelectIconScreen.IconGrid.IconGrid(4)`: its body couldn't be read: an inner class creation (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:116)
- [ ] `WheelSelectIconScreen.IconGrid.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:133)

### `WheelSelectIconScreen.IconRow`: it extends Minecraft's `Entry`

- [ ] `WheelSelectIconScreen.IconRow.IconRow(1)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:141)
- [ ] `WheelSelectIconScreen.IconRow.extractContent(5)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:145)
- [ ] `WheelSelectIconScreen.IconRow.children(0)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:153)
- [ ] `WheelSelectIconScreen.IconRow.narratables(0)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java:157)

### `WheelSelectMacroScreen`: it extends Minecraft's `Screen`

- [ ] `WheelSelectMacroScreen.WheelSelectMacroScreen(2)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:30)
- [ ] `WheelSelectMacroScreen.init(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:36)
- [ ] `WheelSelectMacroScreen.isMacroAlreadyInWheel(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:57)
- [ ] `WheelSelectMacroScreen.selectMacro(1)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:63)
- [ ] `WheelSelectMacroScreen.onClose(0)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:68)
- [ ] `WheelSelectMacroScreen.extractRenderState(4)`: its class isn't translated (it extends Minecraft's `Screen`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:72)

### `WheelSelectMacroScreen.MacroEntry`: it extends Minecraft's `Entry`

- [ ] `WheelSelectMacroScreen.MacroEntry.MacroEntry(1)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:82)
- [ ] `WheelSelectMacroScreen.MacroEntry.extractContent(5)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:88)
- [ ] `WheelSelectMacroScreen.MacroEntry.children(0)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:94)
- [ ] `WheelSelectMacroScreen.MacroEntry.narratables(0)`: its class isn't translated (it extends Minecraft's `Entry`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:98)

### `WheelSelectMacroScreen.MacroSelectionList`: it extends Minecraft's `ContainerObjectSelectionList`

- [ ] `WheelSelectMacroScreen.MacroSelectionList.MacroSelectionList(5)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:106)
- [ ] `WheelSelectMacroScreen.MacroSelectionList.update(1)`: its body couldn't be read: an inner class creation (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:112)
- [ ] `WheelSelectMacroScreen.MacroSelectionList.getRowWidth(0)`: its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`) (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java:122)

### `WheelWidget`: it extends Minecraft's `AbstractContainerWidget`

- [x] `WheelWidget.SLOTS_MAX` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:27)
- [x] `WheelWidget.SLOT_SIZE` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:28)
- [x] `WheelWidget.THICKNESS_LINE` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:29)
- [x] `WheelWidget.FIRST_SLOT_START_ANGLE` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:30)
- [x] `WheelWidget.SLOT_COLOR` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:31)
- [x] `WheelWidget.HOVERED_SLOT_COLOR` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:32)
- [x] `WheelWidget.DISABLED_OVERLAY_COLOR` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:33)
- [x] `WheelWidget.LINE_COLOR` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:34)
- [x] `WheelWidget.FULL_CIRCLE` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:35)
- [x] `WheelWidget.RENDER_SCALE` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:36)
- [x] `WheelWidget.PIXEL_CENTER` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:37)
- [x] `WheelWidget.MAX_NAME_WIDTH` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:38)
- [x] `WheelWidget.NAME_PADDING` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:39)
- [ ] `WheelWidget.WheelWidget(9)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:49)
- [x] `WheelWidget.getSlotMiddleAngle(1)` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:76)
- [ ] `WheelWidget.children(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:80)
- [ ] `WheelWidget.contentHeight(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:84)
- [ ] `WheelWidget.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:88)
- [ ] `WheelWidget.renderDisabledOverlay(5)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:118)
- [ ] `WheelWidget.renderSlotContents(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:137)
- [ ] `WheelWidget.renderMacroState(6)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:181)
- [ ] `WheelWidget.findMacro(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:203)
- [ ] `WheelWidget.isDisabled(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:207)
- [ ] `WheelWidget.findIcon(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:212)
- [ ] `WheelWidget.getMaxNameWidth(6)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:222)
- [ ] `WheelWidget.isInsideSlot(5)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:241)
- [ ] `WheelWidget.trimName(3)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:248)
- [ ] `WheelWidget.fillArc(6)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:266)
- [ ] `WheelWidget.drawSeparator(4)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:298)
- [x] `WheelWidget.containsAngle(3)` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:310)
- [x] `WheelWidget.normalizeAngle(1)` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:319)
- [ ] `WheelWidget.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:324)
- [ ] `WheelWidget.mouseMoved(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:327)
- [ ] `WheelWidget.updateHoveredSlot(2)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:335)
- [ ] `WheelWidget.getHoveredSlot(0)`: its class isn't translated (it extends Minecraft's `AbstractContainerWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:352)

### `WheelWidget.Slot`: it extends Minecraft's `AbstractWidget`

- [ ] `WheelWidget.Slot.Slot(4)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:362)
- [ ] `WheelWidget.Slot.isMouseOver(2)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:370)
- [ ] `WheelWidget.Slot.onClick(2)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:379)
- [ ] `WheelWidget.Slot.mouseClicked(2)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:385)
- [ ] `WheelWidget.Slot.extractWidgetRenderState(4)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:390)
- [ ] `WheelWidget.Slot.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:393)
- [ ] `WheelWidget.Slot.getWheelSlot(0)`: its class isn't translated (it extends Minecraft's `AbstractWidget`) (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java:396)

### `MacroMod`

- [x] `MacroMod.MODID` (com/thomas7520/macrokeybinds/MacroMod.java:13)
- [x] `MacroMod.LOGGER` (com/thomas7520/macrokeybinds/MacroMod.java:14)
- [ ] `MacroMod.setup(0)`: it uses `File` (java.io) (com/thomas7520/macrokeybinds/MacroMod.java:16)

### `ModMenuIntegration`: it implements `ModMenuApi` (com.terraformersmc.modmenu.api)

- [ ] `ModMenuIntegration.getModConfigScreenFactory(0)`: done another way in Horizon: the mod's page in Options > Mods (its settings buttons) (com/thomas7520/macrokeybinds/modmenu/ModMenuIntegration.java:8)

### `AlternateMacro`: it implements `IMacro`

- [ ] `AlternateMacro.AlternateMacro(11)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:23)
- [ ] `AlternateMacro.getUUID(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:49)
- [ ] `AlternateMacro.getName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:54)
- [ ] `AlternateMacro.getCreatedTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:59)
- [ ] `AlternateMacro.getType(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:64)
- [ ] `AlternateMacro.getActionText(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:69)
- [ ] `AlternateMacro.getSecondActionText(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:74)
- [ ] `AlternateMacro.getAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:78)
- [ ] `AlternateMacro.getSecondAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:83)
- [ ] `AlternateMacro.doAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:87)
- [ ] `AlternateMacro.start(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:101)
- [ ] `AlternateMacro.isStart(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:106)
- [ ] `AlternateMacro.isSecondActionNext(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:110)
- [ ] `AlternateMacro.reset(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:114)
- [ ] `AlternateMacro.getKey(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:120)
- [ ] `AlternateMacro.getKeyName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:125)
- [ ] `AlternateMacro.isEnable(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:130)
- [ ] `AlternateMacro.setEnable(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:135)
- [ ] `AlternateMacro.setKey(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:143)
- [ ] `AlternateMacro.getModifier(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:148)
- [ ] `AlternateMacro.setModifier(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java:153)

### `CountedRepeatMacro`: it implements `IMacro`

- [ ] `CountedRepeatMacro.CountedRepeatMacro(11)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:23)
- [ ] `CountedRepeatMacro.getUUID(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:49)
- [ ] `CountedRepeatMacro.getName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:54)
- [ ] `CountedRepeatMacro.getCreatedTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:59)
- [ ] `CountedRepeatMacro.getType(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:64)
- [ ] `CountedRepeatMacro.getActionText(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:69)
- [ ] `CountedRepeatMacro.getAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:74)
- [ ] `CountedRepeatMacro.getKey(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:79)
- [ ] `CountedRepeatMacro.getKeyName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:84)
- [ ] `CountedRepeatMacro.isEnable(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:89)
- [ ] `CountedRepeatMacro.setEnable(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:94)
- [ ] `CountedRepeatMacro.setKey(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:99)
- [ ] `CountedRepeatMacro.getModifier(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:104)
- [ ] `CountedRepeatMacro.setModifier(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:109)
- [ ] `CountedRepeatMacro.doAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:114)
- [ ] `CountedRepeatMacro.start(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:129)
- [ ] `CountedRepeatMacro.cancel(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:137)
- [ ] `CountedRepeatMacro.isRunning(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:143)
- [ ] `CountedRepeatMacro.getIntervalTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:147)
- [ ] `CountedRepeatMacro.getRepeatCount(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java:151)

### `DelayedMacro`: it implements `IMacro`

- [ ] `DelayedMacro.DelayedMacro(10)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:21)
- [ ] `DelayedMacro.getUUID(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:45)
- [ ] `DelayedMacro.getName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:50)
- [ ] `DelayedMacro.getCreatedTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:55)
- [ ] `DelayedMacro.getType(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:60)
- [ ] `DelayedMacro.getActionText(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:65)
- [ ] `DelayedMacro.getKey(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:70)
- [ ] `DelayedMacro.getKeyName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:75)
- [ ] `DelayedMacro.getDelayedTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:80)
- [ ] `DelayedMacro.getStartTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:84)
- [ ] `DelayedMacro.setStartTime(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:88)
- [ ] `DelayedMacro.isStart(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:92)
- [ ] `DelayedMacro.setStart(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:96)
- [ ] `DelayedMacro.isEnable(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:100)
- [ ] `DelayedMacro.setEnable(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:105)
- [ ] `DelayedMacro.setKey(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:110)
- [ ] `DelayedMacro.getModifier(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:115)
- [ ] `DelayedMacro.setModifier(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:120)
- [ ] `DelayedMacro.getAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:125)
- [ ] `DelayedMacro.doAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java:130)

### `IMacro`: it's an interface

- [ ] `IMacro.getUUID(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:6)
- [ ] `IMacro.getName(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:8)
- [ ] `IMacro.getCreatedTime(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:10)
- [ ] `IMacro.getType(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:12)
- [ ] `IMacro.getActionText(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:14)
- [ ] `IMacro.getAction(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:16)
- [ ] `IMacro.doAction(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:18)
- [ ] `IMacro.getKey(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:20)
- [ ] `IMacro.getKeyName(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:22)
- [ ] `IMacro.isEnable(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:24)
- [ ] `IMacro.setEnable(1)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:26)
- [ ] `IMacro.setKey(1)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:28)
- [ ] `IMacro.getModifier(0)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:30)
- [ ] `IMacro.setModifier(1)`: it has no body (com/thomas7520/macrokeybinds/object/macro/IMacro.java:32)

### `KeyAction`


### `MacroModifier`


### `MacroType`


### `RepeatMacro`: it implements `IMacro`

- [ ] `RepeatMacro.RepeatMacro(10)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:21)
- [ ] `RepeatMacro.getUUID(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:45)
- [ ] `RepeatMacro.getName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:50)
- [ ] `RepeatMacro.getCreatedTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:55)
- [ ] `RepeatMacro.getActionText(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:60)
- [ ] `RepeatMacro.getKey(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:65)
- [ ] `RepeatMacro.getKeyName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:70)
- [ ] `RepeatMacro.isEnable(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:75)
- [ ] `RepeatMacro.setEnable(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:80)
- [ ] `RepeatMacro.setKey(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:85)
- [ ] `RepeatMacro.getModifier(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:90)
- [ ] `RepeatMacro.setModifier(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:95)
- [ ] `RepeatMacro.getType(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:100)
- [ ] `RepeatMacro.getAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:105)
- [ ] `RepeatMacro.doAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:110)
- [ ] `RepeatMacro.getCooldownTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:118)
- [ ] `RepeatMacro.isRepeat(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:122)
- [ ] `RepeatMacro.setRepeat(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java:126)

### `SimpleMacro`: it implements `IMacro`

- [ ] `SimpleMacro.SimpleMacro(9)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:20)
- [ ] `SimpleMacro.getUUID(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:34)
- [ ] `SimpleMacro.getName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:39)
- [ ] `SimpleMacro.getCreatedTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:44)
- [ ] `SimpleMacro.getActionText(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:49)
- [ ] `SimpleMacro.getKey(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:54)
- [ ] `SimpleMacro.getKeyName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:59)
- [ ] `SimpleMacro.isEnable(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:64)
- [ ] `SimpleMacro.setEnable(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:69)
- [ ] `SimpleMacro.setKey(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:74)
- [ ] `SimpleMacro.getModifier(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:79)
- [ ] `SimpleMacro.setModifier(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:84)
- [ ] `SimpleMacro.getAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:89)
- [ ] `SimpleMacro.getType(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:94)
- [ ] `SimpleMacro.getStartTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:99)
- [ ] `SimpleMacro.setStartTime(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:103)
- [ ] `SimpleMacro.isStart(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:107)
- [ ] `SimpleMacro.setStart(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:111)
- [ ] `SimpleMacro.doAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java:115)

### `ToggleMacro`: it implements `IMacro`

- [ ] `ToggleMacro.ToggleMacro(10)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:21)
- [ ] `ToggleMacro.getUUID(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:45)
- [ ] `ToggleMacro.getName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:50)
- [ ] `ToggleMacro.getCreatedTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:55)
- [ ] `ToggleMacro.getActionText(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:60)
- [ ] `ToggleMacro.getKey(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:65)
- [ ] `ToggleMacro.getKeyName(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:70)
- [ ] `ToggleMacro.getAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:75)
- [ ] `ToggleMacro.isEnable(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:80)
- [ ] `ToggleMacro.setEnable(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:85)
- [ ] `ToggleMacro.setKey(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:90)
- [ ] `ToggleMacro.getModifier(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:95)
- [ ] `ToggleMacro.setModifier(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:100)
- [ ] `ToggleMacro.getType(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:105)
- [ ] `ToggleMacro.doAction(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:110)
- [ ] `ToggleMacro.getCooldownTime(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:118)
- [ ] `ToggleMacro.isToggled(0)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:122)
- [ ] `ToggleMacro.setToggled(1)`: its class isn't translated (it implements `IMacro`) (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java:126)

### `Wheel`: it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))

- [x] `Wheel.MAX_MACROS` (com/thomas7520/macrokeybinds/object/wheel/Wheel.java:8)
- [ ] `Wheel.Wheel(0)`: a WheelMode with no value (null in Java) (com/thomas7520/macrokeybinds/object/wheel/Wheel.java:12)
- [ ] `Wheel.Wheel(2)`: its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/object/wheel/Wheel.java:17)
- [ ] `Wheel.getMode(0)`: its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/object/wheel/Wheel.java:22)
- [ ] `Wheel.setMode(1)`: its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/object/wheel/Wheel.java:26)
- [ ] `Wheel.getMacros(0)`: its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/object/wheel/Wheel.java:30)
- [ ] `Wheel.setMacros(1)`: its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/object/wheel/Wheel.java:34)

### `WheelMode`


### `WheelSlot`: it uses `UUID` (java.util)

- [ ] `WheelSlot.WheelSlot(2)`: its class isn't translated (it uses `UUID` (java.util)) (com/thomas7520/macrokeybinds/object/wheel/WheelSlot.java:10)
- [ ] `WheelSlot.getMacroId(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/thomas7520/macrokeybinds/object/wheel/WheelSlot.java:15)
- [ ] `WheelSlot.setMacroId(1)`: its class isn't translated (it uses `UUID` (java.util)) (com/thomas7520/macrokeybinds/object/wheel/WheelSlot.java:19)
- [ ] `WheelSlot.getIconId(0)`: its class isn't translated (it uses `UUID` (java.util)) (com/thomas7520/macrokeybinds/object/wheel/WheelSlot.java:23)
- [ ] `WheelSlot.setIconId(1)`: its class isn't translated (it uses `UUID` (java.util)) (com/thomas7520/macrokeybinds/object/wheel/WheelSlot.java:27)

### `FabricPlatformHelper`: it implements `IPlatformHelper` (com.thomas7520.macrokeybinds.platform.services)

- [ ] `FabricPlatformHelper.getConfigDirectory(0)`: its class isn't translated (it implements `IPlatformHelper` (com.thomas7520.macrokeybinds.platform.services)) (com/thomas7520/macrokeybinds/platform/FabricPlatformHelper.java:8)

### `Services`

- [ ] `Services.PLATFORM`: it uses IPlatformHelper, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/platform/Services.java:7)
- [ ] `Services.load(1)`: it's generic (com/thomas7520/macrokeybinds/platform/Services.java:9)

### `IPlatformHelper`: it's an interface

- [ ] `IPlatformHelper.getConfigDirectory(0)`: it has no body (com/thomas7520/macrokeybinds/platform/services/IPlatformHelper.java:6)

### `MacroActionExecutor`

- [x] `MacroActionExecutor.MacroActionExecutor(0)` (com/thomas7520/macrokeybinds/util/MacroActionExecutor.java:9)
- [ ] `MacroActionExecutor.execute(2)`: it uses Minecraft's LocalPlayer.connection (no Horizon helper for it yet) (com/thomas7520/macrokeybinds/util/MacroActionExecutor.java:12)

### `MacroExecutor`

- [ ] `MacroExecutor.trigger(1)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroExecutor.java:13)
- [ ] `MacroExecutor.tick(0)`: it calls MacroUtil.getAllMacros, which isn't translated (com/thomas7520/macrokeybinds/util/MacroExecutor.java:43)

### `MacroFlow`

- [x] `MacroFlow.CURRENT_VERSION` (com/thomas7520/macrokeybinds/util/MacroFlow.java:24)
- [ ] `MacroFlow.getMacroFromFile(1)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroFlow.java:26)
- [ ] `MacroFlow.validateMacro(1)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroFlow.java:58)
- [x] `MacroFlow.migrateLegacyMacroTo1_4_0(1)` (com/thomas7520/macrokeybinds/util/MacroFlow.java:71)
- [ ] `MacroFlow.migrateInputToSdl(1)`: it calls MacroInputHandler.migrateLegacyKey, which isn't translated (com/thomas7520/macrokeybinds/util/MacroFlow.java:89)
- [ ] `MacroFlow.writeMacro(2)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroFlow.java:99)
- [ ] `MacroFlow.writeMacroFile(2)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroFlow.java:108)

### `MacroInputHandler`

- [x] `MacroInputHandler.KEYBOARD_KEY_OFFSET` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:23)
- [x] `MacroInputHandler.MOUSE_KEY_OFFSET` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:24)
- [ ] `MacroInputHandler.checkOpenGui(0)`: `null` (Rust has no null: this needs an Option) (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:26)
- [x] `MacroInputHandler.canReceiveInput(0)` in game_side.rs (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:36)
- [ ] `MacroInputHandler.isWheelBindingDown(0)`: `null` (Rust has no null: this needs an Option) (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:41)
- [ ] `MacroInputHandler.checkInputs(1)`: it calls MacroInputHandler.checkOpenGui, which isn't translated (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:54)
- [ ] `MacroInputHandler.handleKeyAction(3)`: it calls MacroInputHandler.checkOpenGui, which isn't translated (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:72)
- [ ] `MacroInputHandler.handleMouseAction(2)`: it calls MacroInputHandler.checkOpenGui, which isn't translated (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:79)
- [ ] `MacroInputHandler.handleInput(4)`: its body couldn't be read: a labelled statement (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:86)
- [ ] `MacroInputHandler.getEnabledMacroKeys(0)`: it calls MacroUtil.getAllMacros, which isn't translated (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:158)
- [x] `MacroInputHandler.modifierFromMask(1)` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:170)
- [ ] `MacroInputHandler.getPressedModifier(0)`: it calls MacroInputHandler.isPressed, which isn't translated (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:180)
- [ ] `MacroInputHandler.isPressed(2)`: it calls Minecraft's `InputConstants`.isKeyDown (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:190)
- [ ] `MacroInputHandler.isStoredKeyDown(1)`: it calls Minecraft's `InputConstants`.isKeyDown (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:194)
- [ ] `MacroInputHandler.isMouseButtonDown(1)`: `null` (Rust has no null: this needs an Option) (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:202)
- [x] `MacroInputHandler.isKeyboardKey(1)` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:206)
- [x] `MacroInputHandler.isMouseKey(1)` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:210)
- [x] `MacroInputHandler.encodeKeyboardKey(1)` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:214)
- [x] `MacroInputHandler.encodeMouseButton(1)` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:218)
- [ ] `MacroInputHandler.migrateLegacyKey(1)`: it uses Minecraft's `InputConstants`.UNKNOWN (com/thomas7520/macrokeybinds/util/MacroInputHandler.java:222)

### `MacroSessionManager`

- [ ] `MacroSessionManager.connectToServer(1)`: it calls MacroUtil.initServerMacros, which isn't translated (com/thomas7520/macrokeybinds/util/MacroSessionManager.java:11)
- [ ] `MacroSessionManager.disconnectFromServer(0)`: it uses `UUID` (java.util) (com/thomas7520/macrokeybinds/util/MacroSessionManager.java:17)
- [ ] `MacroSessionManager.resetRuntimeState(0)`: it calls MacroUtil.getAllMacros, which isn't translated (com/thomas7520/macrokeybinds/util/MacroSessionManager.java:23)

### `MacroUtil`

- [ ] `MacroUtil.keybinds`: it uses `UUID` (java.util) (com/thomas7520/macrokeybinds/util/MacroUtil.java:23)
- [ ] `MacroUtil.serverKeybinds`: it uses `UUID` (java.util) (com/thomas7520/macrokeybinds/util/MacroUtil.java:24)
- [ ] `MacroUtil.wheel`: it uses Wheel, which isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/util/MacroUtil.java:25)
- [x] `MacroUtil.serverIP` (com/thomas7520/macrokeybinds/util/MacroUtil.java:26)
- [ ] `MacroUtil.guiBinding`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/thomas7520/macrokeybinds/util/MacroUtil.java:27)
- [ ] `MacroUtil.wheelBinding`: done another way in Horizon: the mod's keys in Options > Mods, read through Horizon's key hook (com/thomas7520/macrokeybinds/util/MacroUtil.java:28)
- [ ] `MacroUtil.initMacroDirectories(0)`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (com/thomas7520/macrokeybinds/util/MacroUtil.java:30)
- [ ] `MacroUtil.initServerMacros(1)`: it uses `File` (java.io) (com/thomas7520/macrokeybinds/util/MacroUtil.java:40)
- [ ] `MacroUtil.getGlobalKeybindsMap(0)`: it uses `UUID` (java.util) (com/thomas7520/macrokeybinds/util/MacroUtil.java:73)
- [ ] `MacroUtil.getServerKeybinds(0)`: it uses `UUID` (java.util) (com/thomas7520/macrokeybinds/util/MacroUtil.java:77)
- [ ] `MacroUtil.getWheel(0)`: it uses Wheel, which isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/util/MacroUtil.java:81)
- [ ] `MacroUtil.setWheel(1)`: it uses Wheel, which isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/util/MacroUtil.java:85)
- [ ] `MacroUtil.getAllMacros(0)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroUtil.java:89)
- [ ] `MacroUtil.getMacro(1)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroUtil.java:95)
- [x] `MacroUtil.getServerIP(0)` (com/thomas7520/macrokeybinds/util/MacroUtil.java:100)
- [ ] `MacroUtil.getGlobalMacroDirectory(0)`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (com/thomas7520/macrokeybinds/util/MacroUtil.java:104)
- [ ] `MacroUtil.getServerMacrosDirectory(0)`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (com/thomas7520/macrokeybinds/util/MacroUtil.java:108)
- [ ] `MacroUtil.getServerMacroDirectory(0)`: String.getBytes with 1 values (com/thomas7520/macrokeybinds/util/MacroUtil.java:112)
- [x] `MacroUtil.migrateLegacyMacroDirectory(2)` (com/thomas7520/macrokeybinds/util/MacroUtil.java:127)
- [ ] `MacroUtil.migrateLegacyServerDirectory(3)`: a try/catch (InvalidPathException) (com/thomas7520/macrokeybinds/util/MacroUtil.java:134)
- [x] `MacroUtil.setServerIP(1)` (com/thomas7520/macrokeybinds/util/MacroUtil.java:148)
- [ ] `MacroUtil.isNumeric(1)`: it creates `BigInteger` (java.math) (com/thomas7520/macrokeybinds/util/MacroUtil.java:152)
- [ ] `MacroUtil.isCombinationAssigned(1)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroUtil.java:168)
- [ ] `MacroUtil.isCombinationAssigned(3)`: it uses IMacro, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/MacroUtil.java:172)
- [ ] `MacroUtil.isCombinationAssigned(2)`: it calls MacroUtil.getServerKeybinds, which isn't translated (com/thomas7520/macrokeybinds/util/MacroUtil.java:193)

### `WheelFlow`

- [x] `WheelFlow.GSON` (com/thomas7520/macrokeybinds/util/WheelFlow.java:16)
- [x] `WheelFlow.WheelFlow(0)` (com/thomas7520/macrokeybinds/util/WheelFlow.java:18)
- [ ] `WheelFlow.getWheel(0)`: it uses Wheel, which isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))) (com/thomas7520/macrokeybinds/util/WheelFlow.java:21)
- [ ] `WheelFlow.writeWheel(1)`: a try-with-resources without a catch (its errors go up) (com/thomas7520/macrokeybinds/util/WheelFlow.java:37)
- [ ] `WheelFlow.getWheelFile(0)`: it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)) (com/thomas7520/macrokeybinds/util/WheelFlow.java:46)

### `ButtonImageWidget`: it extends Minecraft's `AbstractButton`

- [x] `ButtonImageWidget.DEFAULT_WIDTH_SMALL` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:17)
- [x] `ButtonImageWidget.DEFAULT_WIDTH` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:18)
- [x] `ButtonImageWidget.DEFAULT_HEIGHT` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:19)
- [x] `ButtonImageWidget.field_46856` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:20)
- [ ] `ButtonImageWidget.DEFAULT_NARRATION_SUPPLIER`: it uses NarrationSupplier, which isn't translated (it's an interface) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:21)
- [ ] `ButtonImageWidget.builder(2)`: it uses Builder, which isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:26)
- [ ] `ButtonImageWidget.ButtonImageWidget(8)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:30)
- [ ] `ButtonImageWidget.createNarrationMessage(0)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:46)
- [ ] `ButtonImageWidget.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:50)
- [ ] `ButtonImageWidget.onPress(1)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:54)
- [ ] `ButtonImageWidget.extractContents(4)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:58)

### `ButtonImageWidget.Builder`: it uses Minecraft's `Component`

- [ ] `ButtonImageWidget.Builder.Builder(2)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:94)
- [ ] `ButtonImageWidget.Builder.position(2)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:99)
- [ ] `ButtonImageWidget.Builder.width(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:105)
- [ ] `ButtonImageWidget.Builder.size(2)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:110)
- [ ] `ButtonImageWidget.Builder.dimensions(4)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:116)
- [ ] `ButtonImageWidget.Builder.icon(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:120)
- [ ] `ButtonImageWidget.Builder.tooltip(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:125)
- [ ] `ButtonImageWidget.Builder.narrationSupplier(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:130)
- [ ] `ButtonImageWidget.Builder.build(0)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:135)

### `ButtonImageWidget.NarrationSupplier`: it's an interface

- [ ] `ButtonImageWidget.NarrationSupplier.createNarrationMessage(1)`: it has no body (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:145)

### `ButtonImageWidget.PressAction`: it's an interface

- [ ] `ButtonImageWidget.PressAction.onPress(1)`: it has no body (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java:149)

### `CheckboxEdited`: it extends Minecraft's `AbstractButton`

- [x] `CheckboxEdited.SELECTED_HIGHLIGHTED_TEXTURE` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:19)
- [x] `CheckboxEdited.SELECTED_TEXTURE` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:20)
- [x] `CheckboxEdited.HIGHLIGHTED_TEXTURE` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:21)
- [x] `CheckboxEdited.TEXTURE` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:22)
- [x] `CheckboxEdited.TEXT_COLOR` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:23)
- [x] `CheckboxEdited.field_47105` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:24)
- [x] `CheckboxEdited.field_47106` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:25)
- [ ] `CheckboxEdited.CheckboxEdited(6)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:29)
- [ ] `CheckboxEdited.builder(2)`: it uses Builder, which isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:35)
- [x] `CheckboxEdited.getSize(1)` in game_side.rs (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:39)
- [ ] `CheckboxEdited.isChecked(0)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:43)
- [ ] `CheckboxEdited.updateWidgetNarration(1)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:47)
- [ ] `CheckboxEdited.onPress(1)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:58)
- [ ] `CheckboxEdited.extractContents(4)`: its class isn't translated (it extends Minecraft's `AbstractButton`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:63)

### `CheckboxEdited.Builder`: it uses Minecraft's `Component`

- [ ] `CheckboxEdited.Builder.Builder(2)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:88)
- [ ] `CheckboxEdited.Builder.pos(2)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:93)
- [ ] `CheckboxEdited.Builder.callback(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:99)
- [ ] `CheckboxEdited.Builder.checked(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:104)
- [ ] `CheckboxEdited.Builder.option(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:110)
- [ ] `CheckboxEdited.Builder.tooltip(1)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:116)
- [ ] `CheckboxEdited.Builder.build(0)`: its class isn't translated (it uses Minecraft's `Component`) (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:121)

### `CheckboxEdited.Callback`: it's an interface

- [ ] `CheckboxEdited.Callback.onValueChange(2)`: it has no body (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java:135)

### `MacroCMDSuggestor`: it uses Minecraft's `Minecraft`

- [x] `MacroCMDSuggestor.WHITESPACE_PATTERN` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:52)
- [ ] `MacroCMDSuggestor.ERROR_STYLE`: it uses Minecraft's `Style` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:53)
- [ ] `MacroCMDSuggestor.INFO_STYLE`: it uses Minecraft's `Style` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:54)
- [ ] `MacroCMDSuggestor.HIGHLIGHT_STYLES`: it uses Minecraft's `Style` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:55)
- [ ] `MacroCMDSuggestor.MacroCMDSuggestor(9)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:82)
- [ ] `MacroCMDSuggestor.setWindowActive(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:105)
- [ ] `MacroCMDSuggestor.setCanLeave(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:112)
- [ ] `MacroCMDSuggestor.keyPressed(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:116)
- [ ] `MacroCMDSuggestor.mouseScrolled(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:128)
- [ ] `MacroCMDSuggestor.mouseClicked(3)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:132)
- [ ] `MacroCMDSuggestor.show(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:136)
- [ ] `MacroCMDSuggestor.isOpen(0)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:150)
- [ ] `MacroCMDSuggestor.getSuggestionUsageNarrationText(0)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:154)
- [ ] `MacroCMDSuggestor.clearWindow(0)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:166)
- [ ] `MacroCMDSuggestor.sortSuggestions(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:170)
- [ ] `MacroCMDSuggestor.refresh(0)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:189)
- [ ] `MacroCMDSuggestor.getStartOfCurrentWord(1)`: it calls `Strings` (com.google.common.base).isNullOrEmpty (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:242)
- [ ] `MacroCMDSuggestor.formatException(1)`: it uses Minecraft's `FormattedCharSequence` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:257)
- [ ] `MacroCMDSuggestor.showCommandSuggestions(0)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:265)
- [ ] `MacroCMDSuggestor.showUsages(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:300)
- [ ] `MacroCMDSuggestor.provideRenderText(2)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:329)
- [x] `MacroCMDSuggestor.getSuggestionSuffix(2)` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:333)
- [ ] `MacroCMDSuggestor.highlight(3)`: it uses Minecraft's `FormattedCharSequence` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:338)
- [ ] `MacroCMDSuggestor.render(3)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:376)
- [ ] `MacroCMDSuggestor.tryRenderWindow(3)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:382)
- [ ] `MacroCMDSuggestor.renderMessages(1)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:391)
- [ ] `MacroCMDSuggestor.getNarration(0)`: its class isn't translated (it uses Minecraft's `Minecraft`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:402)

### `MacroCMDSuggestor.SuggestionWindow`: it uses Minecraft's `Rect2i`

- [ ] `MacroCMDSuggestor.SuggestionWindow.SuggestionWindow(5)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:417)
- [ ] `MacroCMDSuggestor.SuggestionWindow.render(3)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:434)
- [ ] `MacroCMDSuggestor.SuggestionWindow.mouseClicked(3)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:514)
- [ ] `MacroCMDSuggestor.SuggestionWindow.mouseScrolled(1)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:528)
- [ ] `MacroCMDSuggestor.SuggestionWindow.keyPressed(1)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:550)
- [ ] `MacroCMDSuggestor.SuggestionWindow.scroll(1)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:579)
- [ ] `MacroCMDSuggestor.SuggestionWindow.select(1)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:592)
- [ ] `MacroCMDSuggestor.SuggestionWindow.complete(0)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:608)
- [ ] `MacroCMDSuggestor.SuggestionWindow.getNarration(0)`: its class isn't translated (it uses Minecraft's `Rect2i`) (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java:620)

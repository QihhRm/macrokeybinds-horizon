# Horizon port report: Macro Keybinds

`macrokeybinds` 2.0.0 · 26.3-macrokeybinds-fabric-2.0.0.jar · 63 classes

**Partly convertible**

| Status | Count | Meaning |
|---|---|---|
| auto | 4 | The Horizon hook or injection is known: the wiring can be generated. The Java body still has to become Rust. |
| content | 1 | Data and assets: horizon-port-pack carries these into a content mod (.hmod). |
| needs-mapping | 0 | The kind of change carries over, but the matching Horizon function isn't in the table yet. Run `match`. |
| needs-translation | 24 | No direct equivalent: a person (or an AI agent) has to rewrite this part. |
| unsupported | 0 | Horizon has nothing this can attach to yet. |

## Entry points

- `client`: com.thomas7520.macrokeybinds.FabricMacroMod
- `modmenu`: com.thomas7520.macrokeybinds.modmenu.ModMenuIntegration

## Fabric events

| Event | In | Horizon | Status | Note |
|---|---|---|---|---|
| KeyMappingHelper.registerKeyMapping | `FabricMacroMod` | hook `key` | auto | Mojang-named variant of KeyBindingHelper. |
| ClientTickEvents.END_CLIENT_TICK | `MacroEvent` | hook `tick` | auto | The Fabric API layer (jfabricapi.rs) runs START_CLIENT_TICK, then the level ticks (START/END_LEVEL_TICK, while in a world), then END_CLIENT_TICK, from Horizon's tick hook, in the window's Game only. |
| ClientPlayConnectionEvents.JOIN | `MacroEvent` | hook `tick` | auto | The Fabric API layer runs JOIN on the window's first tick with a player after none, DISCONNECT on the first tick without one after. |
| ClientPlayConnectionEvents.DISCONNECT | `MacroEvent` | hook `tick` | auto | The Fabric API layer runs JOIN on the window's first tick with a player after none, DISCONNECT on the first tick without one after. |

## Classes extending the game

| Class | In | Status | Note |
|---|---|---|---|
| extends Screen | `EditMacroScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Button | `GlobalMacroScreen$1` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `GlobalMacroScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Screen | `MainMacroScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Button | `ServerMacroScreen$1` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `ServerMacroScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Entry | `EditMacroFormList$Entry` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `EditMacroFormList` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `MacroList$Entry` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `MacroList` | needs-translation | No mapping for this superclass yet. |
| extends AbstractButton | `WheelOptionScreen$SlotIconButton` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `WheelOptionScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Screen | `WheelScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends AbstractWidget | `WheelSelectIconScreen$IconButton` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `WheelSelectIconScreen$IconGrid` | needs-translation | No mapping for this superclass yet. |
| extends Entry | `WheelSelectIconScreen$IconRow` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `WheelSelectIconScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends Entry | `WheelSelectMacroScreen$MacroEntry` | needs-translation | No mapping for this superclass yet. |
| extends ContainerObjectSelectionList | `WheelSelectMacroScreen$MacroSelectionList` | needs-translation | No mapping for this superclass yet. |
| extends Screen | `WheelSelectMacroScreen` | needs-translation | No general screen hook; simple option pages can use the settings hook. |
| extends AbstractWidget | `WheelWidget$Slot` | needs-translation | No mapping for this superclass yet. |
| extends AbstractContainerWidget | `WheelWidget` | needs-translation | No mapping for this superclass yet. |
| extends AbstractButton | `ButtonImageWidget` | needs-translation | No mapping for this superclass yet. |
| extends AbstractButton | `CheckboxEdited` | needs-translation | No mapping for this superclass yet. |

## Content

| What | In | Status | Note |
|---|---|---|---|
| 8 asset files, 0 data files | `jar resources` | content | Recipes, loot, tags, models, textures, worldgen: horizon-port-pack carries these. |

## Game methods and fields used

19 of 195 are mapped to Horizon. Every Java body that uses an unmapped one needs it mapped before its Rust can be written.

<details><summary>Mapped</summary>

| Java | Horizon |
|---|---|
| `com/mojang/blaze3d/platform/InputConstants$Key#getDisplayName` | horizon_client_core::key_display::display_name (suggested) |
| `net/minecraft/client/KeyMapping#<init>` | `self.native_mod_key(MOD_ID, name, default)` |
| `net/minecraft/client/KeyMapping#consumeClick` | `key == self.native_mod_key(..) && pressed, in the key hook` |
| `net/minecraft/client/Minecraft#font` | `(implicit)` |
| `net/minecraft/client/Minecraft#getInstance` | `self` |
| `net/minecraft/client/Minecraft#player` | `self.native_player()` |
| `net/minecraft/client/OptionInstance#set` | horizon_client_render::options_file::OptionsFile::set (suggested) |
| `net/minecraft/client/gui/Font#plainSubstrByWidth` | horizon_client_render::font::Font::fit (suggested) |
| `net/minecraft/client/gui/Font#split` | horizon_client_render::font::Font::split_formatted (suggested) |
| `net/minecraft/client/gui/Font#width` | `self.native_text_width(text)` |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blitSprite` | horizon_client_render::ui::blit_sprite (suggested) |
| `net/minecraft/client/gui/components/EditBox#getScreenX` | horizon_client_render::ui::EditBox::screen_x (suggested) |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendChat` | horizon_client::game::remote::Game<'_>::send_chat (suggested) |
| `net/minecraft/core/registries/BuiltInRegistries#ITEM` | content |
| `net/minecraft/network/chat/MutableComponent#append` | horizon_client::game::dedicated::append (suggested) |
| `net/minecraft/network/chat/MutableComponent#withStyle` | horizon_client_core::text::Text::with_style (suggested) |
| `net/minecraft/util/ARGB#white` | horizon_client_render::hud_layout::white (suggested) |
| `net/minecraft/util/Mth#ceil` | horizon_client_render::hud_layout::ceil (suggested) |
| `net/minecraft/util/Mth#clamp` | horizon_client_render::chat::clamp (suggested) |

</details>

<details><summary>Not mapped yet</summary>

| Java | Kind | Classes using it |
|---|---|---|
| `com/mojang/blaze3d/platform/InputConstants#UNKNOWN` | field | 1 |
| `com/mojang/blaze3d/platform/InputConstants#getKey` | method | 2 |
| `com/mojang/blaze3d/platform/InputConstants#isKeyDown` | method | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getType` | method | 1 |
| `com/mojang/blaze3d/platform/InputConstants$Key#getValue` | method | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#KEYBOARD` | field | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#MOUSE` | field | 2 |
| `com/mojang/blaze3d/platform/InputConstants$Type#getOrCreate` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getGuiScaledWidth` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getHeight` | method | 1 |
| `com/mojang/blaze3d/platform/Window#getWidth` | method | 1 |
| `net/minecraft/ChatFormatting#AQUA` | field | 1 |
| `net/minecraft/ChatFormatting#GOLD` | field | 1 |
| `net/minecraft/ChatFormatting#GRAY` | field | 1 |
| `net/minecraft/ChatFormatting#GREEN` | field | 1 |
| `net/minecraft/ChatFormatting#LIGHT_PURPLE` | field | 1 |
| `net/minecraft/ChatFormatting#RED` | field | 2 |
| `net/minecraft/ChatFormatting#YELLOW` | field | 2 |
| `net/minecraft/client/GameNarrator#saySystemNow` | method | 1 |
| `net/minecraft/client/InputType#isKeyboard` | method | 3 |
| `net/minecraft/client/KeyMapping#isUnbound` | method | 1 |
| `net/minecraft/client/KeyMapping#saveString` | method | 1 |
| `net/minecraft/client/KeyMapping$Category#register` | method | 1 |
| `net/minecraft/client/Minecraft#getConnection` | method | 2 |
| `net/minecraft/client/Minecraft#getLastInputType` | method | 3 |
| `net/minecraft/client/Minecraft#getNarrator` | method | 1 |
| `net/minecraft/client/Minecraft#getSoundManager` | method | 1 |
| `net/minecraft/client/Minecraft#getWindow` | method | 1 |
| `net/minecraft/client/Minecraft#gui` | field | 10 |
| `net/minecraft/client/Minecraft#level` | field | 3 |
| `net/minecraft/client/Minecraft#mouseHandler` | field | 1 |
| `net/minecraft/client/Minecraft#options` | field | 1 |
| `net/minecraft/client/MouseHandler#xpos` | method | 1 |
| `net/minecraft/client/MouseHandler#ypos` | method | 1 |
| `net/minecraft/client/OptionInstance#get` | method | 2 |
| `net/minecraft/client/Options#autoSuggestions` | method | 1 |
| `net/minecraft/client/gui/Gui#hud` | field | 1 |
| `net/minecraft/client/gui/Gui#screen` | method | 1 |
| `net/minecraft/client/gui/Gui#setScreen` | method | 10 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#blit` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#centeredText` | method | 3 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#fill` | method | 5 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#item` | method | 4 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#outline` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#pose` | method | 1 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#setTooltipForNextFrame` | method | 5 |
| `net/minecraft/client/gui/GuiGraphicsExtractor#text` | method | 11 |
| `net/minecraft/client/gui/Hud#getChat` | method | 1 |
| `net/minecraft/client/gui/components/AbstractButton#<init>` | method | 3 |
| `net/minecraft/client/gui/components/AbstractButton#createNarrationMessage` | method | 1 |
| `net/minecraft/client/gui/components/AbstractContainerWidget#<init>` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#<init>` | method | 2 |
| `net/minecraft/client/gui/components/AbstractWidget#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#getRectangle` | method | 3 |
| `net/minecraft/client/gui/components/AbstractWidget#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#playButtonClickSound` | method | 1 |
| `net/minecraft/client/gui/components/AbstractWidget#setPosition` | method | 1 |
| `net/minecraft/client/gui/components/Button#<init>` | method | 2 |
| `net/minecraft/client/gui/components/Button#active` | field | 4 |
| `net/minecraft/client/gui/components/Button#builder` | method | 8 |
| `net/minecraft/client/gui/components/Button#extractRenderState` | method | 1 |
| `net/minecraft/client/gui/components/Button#getMessage` | method | 1 |
| `net/minecraft/client/gui/components/Button#isHovered` | method | 2 |
| `net/minecraft/client/gui/components/Button#setMessage` | method | 3 |
| `net/minecraft/client/gui/components/Button#setPosition` | method | 1 |
| `net/minecraft/client/gui/components/Button#setTooltip` | method | 5 |
| `net/minecraft/client/gui/components/Button#setWidth` | method | 1 |
| `net/minecraft/client/gui/components/Button#visible` | field | 1 |
| `net/minecraft/client/gui/components/Button$Builder#bounds` | method | 8 |
| `net/minecraft/client/gui/components/Button$Builder#build` | method | 8 |
| `net/minecraft/client/gui/components/ChatComponent#addClientSystemMessage` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#<init>` | method | 4 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#getRowWidth` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList#scrollBarX` | method | 1 |
| `net/minecraft/client/gui/components/ContainerObjectSelectionList$Entry#<init>` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#<init>` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#addFormatter` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#getCursorPosition` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#getHeight` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#getInnerWidth` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#getValue` | method | 6 |
| `net/minecraft/client/gui/components/EditBox#getX` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#getY` | method | 3 |
| `net/minecraft/client/gui/components/EditBox#isFocused` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setCursorPosition` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setHighlightPos` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setMaxLength` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#setResponder` | method | 4 |
| `net/minecraft/client/gui/components/EditBox#setSuggestion` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setValue` | method | 2 |
| `net/minecraft/client/gui/components/EditBox#setWidth` | method | 1 |
| `net/minecraft/client/gui/components/EditBox#visible` | field | 1 |
| `net/minecraft/client/gui/components/StringWidget#<init>` | method | 4 |
| `net/minecraft/client/gui/components/StringWidget#getWidth` | method | 4 |
| `net/minecraft/client/gui/components/StringWidget#setPosition` | method | 4 |
| `net/minecraft/client/gui/components/Tooltip#create` | method | 6 |
| `net/minecraft/client/gui/components/events/GuiEventListener#isMouseOver` | method | 1 |
| `net/minecraft/client/gui/narration/NarratedElementType#TITLE` | field | 1 |
| `net/minecraft/client/gui/narration/NarratedElementType#USAGE` | field | 1 |
| `net/minecraft/client/gui/narration/NarrationElementOutput#add` | method | 1 |
| `net/minecraft/client/gui/screens/ChatScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/ConfirmLinkScreen#confirmLinkNow` | method | 1 |
| `net/minecraft/client/gui/screens/ConfirmScreen#<init>` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#<init>` | method | 8 |
| `net/minecraft/client/gui/screens/Screen#extractRenderState` | method | 6 |
| `net/minecraft/client/gui/screens/Screen#height` | field | 3 |
| `net/minecraft/client/gui/screens/Screen#init` | method | 3 |
| `net/minecraft/client/gui/screens/Screen#keyPressed` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#keyReleased` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseClicked` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#mouseScrolled` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#onClose` | method | 3 |
| `net/minecraft/client/gui/screens/Screen#resize` | method | 1 |
| `net/minecraft/client/gui/screens/Screen#tick` | method | 2 |
| `net/minecraft/client/gui/screens/Screen#width` | field | 4 |
| `net/minecraft/client/gui/screens/inventory/tooltip/BelowOrAboveWidgetTooltipPositioner#<init>` | method | 3 |
| `net/minecraft/client/gui/screens/inventory/tooltip/DefaultTooltipPositioner#INSTANCE` | field | 3 |
| `net/minecraft/client/input/KeyEvent#hasShiftDown` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isDown` | method | 1 |
| `net/minecraft/client/input/KeyEvent#isEscape` | method | 2 |
| `net/minecraft/client/input/KeyEvent#isUp` | method | 1 |
| `net/minecraft/client/input/KeyEvent#key` | method | 2 |
| `net/minecraft/client/input/MouseButtonEvent#button` | method | 2 |
| `net/minecraft/client/input/MouseButtonEvent#x` | method | 2 |
| `net/minecraft/client/input/MouseButtonEvent#y` | method | 2 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getCommands` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getServerData` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#getSuggestionsProvider` | method | 1 |
| `net/minecraft/client/multiplayer/ClientPacketListener#sendCommand` | method | 1 |
| `net/minecraft/client/multiplayer/ClientSuggestionProvider#getCustomTabSuggestions` | method | 1 |
| `net/minecraft/client/multiplayer/ServerData#ip` | field | 1 |
| `net/minecraft/client/multiplayer/ServerData#isLan` | method | 1 |
| `net/minecraft/client/player/LocalPlayer#connection` | field | 2 |
| `net/minecraft/client/renderer/Rect2i#<init>` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#contains` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getHeight` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getWidth` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getX` | method | 1 |
| `net/minecraft/client/renderer/Rect2i#getY` | method | 1 |
| `net/minecraft/client/renderer/RenderPipelines#GUI_TEXTURED` | field | 4 |
| `net/minecraft/commands/Commands#getParseException` | method | 1 |
| `net/minecraft/commands/SharedSuggestionProvider#suggest` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#entrySet` | method | 1 |
| `net/minecraft/core/DefaultedRegistry#getOptional` | method | 2 |
| `net/minecraft/network/chat/CommonComponents#EMPTY` | field | 1 |
| `net/minecraft/network/chat/CommonComponents#GUI_CANCEL` | field | 3 |
| `net/minecraft/network/chat/CommonComponents#GUI_DONE` | field | 5 |
| `net/minecraft/network/chat/CommonComponents#NEW_LINE` | field | 1 |
| `net/minecraft/network/chat/Component#copy` | method | 2 |
| `net/minecraft/network/chat/Component#empty` | method | 6 |
| `net/minecraft/network/chat/Component#getString` | method | 1 |
| `net/minecraft/network/chat/Component#getVisualOrderText` | method | 1 |
| `net/minecraft/network/chat/Component#literal` | method | 8 |
| `net/minecraft/network/chat/Component#translatable` | method | 15 |
| `net/minecraft/network/chat/ComponentUtils#fromMessage` | method | 2 |
| `net/minecraft/network/chat/MutableComponent#getString` | method | 1 |
| `net/minecraft/network/chat/MutableComponent#getVisualOrderText` | method | 1 |
| `net/minecraft/network/chat/Style#EMPTY` | field | 1 |
| `net/minecraft/network/chat/Style#withColor` | method | 1 |
| `net/minecraft/resources/Identifier#fromNamespaceAndPath` | method | 5 |
| `net/minecraft/resources/Identifier#toString` | method | 1 |
| `net/minecraft/resources/Identifier#tryParse` | method | 1 |
| `net/minecraft/resources/Identifier#withDefaultNamespace` | method | 1 |
| `net/minecraft/resources/ResourceKey#identifier` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#composite` | method | 1 |
| `net/minecraft/util/FormattedCharSequence#forward` | method | 1 |
| `net/minecraft/world/item/Item#getDefaultInstance` | method | 3 |
| `net/minecraft/world/item/ItemStack#EMPTY` | field | 2 |
| `net/minecraft/world/item/ItemStack#getHoverName` | method | 2 |
| `net/minecraft/world/item/ItemStack#isEmpty` | method | 3 |
| `net/minecraft/world/phys/Vec2#<init>` | method | 1 |
| `net/minecraft/world/phys/Vec2#ZERO` | field | 1 |
| `net/minecraft/world/phys/Vec2#x` | field | 1 |
| `net/minecraft/world/phys/Vec2#y` | field | 1 |

</details>


// Macro Keybinds, ported to Horizon by horizon-port from 26.3-macrokeybinds-fabric-2.0.0.jar.
// This file is both the bundle's Cargo library (its tests) and the module the bundle adds to
// the game (crate::game::port_macrokeybinds). Put the mod's own logic here, in plain Rust
// that doesn't need the game, and call it from the helpers. No `//!` comments: module payloads
// can't hold inner attributes.

/// The mod's id (its settings folder), name, version and description.
pub const ID: &str = "macrokeybinds";
pub const NAME: &str = "Macro Keybinds";
pub const VERSION: &str = "2.0.0";
pub const DESCRIPTION: &str = "MacroKeybinds is a mod that allows the user to create macros in order to execute a task faster.";

/// The mod's keys for Options > Mods: name, label, default key.
pub const KEYS: &[[&str; 3]] = &[
    ["open_options", "Open Macro Options", "key.keyboard.unknown"],
    ["open_wheel", "Open Macro Wheel", "key.keyboard.unknown"],
];

/// An action in the Horizon-native MVP.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MacroAction {
    ExecuteCommand(String),
    SendMessage(String),
    ShowLocalMessage(String),
}

/// A key-triggered macro. `key` uses Horizon's vanilla key names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Macro {
    pub name: String,
    pub key: String,
    pub actions: Vec<MacroAction>,
}

impl Macro {
    pub fn new(name: impl Into<String>, key: impl Into<String>) -> Self {
        Self { name: name.into(), key: key.into(), actions: Vec::new() }
    }
}

/// Persistent macro store and immediate action queue.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct MacroStore {
    pub macros: Vec<Macro>,
    pub queue: Vec<MacroAction>,
}

impl MacroStore {
    pub fn add(&mut self, macro_def: Macro) {
        self.macros.push(macro_def);
    }

    pub fn remove(&mut self, name: &str) -> bool {
        let before = self.macros.len();
        self.macros.retain(|m| m.name != name);
        self.macros.len() != before
    }

    pub fn trigger_key(&mut self, key: &str) -> usize {
        let mut triggered = 0;
        for macro_def in &self.macros {
            if macro_def.key == key {
                self.queue.extend(macro_def.actions.clone());
                triggered += 1;
            }
        }
        triggered
    }

    pub fn pop_action(&mut self) -> Option<MacroAction> {
        if self.queue.is_empty() { None } else { Some(self.queue.remove(0)) }
    }

    pub fn parse_json(text: &str) -> Result<Self, String> {
        let root = java::JJson::parse(text);
        let arr = root.get("macros").ok_or_else(|| "missing macros array".to_string())?;
        let java::JJson::Arr(entries) = arr else {
            return Err("macros must be an array".to_string());
        };

        let mut store = Self::default();
        for entry in entries {
            let name = entry.get("name").and_then(|v| v.is_string().then(|| v.as_string())).unwrap_or_default();
            let key = entry.get("key").and_then(|v| v.is_string().then(|| v.as_string())).unwrap_or_default();
            let mut macro_def = Macro::new(name, key);

            if let Some(java::JJson::Arr(actions)) = entry.get("actions") {
                for action in actions {
                    let kind = action.get("type").map(|v| v.as_string()).unwrap_or_default();
                    let value = action.get("value").map(|v| v.as_string()).unwrap_or_default();
                    macro_def.actions.push(match kind.as_str() {
                        "command" => MacroAction::ExecuteCommand(value),
                        "message" => MacroAction::SendMessage(value),
                        "local" => MacroAction::ShowLocalMessage(value),
                        _ => continue,
                    });
                }
            }
            store.add(macro_def);
        }
        Ok(store)
    }

    pub fn to_json(&self) -> java::JJson {
        let mut macros = java::JJson::array();
        for macro_def in &self.macros {
            let mut m = java::JJson::object();
            m.add("name", java::JJson::str(&macro_def.name));
            m.add("key", java::JJson::str(&macro_def.key));
            let mut actions = java::JJson::array();
            for action in &macro_def.actions {
                let mut a = java::JJson::object();
                match action {
                    MacroAction::ExecuteCommand(value) => {
                        a.add("type", java::JJson::str("command"));
                        a.add("value", java::JJson::str(value));
                    }
                    MacroAction::SendMessage(value) => {
                        a.add("type", java::JJson::str("message"));
                        a.add("value", java::JJson::str(value));
                    }
                    MacroAction::ShowLocalMessage(value) => {
                        a.add("type", java::JJson::str("local"));
                        a.add("value", java::JJson::str(value));
                    }
                }
                actions.push(a);
            }
            m.add("actions", actions);
            macros.push(m);
        }
        let mut root = java::JJson::object();
        root.add("macros", macros);
        root
    }
}

/// The mod's own Java that doesn't need the game, translated to Rust by horizon-port.
/// Each Java method is here whole or not at all; the ones left out say why.
#[allow(dead_code, unused_variables, unused_mut, unused_parens, unused_imports, unreachable_code,
    non_snake_case, non_upper_case_globals, non_camel_case_types, clippy::all)]
pub mod java {
    use std::sync::{LazyLock, Mutex};

    /// A Java static that changes: one value behind a lock.
    pub struct JCell<T>(Mutex<T>);
    impl<T: Clone> JCell<T> {
        pub const fn new(v: T) -> Self {
            Self(Mutex::new(v))
        }
        pub fn get(&self) -> T {
            self.0.lock().unwrap_or_else(|e| e.into_inner()).clone()
        }
        pub fn set(&self, v: T) {
            *self.0.lock().unwrap_or_else(|e| e.into_inner()) = v;
        }
        pub fn with<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
            f(&mut self.0.lock().unwrap_or_else(|e| e.into_inner()))
        }
    }

    /// Java's HashMap, LinkedHashMap and TreeMap. Keeps insertion order (LinkedHashMap's; HashMap's
    /// order is unspecified), or key order when made with `new_sorted` (TreeMap's).
    #[derive(Clone, Debug)]
    pub struct JMap<K, V> {
        index: std::collections::HashMap<K, usize>,
        entries: Vec<(K, V)>,
        sorted: bool,
    }
    /// Java's Map.equals: the same keys with equal values, in any order.
    impl<K: std::hash::Hash + Eq, V: PartialEq> PartialEq for JMap<K, V> {
        fn eq(&self, o: &Self) -> bool {
            self.entries.len() == o.entries.len()
                && self.entries.iter().all(|(k, v)| o.index.get(k).is_some_and(|&i| o.entries[i].1 == *v))
        }
    }
    impl<K, V> Default for JMap<K, V> {
        fn default() -> Self {
            Self { index: std::collections::HashMap::new(), entries: Vec::new(), sorted: false }
        }
    }
    impl<K: std::hash::Hash + Ord + Clone, V> JMap<K, V> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self { sorted: true, ..Self::default() }
        }
        pub fn from_iter(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn from_iter_sorted(pairs: impl IntoIterator<Item = (K, V)>) -> Self {
            let mut m = Self::new_sorted();
            for (k, v) in pairs {
                m.insert(k, v);
            }
            m
        }
        pub fn len(&self) -> usize {
            self.entries.len()
        }
        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }
        pub fn clear(&mut self) {
            self.index.clear();
            self.entries.clear();
        }
        pub fn contains_key<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> bool
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.contains_key(k)
        }
        pub fn get<Q: std::hash::Hash + Eq + ?Sized>(&self, k: &Q) -> Option<&V>
        where
            K: std::borrow::Borrow<Q>,
        {
            self.index.get(k).map(|&i| &self.entries[i].1)
        }
        fn reindex(&mut self) {
            self.index.clear();
            for (i, (k, _)) in self.entries.iter().enumerate() {
                self.index.insert(k.clone(), i);
            }
        }
        fn slot(&mut self, k: K, v: V) -> usize {
            if self.sorted {
                let at = self.entries.partition_point(|e| e.0 < k);
                self.entries.insert(at, (k, v));
                self.reindex();
                at
            } else {
                self.index.insert(k.clone(), self.entries.len());
                self.entries.push((k, v));
                self.entries.len() - 1
            }
        }
        pub fn insert(&mut self, k: K, v: V) -> Option<V> {
            if let Some(&i) = self.index.get(&k) {
                return Some(std::mem::replace(&mut self.entries[i].1, v));
            }
            self.slot(k, v);
            None
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, k: &Q) -> Option<V>
        where
            K: std::borrow::Borrow<Q>,
        {
            let i = *self.index.get(k)?;
            let (_, v) = self.entries.remove(i);
            self.reindex();
            Some(v)
        }
        /// computeIfAbsent and putIfAbsent: the value, made first when the key is missing.
        pub fn or_insert_with(&mut self, k: K, f: impl FnOnce() -> V) -> &mut V {
            let i = match self.index.get(&k) {
                Some(&i) => i,
                None => self.slot(k, f()),
            };
            &mut self.entries[i].1
        }
        pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
            self.entries.iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &K> {
            self.entries.iter().map(|e| &e.0)
        }
        pub fn values(&self) -> impl Iterator<Item = &V> {
            self.entries.iter().map(|e| &e.1)
        }
        pub fn clone_pairs(&self) -> Vec<(K, V)>
        where
            V: Clone,
        {
            self.entries.clone()
        }
    }

    /// Java's HashSet, LinkedHashSet and TreeSet, kept like JMap's keys.
    #[derive(Clone, Debug)]
    pub struct JSet<T>(JMap<T, ()>);
    impl<T: std::hash::Hash + Eq> PartialEq for JSet<T> {
        fn eq(&self, o: &Self) -> bool {
            self.0 == o.0
        }
    }
    impl<T> Default for JSet<T> {
        fn default() -> Self {
            Self(JMap::default())
        }
    }
    impl<T: std::hash::Hash + Ord + Clone> JSet<T> {
        pub fn new() -> Self {
            Self::default()
        }
        pub fn new_sorted() -> Self {
            Self(JMap::new_sorted())
        }
        pub fn from_iter(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn from_iter_sorted(items: impl IntoIterator<Item = T>) -> Self {
            let mut s = Self::new_sorted();
            for v in items {
                s.insert(v);
            }
            s
        }
        pub fn len(&self) -> usize {
            self.0.len()
        }
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
        pub fn clear(&mut self) {
            self.0.clear();
        }
        /// True when it wasn't there yet, as Java's add.
        pub fn insert(&mut self, v: T) -> bool {
            self.0.insert(v, ()).is_none()
        }
        pub fn contains<Q: std::hash::Hash + Eq + ?Sized>(&self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.contains_key(v)
        }
        pub fn remove<Q: std::hash::Hash + Eq + ?Sized>(&mut self, v: &Q) -> bool
        where
            T: std::borrow::Borrow<Q>,
        {
            self.0.remove(v).is_some()
        }
        pub fn iter(&self) -> impl Iterator<Item = &T> {
            self.0.keys()
        }
        pub fn clone_pairs(&self) -> Vec<T> {
            self.0.keys().cloned().collect()
        }
    }

    /// The mod's data folder in Horizon, set by the hooks (game_side.rs `ensure_init`) before the mod's code runs.
    pub static J_DATA_DIR: JCell<Option<std::path::PathBuf>> = JCell::new(None);
    /// FabricLoader's config folder: the mod's data folder in Horizon (./config until the game sets it).
    pub fn j_config_dir() -> std::path::PathBuf {
        J_DATA_DIR.get().unwrap_or_else(|| std::path::PathBuf::from("config"))
    }

    /// Minecraft's RandomSource / java.util.Random: other numbers than Java's, the same ranges.
    #[derive(Clone, Debug)]
    pub struct JRandom(u64);
    impl JRandom {
        pub fn new() -> Self {
            let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_nanos() as u64).unwrap_or(0x9E37_79B9);
            Self(t | 1)
        }
        fn next(&mut self) -> u64 {
            let mut v = self.0;
            v ^= v << 13;
            v ^= v >> 7;
            v ^= v << 17;
            self.0 = v;
            v
        }
        /// 0 inclusive to n exclusive; Java throws on n <= 0.
        pub fn next_int(&mut self, n: i32) -> i32 {
            assert!(n > 0, "bound must be positive");
            (self.next() % n as u64) as i32
        }
        pub fn next_int_between(&mut self, lo: i32, hi: i32) -> i32 {
            lo + self.next_int(hi - lo)
        }
        pub fn next_float(&mut self) -> f32 {
            (self.next() >> 40) as f32 / (1u64 << 24) as f32
        }
        pub fn next_double(&mut self) -> f64 {
            (self.next() >> 11) as f64 / (1u64 << 53) as f64
        }
        pub fn next_boolean(&mut self) -> bool {
            self.next() & 1 == 1
        }
        pub fn next_gaussian(&mut self) -> f64 {
            let (u, v) = (self.next_double().max(1e-12), self.next_double());
            (-2.0 * u.ln()).sqrt() * (std::f64::consts::TAU * v).cos()
        }
    }

    /// Minecraft's Identifier (`namespace:path`), which is only text.
    #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct JIdentifier {
        pub namespace: String,
        pub path: String,
    }
    impl JIdentifier {
        pub fn new(namespace: &str, path: &str) -> Self {
            Self { namespace: namespace.to_string(), path: path.to_string() }
        }
        /// `a:b`, or `minecraft:b` without a namespace.
        pub fn parse(s: &str) -> Self {
            match s.split_once(':') {
                Some((n, p)) => Self::new(n, p),
                None => Self::new("minecraft", s),
            }
        }
        /// Identifier.tryParse: None (null) when it has characters an identifier can't.
        pub fn try_parse(s: &str) -> Option<Self> {
            let id = Self::parse(s);
            Self::try_build(&id.namespace, &id.path)
        }
        pub fn try_build(namespace: &str, path: &str) -> Option<Self> {
            let ns_ok = namespace.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-'));
            let path_ok = path.chars().all(|c| matches!(c, 'a'..='z' | '0'..='9' | '_' | '.' | '-' | '/'));
            if ns_ok && path_ok { Some(Self::new(namespace, path)) } else { None }
        }
    }
    impl std::fmt::Display for JIdentifier {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}:{}", self.namespace, self.path)
        }
    }

    /// Java's String.equals.
    pub fn j_eq(a: &str, b: &str) -> bool {
        a == b
    }
    pub fn j_eq_ignore_case(a: &str, b: &str) -> bool {
        a.to_lowercase() == b.to_lowercase()
    }
    /// Java's String.length (in chars).
    pub fn j_len(s: &str) -> i32 {
        s.chars().count() as i32
    }
    pub fn j_char_at(s: &str, i: i32) -> char {
        s.chars().nth(i as usize).expect("charAt out of range")
    }
    /// Java's substring, by char.
    pub fn j_substring(s: &str, from: i32, to: Option<i32>) -> String {
        let n = s.chars().count() as i32;
        let to = to.unwrap_or(n);
        assert!(0 <= from && from <= to && to <= n, "substring out of range");
        s.chars().skip(from as usize).take((to - from) as usize).collect()
    }
    pub fn j_index_of(s: &str, sub: &str) -> i32 {
        s.find(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    pub fn j_index_of_char(s: &str, c: char) -> i32 {
        s.chars().position(|x| x == c).map_or(-1, |i| i as i32)
    }
    pub fn j_last_index_of(s: &str, sub: &str) -> i32 {
        s.rfind(sub).map_or(-1, |b| s[..b].chars().count() as i32)
    }
    /// Java's String.split on plain text: trailing empty parts dropped.
    pub fn j_split(s: &str, sep: &str) -> Vec<String> {
        let mut v: Vec<String> = s.split(sep).map(str::to_string).collect();
        while v.len() > 1 && v.last().is_some_and(|p| p.is_empty()) {
            v.pop();
        }
        v
    }
    /// Java's String.split(sep, limit) on plain text.
    pub fn j_split_limit(s: &str, sep: &str, limit: i32) -> Vec<String> {
        if limit == 0 {
            return j_split(s, sep);
        }
        if limit > 0 {
            return s.splitn(limit as usize, sep).map(str::to_string).collect();
        }
        s.split(sep).map(str::to_string).collect()
    }
    /// Java's String.compareTo: by UTF-16 unit, then by length.
    pub fn j_compare(a: &str, b: &str) -> i32 {
        let (x, y): (Vec<u16>, Vec<u16>) = (a.encode_utf16().collect(), b.encode_utf16().collect());
        for (p, q) in x.iter().zip(y.iter()) {
            if p != q {
                return *p as i32 - *q as i32;
            }
        }
        x.len() as i32 - y.len() as i32
    }
    /// Java's String.regionMatches, by char.
    pub fn j_region_matches(s: &str, ignore_case: bool, from: i32, other: &str, other_from: i32, len: i32) -> bool {
        let (a, b): (Vec<char>, Vec<char>) = (s.chars().collect(), other.chars().collect());
        if from < 0 || other_from < 0 || from as i64 + len as i64 > a.len() as i64 || other_from as i64 + len as i64 > b.len() as i64 {
            return false;
        }
        (0..len.max(0) as usize).all(|i| {
            let (p, q) = (a[from as usize + i], b[other_from as usize + i]);
            p == q || ignore_case && (j_upper_char(p) == j_upper_char(q) || j_lower_char(p) == j_lower_char(q))
        })
    }
    /// Java's Character.toLowerCase / toUpperCase (one char).
    pub fn j_lower_char(c: char) -> char {
        let mut l = c.to_lowercase();
        if l.len() == 1 { l.next().unwrap_or(c) } else { c }
    }
    pub fn j_upper_char(c: char) -> char {
        let mut u = c.to_uppercase();
        if u.len() == 1 { u.next().unwrap_or(c) } else { c }
    }
    /// Java's Arrays.copyOfRange: past the end is filled with defaults.
    pub fn j_copy_of_range<T: Clone + Default>(a: &[T], from: usize, to: usize) -> Vec<T> {
        assert!(from <= to && from <= a.len(), "copyOfRange out of range");
        let mut v: Vec<T> = a[from..to.min(a.len())].to_vec();
        v.resize(to - from, T::default());
        v
    }
    pub fn j_hash(s: &str) -> i32 {
        s.encode_utf16().fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32))
    }
    /// Java's Boolean.parseBoolean.
    pub fn j_parse_bool(s: &str) -> bool {
        s.eq_ignore_ascii_case("true")
    }
    /// A float as Java prints it in "..." + x (1.0, not 1).
    pub fn j_float(x: f64) -> String {
        if x.is_finite() && x == x.trunc() && x.abs() < 1e7 { format!("{x:.1}") } else { format!("{x}") }
    }
    /// Java's Math.round.
    pub fn j_round_f(x: f32) -> i32 {
        (x + 0.5).floor() as i32
    }
    pub fn j_round_d(x: f64) -> i64 {
        (x + 0.5).floor() as i64
    }
    /// Java's Math.clamp / Mth.clamp (no panic when min > max).
    pub fn j_clamp<T: PartialOrd>(v: T, min: T, max: T) -> T {
        if v < min { min } else if v > max { max } else { v }
    }
    pub fn j_lerp<T: Copy + std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>>(d: T, a: T, b: T) -> T {
        a + d * (b - a)
    }
    pub fn j_signum_f(x: f32) -> f32 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_signum_d(x: f64) -> f64 {
        if x > 0.0 { 1.0 } else if x < 0.0 { -1.0 } else { x }
    }
    pub fn j_wrap_degrees(x: f64) -> f64 {
        let mut v = x % 360.0;
        if v >= 180.0 {
            v -= 360.0;
        }
        if v < -180.0 {
            v += 360.0;
        }
        v
    }
    pub fn j_frac(x: f64) -> f64 {
        x - x.floor()
    }
    pub fn j_compare_f(a: f64, b: f64) -> i32 {
        a.partial_cmp(&b).map_or(0, |o| o as i32)
    }
    pub fn j_copy_of<T: Clone + Default>(a: &[T], n: usize) -> Vec<T> {
        let mut v: Vec<T> = a.iter().take(n).cloned().collect();
        v.resize(n, T::default());
        v
    }
    /// Java's System.currentTimeMillis.
    pub fn j_millis() -> i64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
    }
    /// Java's System.nanoTime: only differences mean anything.
    pub fn j_nanos() -> i64 {
        static START: LazyLock<std::time::Instant> = LazyLock::new(std::time::Instant::now);
        START.elapsed().as_nanos() as i64
    }

    /// `MacroEvent` (com/thomas7520/macrokeybinds/event/MacroEvent.java)
    pub mod macro_event {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MacroEvent {
            pub pressed_keys: JSet<i32>,
        }
        impl MacroEvent {
            pub fn new() -> Self {
                let mut s = Self { pressed_keys: JSet::new() };
                s
            }
            // MacroEvent.onInputEvent(0): not translated, it uses Fabric's `ClientTickEvents`.END_CLIENT_TICK.
            // MacroEvent.onTick(0): not translated, it uses Fabric's `ClientTickEvents`.END_CLIENT_TICK.
            // MacroEvent.onServerConnect(0): not translated, it uses Fabric's `ClientPlayConnectionEvents`.JOIN.
            // MacroEvent.onServerDisconnect(0): not translated, it uses Fabric's `ClientPlayConnectionEvents`.DISCONNECT.
        }
    }

    /// `FabricMacroMod` (com/thomas7520/macrokeybinds/FabricMacroMod.java)
    pub mod fabric_macro_mod {
        use super::*;
        // The class itself isn't translated: it implements Fabric's `ClientModInitializer`. Its static parts are below.
        // FabricMacroMod.onInitializeClient(0): not translated, its class isn't translated (it implements Fabric's `ClientModInitializer`).
    }

    /// `EditMacroScreen` (com/thomas7520/macrokeybinds/gui/EditMacroScreen.java)
    pub mod edit_macro_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const FORM_WIDTH: i32 = 310;
        pub const COLUMN_WIDTH: i32 = 150;
        pub const COLUMN_GAP: i32 = 10;
        pub const COMPACT_FIELD_WIDTH: i32 = 70;
        pub const WIDGET_HEIGHT: i32 = 20;
        pub const TITLE_COLOR: i32 = -1;
        // EditMacroScreen.EditMacroScreen(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.onEdited(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.onSecondEdited(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.cycleActionType(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.keyPressed(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.keyReleased(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.mouseScrolled(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.resize(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.initDataMacro(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.updateTimingFields(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.updateMacroTypeTooltip(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.updateActionTypeTooltip(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.rebuildFormRows(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.isValidRepeatCount(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.getKeyName(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.hasConflictKey(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.createButton(6): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.createPositioner(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.isKeyCodeModifier(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // EditMacroScreen.getPressedModifierKeyCode(0): not translated, it calls Minecraft's `InputConstants`.isKeyDown.
    }

    /// `GlobalMacroScreen` (com/thomas7520/macrokeybinds/gui/GlobalMacroScreen.java)
    pub mod global_macro_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub static STOP_ICON: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("macrokeybinds", "textures/stop_icon.png"));
        // GlobalMacroScreen.GlobalMacroScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GlobalMacroScreen.init(0): not translated, its body couldn't be read: an anonymous class.
        // GlobalMacroScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // GlobalMacroScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `MainMacroScreen` (com/thomas7520/macrokeybinds/gui/MainMacroScreen.java)
    pub mod main_macro_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // MainMacroScreen.MainMacroScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MainMacroScreen.MainMacroScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MainMacroScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MainMacroScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MainMacroScreen.createButton(6): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MainMacroScreen.createUrlButton(6): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MainMacroScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // MainMacroScreen.createPositioner(3): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `EditMacroFormList` (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java)
    pub mod edit_macro_form_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        pub const FORM_WIDTH: i32 = 310;
        pub const ROW_HEIGHT: i32 = 40;
        pub const LABEL_COLOR: i32 = -2039584;
        // EditMacroFormList.EditMacroFormList(2): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // EditMacroFormList.clearRows(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // EditMacroFormList.addRow(1): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // EditMacroFormList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // EditMacroFormList.scrollBarX(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // EditMacroFormList.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `EditMacroFormList.Entry` (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java)
    pub mod edit_macro_form_list_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // EditMacroFormList.Entry.Entry(2): not translated, its class isn't translated (it extends `Entry`).
        // EditMacroFormList.Entry.extractContent(5): not translated, its class isn't translated (it extends `Entry`).
        // EditMacroFormList.Entry.children(0): not translated, its class isn't translated (it extends `Entry`).
        // EditMacroFormList.Entry.narratables(0): not translated, its class isn't translated (it extends `Entry`).
    }

    /// `EditMacroFormList.Field` (com/thomas7520/macrokeybinds/gui/other/EditMacroFormList.java)
    pub mod edit_macro_form_list_field {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // EditMacroFormList.Field.Field(3): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // EditMacroFormList.Field.label(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // EditMacroFormList.Field.widget(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // EditMacroFormList.Field.xOffset(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `MacroList` (com/thomas7520/macrokeybinds/gui/other/MacroList.java)
    pub mod macro_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        pub static EDIT_ICON: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("macrokeybinds", "textures/edit_button.png"));
        pub static DELETE_ICON: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("macrokeybinds", "textures/delete_button.png"));
        // MacroList.MacroList(4): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // MacroList.update(2): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // MacroList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // MacroList.updateList(1): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // MacroList.scrollBarX(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // MacroList.createPositioner(3): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `MacroList.Entry` (com/thomas7520/macrokeybinds/gui/other/MacroList.java)
    pub mod macro_list_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // MacroList.Entry.update(0): not translated, it has no body.
    }

    /// `MacroList.KeyBindingEntry` (com/thomas7520/macrokeybinds/gui/other/MacroList.java)
    pub mod macro_list_key_binding_entry {
        use super::*;
        // The class itself isn't translated: it extends `Entry`. Its static parts are below.
        // MacroList.KeyBindingEntry.KeyBindingEntry(3): not translated, its class isn't translated (it extends `Entry`).
        // MacroList.KeyBindingEntry.extractContent(5): not translated, its class isn't translated (it extends `Entry`).
        // MacroList.KeyBindingEntry.children(0): not translated, its class isn't translated (it extends `Entry`).
        // MacroList.KeyBindingEntry.narratables(0): not translated, its class isn't translated (it extends `Entry`).
        // MacroList.KeyBindingEntry.update(0): not translated, its class isn't translated (it extends `Entry`).
    }

    /// `ServerMacroScreen` (com/thomas7520/macrokeybinds/gui/ServerMacroScreen.java)
    pub mod server_macro_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub static STOP_ICON: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("macrokeybinds", "textures/stop_icon.png"));
        // ServerMacroScreen.ServerMacroScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ServerMacroScreen.init(0): not translated, its body couldn't be read: an anonymous class.
        // ServerMacroScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // ServerMacroScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `WheelOptionScreen` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java)
    pub mod wheel_option_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub static DELETE_ICON: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("macrokeybinds", "textures/delete_button.png"));
        pub const MAX_PREVIEW_RADIUS: i32 = 70;
        pub const CONTROLS_DISTANCE_FROM_CIRCLE: i32 = 14;
        pub const DELETE_BUTTON_SIZE: i32 = 20;
        pub const ICON_BUTTON_SIZE: i32 = 20;
        pub const ADD_BUTTON_SIZE: i32 = 18;
        // WheelOptionScreen.WheelOptionScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.addSlotControls(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.openMacroSelector(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.removeMacro(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.getWheelSlot(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.setWheelSlot(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.changeMode(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.modeText(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.changeIcon(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.saveWheel(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelOptionScreen.iconActionText(1): not translated, it uses Minecraft's `Component`.
        // WheelOptionScreen.getIcon(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `WheelOptionScreen.SlotIconButton` (com/thomas7520/macrokeybinds/gui/wheel/WheelOptionScreen.java)
    pub mod wheel_option_screen_slot_icon_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractButton`. Its static parts are below.
        // WheelOptionScreen.SlotIconButton.SlotIconButton(5): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // WheelOptionScreen.SlotIconButton.onPress(1): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // WheelOptionScreen.SlotIconButton.extractContents(4): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // WheelOptionScreen.SlotIconButton.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
    }

    /// `WheelScreen` (com/thomas7520/macrokeybinds/gui/wheel/WheelScreen.java)
    pub mod wheel_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const HOLD_HOVER_DEAD_ZONE: i32 = 10;
        // WheelScreen.WheelScreen(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelScreen.tick(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelScreen.confirmHoveredSlot(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelScreen.isPauseScreen(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `WheelSelectIconScreen` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java)
    pub mod wheel_select_icon_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        pub const ICON_SIZE: i32 = 20;
        pub const ICON_GAP: i32 = 4;
        // WheelSelectIconScreen.WheelSelectIconScreen(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectIconScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectIconScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectIconScreen.selectIcon(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectIconScreen.confirmSelection(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectIconScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `WheelSelectIconScreen.IconButton` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java)
    pub mod wheel_select_icon_screen_icon_button {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractWidget`. Its static parts are below.
        // WheelSelectIconScreen.IconButton.IconButton(2): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelSelectIconScreen.IconButton.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelSelectIconScreen.IconButton.onClick(2): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelSelectIconScreen.IconButton.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
    }

    /// `WheelSelectIconScreen.IconGrid` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java)
    pub mod wheel_select_icon_screen_icon_grid {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        // WheelSelectIconScreen.IconGrid.IconGrid(4): not translated, its body couldn't be read: an inner class creation.
        // WheelSelectIconScreen.IconGrid.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `WheelSelectIconScreen.IconRow` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectIconScreen.java)
    pub mod wheel_select_icon_screen_icon_row {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Entry`. Its static parts are below.
        // WheelSelectIconScreen.IconRow.IconRow(1): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // WheelSelectIconScreen.IconRow.extractContent(5): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // WheelSelectIconScreen.IconRow.children(0): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // WheelSelectIconScreen.IconRow.narratables(0): not translated, its class isn't translated (it extends Minecraft's `Entry`).
    }

    /// `WheelSelectMacroScreen` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java)
    pub mod wheel_select_macro_screen {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Screen`. Its static parts are below.
        // WheelSelectMacroScreen.WheelSelectMacroScreen(2): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectMacroScreen.init(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectMacroScreen.isMacroAlreadyInWheel(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectMacroScreen.selectMacro(1): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectMacroScreen.onClose(0): not translated, its class isn't translated (it extends Minecraft's `Screen`).
        // WheelSelectMacroScreen.extractRenderState(4): not translated, its class isn't translated (it extends Minecraft's `Screen`).
    }

    /// `WheelSelectMacroScreen.MacroEntry` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java)
    pub mod wheel_select_macro_screen_macro_entry {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `Entry`. Its static parts are below.
        // WheelSelectMacroScreen.MacroEntry.MacroEntry(1): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // WheelSelectMacroScreen.MacroEntry.extractContent(5): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // WheelSelectMacroScreen.MacroEntry.children(0): not translated, its class isn't translated (it extends Minecraft's `Entry`).
        // WheelSelectMacroScreen.MacroEntry.narratables(0): not translated, its class isn't translated (it extends Minecraft's `Entry`).
    }

    /// `WheelSelectMacroScreen.MacroSelectionList` (com/thomas7520/macrokeybinds/gui/wheel/WheelSelectMacroScreen.java)
    pub mod wheel_select_macro_screen_macro_selection_list {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `ContainerObjectSelectionList`. Its static parts are below.
        // WheelSelectMacroScreen.MacroSelectionList.MacroSelectionList(5): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
        // WheelSelectMacroScreen.MacroSelectionList.update(1): not translated, its body couldn't be read: an inner class creation.
        // WheelSelectMacroScreen.MacroSelectionList.getRowWidth(0): not translated, its class isn't translated (it extends Minecraft's `ContainerObjectSelectionList`).
    }

    /// `WheelWidget` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java)
    pub mod wheel_widget {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractContainerWidget`. Its static parts are below.
        pub const SLOTS_MAX: i32 = 6;
        pub const SLOT_SIZE: i32 = 60;
        pub const THICKNESS_LINE: i32 = 2;
        pub const FIRST_SLOT_START_ANGLE: f64 = 240.0f64;
        pub const SLOT_COLOR: i32 = -1340071904;
        pub const HOVERED_SLOT_COLOR: i32 = -798990240;
        pub const DISABLED_OVERLAY_COLOR: i32 = -2138038240;
        pub const LINE_COLOR: i32 = -3092272;
        pub const FULL_CIRCLE: f64 = 360.0f64;
        pub const RENDER_SCALE: i32 = 3;
        pub const PIXEL_CENTER: f64 = 0.5f64;
        pub const MAX_NAME_WIDTH: i32 = 72;
        pub const NAME_PADDING: i32 = 3;
        // WheelWidget.WheelWidget(9): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        pub fn get_slot_middle_angle(slot_index: i32) -> f64 {
            return (240.0f64 + ((slot_index * 60) as f64)) + 30.0f64;
        }
        // WheelWidget.children(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.contentHeight(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.renderDisabledOverlay(5): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.renderSlotContents(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.renderMacroState(6): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.findMacro(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.isDisabled(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.findIcon(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.getMaxNameWidth(6): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.isInsideSlot(5): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.trimName(3): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.fillArc(6): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.drawSeparator(4): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        pub fn contains_angle(angle: f64, start_angle: f64, end_angle: f64) -> bool {
            let normalized_angle: f64 = wheel_widget::normalize_angle(angle);
            let normalized_start: f64 = wheel_widget::normalize_angle(start_angle);
            let normalized_end: f64 = wheel_widget::normalize_angle(end_angle);
            return if normalized_start <= normalized_end { (normalized_angle >= normalized_start) && (normalized_angle < normalized_end) } else { (normalized_angle >= normalized_start) || (normalized_angle < normalized_end) };
        }
        pub fn normalize_angle(angle: f64) -> f64 {
            let normalized: f64 = angle % 360.0f64;
            return if normalized < 0.0f64 { normalized + 360.0f64 } else { normalized };
        }
        // WheelWidget.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.mouseMoved(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.updateHoveredSlot(2): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
        // WheelWidget.getHoveredSlot(0): not translated, its class isn't translated (it extends Minecraft's `AbstractContainerWidget`).
    }

    /// `WheelWidget.Slot` (com/thomas7520/macrokeybinds/gui/wheel/WheelWidget.java)
    pub mod wheel_widget_slot {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractWidget`. Its static parts are below.
        // WheelWidget.Slot.Slot(4): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelWidget.Slot.isMouseOver(2): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelWidget.Slot.onClick(2): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelWidget.Slot.mouseClicked(2): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelWidget.Slot.extractWidgetRenderState(4): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelWidget.Slot.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
        // WheelWidget.Slot.getWheelSlot(0): not translated, its class isn't translated (it extends Minecraft's `AbstractWidget`).
    }

    /// `MacroMod` (com/thomas7520/macrokeybinds/MacroMod.java)
    pub mod macro_mod {
        use super::*;
        pub const MODID: &str = "macrokeybinds";
        // LOGGER: the logger. Log calls become comments.
        #[derive(Clone, Debug)]
        pub struct MacroMod {
        }
        impl MacroMod {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MacroMod.setup(0): not translated, it uses `File` (java.io).
    }

    /// `ModMenuIntegration` (com/thomas7520/macrokeybinds/modmenu/ModMenuIntegration.java)
    pub mod mod_menu_integration {
        use super::*;
        // The class itself isn't translated: it implements `ModMenuApi` (com.terraformersmc.modmenu.api). Its static parts are below.
        // ModMenuIntegration.getModConfigScreenFactory(0): not translated, its class isn't translated (it implements `ModMenuApi` (com.terraformersmc.modmenu.api)).
    }

    /// `AlternateMacro` (com/thomas7520/macrokeybinds/object/macro/AlternateMacro.java)
    pub mod alternate_macro {
        use super::*;
        // The class itself isn't translated: it implements `IMacro`. Its static parts are below.
        // AlternateMacro.AlternateMacro(11): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getUUID(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getName(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getCreatedTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getType(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getActionText(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getSecondActionText(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getSecondAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.doAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.start(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.isStart(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.isSecondActionNext(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.reset(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getKey(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getKeyName(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.isEnable(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.setEnable(1): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.setKey(1): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.getModifier(0): not translated, its class isn't translated (it implements `IMacro`).
        // AlternateMacro.setModifier(1): not translated, its class isn't translated (it implements `IMacro`).
    }

    /// `CountedRepeatMacro` (com/thomas7520/macrokeybinds/object/macro/CountedRepeatMacro.java)
    pub mod counted_repeat_macro {
        use super::*;
        // The class itself isn't translated: it implements `IMacro`. Its static parts are below.
        // CountedRepeatMacro.CountedRepeatMacro(11): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getUUID(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getName(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getCreatedTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getType(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getActionText(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getKey(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getKeyName(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.isEnable(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.setEnable(1): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.setKey(1): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getModifier(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.setModifier(1): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.doAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.start(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.cancel(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.isRunning(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getIntervalTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // CountedRepeatMacro.getRepeatCount(0): not translated, its class isn't translated (it implements `IMacro`).
    }

    /// `DelayedMacro` (com/thomas7520/macrokeybinds/object/macro/DelayedMacro.java)
    pub mod delayed_macro {
        use super::*;
        // The class itself isn't translated: it implements `IMacro`. Its static parts are below.
        // DelayedMacro.DelayedMacro(10): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getUUID(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getName(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getCreatedTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getType(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getActionText(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getKey(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getKeyName(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getDelayedTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getStartTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.setStartTime(1): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.isStart(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.setStart(1): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.isEnable(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.setEnable(1): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.setKey(1): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getModifier(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.setModifier(1): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.getAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // DelayedMacro.doAction(0): not translated, its class isn't translated (it implements `IMacro`).
    }

    /// `IMacro` (com/thomas7520/macrokeybinds/object/macro/IMacro.java)
    pub mod i_macro {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IMacro.getUUID(0): not translated, it has no body.
        // IMacro.getName(0): not translated, it has no body.
        // IMacro.getCreatedTime(0): not translated, it has no body.
        // IMacro.getType(0): not translated, it has no body.
        // IMacro.getActionText(0): not translated, it has no body.
        // IMacro.getAction(0): not translated, it has no body.
        // IMacro.doAction(0): not translated, it has no body.
        // IMacro.getKey(0): not translated, it has no body.
        // IMacro.getKeyName(0): not translated, it has no body.
        // IMacro.isEnable(0): not translated, it has no body.
        // IMacro.setEnable(1): not translated, it has no body.
        // IMacro.setKey(1): not translated, it has no body.
        // IMacro.getModifier(0): not translated, it has no body.
        // IMacro.setModifier(1): not translated, it has no body.
    }

    /// `KeyAction` (com/thomas7520/macrokeybinds/object/macro/KeyAction.java)
    pub mod key_action {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum KeyAction {
            Message,
            Command,
            FillChat,
            LocalMessage,
        }
        impl KeyAction {
            pub const VALUES: [KeyAction; 4] = [KeyAction::Message, KeyAction::Command, KeyAction::FillChat, KeyAction::LocalMessage, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    KeyAction::Message => "MESSAGE",
                    KeyAction::Command => "COMMAND",
                    KeyAction::FillChat => "FILL_CHAT",
                    KeyAction::LocalMessage => "LOCAL_MESSAGE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "MESSAGE" => KeyAction::Message,
                    "COMMAND" => KeyAction::Command,
                    "FILL_CHAT" => KeyAction::FillChat,
                    "LOCAL_MESSAGE" => KeyAction::LocalMessage,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `MacroModifier` (com/thomas7520/macrokeybinds/object/macro/MacroModifier.java)
    pub mod macro_modifier {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum MacroModifier {
            None,
            Shift,
            Control,
            Alt,
        }
        impl MacroModifier {
            pub const VALUES: [MacroModifier; 4] = [MacroModifier::None, MacroModifier::Shift, MacroModifier::Control, MacroModifier::Alt, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    MacroModifier::None => "NONE",
                    MacroModifier::Shift => "SHIFT",
                    MacroModifier::Control => "CONTROL",
                    MacroModifier::Alt => "ALT",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "NONE" => MacroModifier::None,
                    "SHIFT" => MacroModifier::Shift,
                    "CONTROL" => MacroModifier::Control,
                    "ALT" => MacroModifier::Alt,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `MacroType` (com/thomas7520/macrokeybinds/object/macro/MacroType.java)
    pub mod macro_type {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum MacroType {
            Simple,
            Toggle,
            Repeat,
            Delayed,
            CountedRepeat,
            Alternate,
        }
        impl MacroType {
            pub const VALUES: [MacroType; 6] = [MacroType::Simple, MacroType::Toggle, MacroType::Repeat, MacroType::Delayed, MacroType::CountedRepeat, MacroType::Alternate, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    MacroType::Simple => "SIMPLE",
                    MacroType::Toggle => "TOGGLE",
                    MacroType::Repeat => "REPEAT",
                    MacroType::Delayed => "DELAYED",
                    MacroType::CountedRepeat => "COUNTED_REPEAT",
                    MacroType::Alternate => "ALTERNATE",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "SIMPLE" => MacroType::Simple,
                    "TOGGLE" => MacroType::Toggle,
                    "REPEAT" => MacroType::Repeat,
                    "DELAYED" => MacroType::Delayed,
                    "COUNTED_REPEAT" => MacroType::CountedRepeat,
                    "ALTERNATE" => MacroType::Alternate,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `RepeatMacro` (com/thomas7520/macrokeybinds/object/macro/RepeatMacro.java)
    pub mod repeat_macro {
        use super::*;
        // The class itself isn't translated: it implements `IMacro`. Its static parts are below.
        // RepeatMacro.RepeatMacro(10): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getUUID(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getName(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getCreatedTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getActionText(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getKey(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getKeyName(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.isEnable(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.setEnable(1): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.setKey(1): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getModifier(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.setModifier(1): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getType(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.doAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.getCooldownTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.isRepeat(0): not translated, its class isn't translated (it implements `IMacro`).
        // RepeatMacro.setRepeat(1): not translated, its class isn't translated (it implements `IMacro`).
    }

    /// `SimpleMacro` (com/thomas7520/macrokeybinds/object/macro/SimpleMacro.java)
    pub mod simple_macro {
        use super::*;
        // The class itself isn't translated: it implements `IMacro`. Its static parts are below.
        // SimpleMacro.SimpleMacro(9): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getUUID(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getName(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getCreatedTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getActionText(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getKey(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getKeyName(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.isEnable(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.setEnable(1): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.setKey(1): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getModifier(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.setModifier(1): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getType(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.getStartTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.setStartTime(1): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.isStart(0): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.setStart(1): not translated, its class isn't translated (it implements `IMacro`).
        // SimpleMacro.doAction(0): not translated, its class isn't translated (it implements `IMacro`).
    }

    /// `ToggleMacro` (com/thomas7520/macrokeybinds/object/macro/ToggleMacro.java)
    pub mod toggle_macro {
        use super::*;
        // The class itself isn't translated: it implements `IMacro`. Its static parts are below.
        // ToggleMacro.ToggleMacro(10): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getUUID(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getName(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getCreatedTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getActionText(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getKey(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getKeyName(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.isEnable(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.setEnable(1): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.setKey(1): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getModifier(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.setModifier(1): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getType(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.doAction(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.getCooldownTime(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.isToggled(0): not translated, its class isn't translated (it implements `IMacro`).
        // ToggleMacro.setToggled(1): not translated, its class isn't translated (it implements `IMacro`).
    }

    /// `Wheel` (com/thomas7520/macrokeybinds/object/wheel/Wheel.java)
    pub mod wheel {
        use super::*;
        // The class itself isn't translated: it uses WheelSlot, which isn't translated (it uses `UUID` (java.util)). Its static parts are below.
        pub const MAX_MACROS: i32 = 6;
        // Wheel.Wheel(0): not translated, a WheelMode with no value (null in Java).
        // Wheel.Wheel(2): not translated, its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
        // Wheel.getMode(0): not translated, its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
        // Wheel.setMode(1): not translated, its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
        // Wheel.getMacros(0): not translated, its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
        // Wheel.setMacros(1): not translated, its class isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
    }

    /// `WheelMode` (com/thomas7520/macrokeybinds/object/wheel/WheelMode.java)
    pub mod wheel_mode {
        use super::*;
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
        pub enum WheelMode {
            Click,
            Hold,
        }
        impl WheelMode {
            pub const VALUES: [WheelMode; 2] = [WheelMode::Click, WheelMode::Hold, ];
            /// Java's `name()`.
            pub fn java_name(self) -> &'static str {
                match self {
                    WheelMode::Click => "CLICK",
                    WheelMode::Hold => "HOLD",
                }
            }
            /// Java's `valueOf`: panics on a name that isn't one, as Java throws.
            pub fn value_of(name: &str) -> Self {
                match name {
                    "CLICK" => WheelMode::Click,
                    "HOLD" => WheelMode::Hold,
                    _ => panic!("No enum constant {name}"),
                }
            }
        }
    }

    /// `WheelSlot` (com/thomas7520/macrokeybinds/object/wheel/WheelSlot.java)
    pub mod wheel_slot {
        use super::*;
        // The class itself isn't translated: it uses `UUID` (java.util). Its static parts are below.
        // WheelSlot.WheelSlot(2): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // WheelSlot.getMacroId(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // WheelSlot.setMacroId(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // WheelSlot.getIconId(0): not translated, its class isn't translated (it uses `UUID` (java.util)).
        // WheelSlot.setIconId(1): not translated, its class isn't translated (it uses `UUID` (java.util)).
    }

    /// `FabricPlatformHelper` (com/thomas7520/macrokeybinds/platform/FabricPlatformHelper.java)
    pub mod fabric_platform_helper {
        use super::*;
        // The class itself isn't translated: it implements `IPlatformHelper` (com.thomas7520.macrokeybinds.platform.services). Its static parts are below.
        // FabricPlatformHelper.getConfigDirectory(0): not translated, its class isn't translated (it implements `IPlatformHelper` (com.thomas7520.macrokeybinds.platform.services)).
    }

    /// `Services` (com/thomas7520/macrokeybinds/platform/Services.java)
    pub mod services {
        use super::*;
        // PLATFORM: not translated, it uses IPlatformHelper, which isn't translated (it's an interface).
        #[derive(Clone, Debug)]
        pub struct Services {
        }
        impl Services {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // Services.load(1): not translated, it's generic.
    }

    /// `IPlatformHelper` (com/thomas7520/macrokeybinds/platform/services/IPlatformHelper.java)
    pub mod i_platform_helper {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // IPlatformHelper.getConfigDirectory(0): not translated, it has no body.
    }

    /// `MacroActionExecutor` (com/thomas7520/macrokeybinds/util/MacroActionExecutor.java)
    pub mod macro_action_executor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MacroActionExecutor {
        }
        impl MacroActionExecutor {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MacroActionExecutor.execute(2): not translated, it uses Minecraft's `Minecraft`.
    }

    /// `MacroExecutor` (com/thomas7520/macrokeybinds/util/MacroExecutor.java)
    pub mod macro_executor {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MacroExecutor {
        }
        impl MacroExecutor {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MacroExecutor.trigger(1): not translated, it uses IMacro, which isn't translated (it's an interface).
        // MacroExecutor.tick(0): not translated, it calls Minecraft's `Minecraft`.getInstance.
    }

    /// `MacroFlow` (com/thomas7520/macrokeybinds/util/MacroFlow.java)
    pub mod macro_flow {
        use super::*;
        pub const CURRENT_VERSION: &str = "2.0.0";
        #[derive(Clone, Debug)]
        pub struct MacroFlow {
        }
        impl MacroFlow {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MacroFlow.getMacroFromFile(1): not translated, it uses IMacro, which isn't translated (it's an interface).
        // MacroFlow.validateMacro(1): not translated, it uses IMacro, which isn't translated (it's an interface).
        pub fn migrate_legacy_macro_to1_4_0(object: &mut JJson) -> bool {
            if object.has("version") {
                return false;
            }
            let macro_type: String = ((object.get("macroType")).expect("null")).as_string();
            match &*macro_type {
                "TOGGLE" => {
                    object.add("macroType", JJson::str("REPEAT"));
                }
                "REPEAT" => {
                    object.add("macroType", JJson::str("TOGGLE"));
                }
                _ => {}
            }
            object.add("version", JJson::str("2.0.0"));
            return true;
        }
        // MacroFlow.migrateInputToSdl(1): not translated, it calls MacroInputHandler.migrateLegacyKey, which isn't translated.
        // MacroFlow.writeMacro(2): not translated, it uses IMacro, which isn't translated (it's an interface).
        // MacroFlow.writeMacroFile(2): not translated, it uses IMacro, which isn't translated (it's an interface).
    }

    /// `MacroInputHandler` (com/thomas7520/macrokeybinds/util/MacroInputHandler.java)
    pub mod macro_input_handler {
        use super::*;
        pub const KEYBOARD_KEY_OFFSET: i32 = 1000;
        pub const MOUSE_KEY_OFFSET: i32 = 2000;
        #[derive(Clone, Debug)]
        pub struct MacroInputHandler {
        }
        impl MacroInputHandler {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MacroInputHandler.checkOpenGui(0): not translated, it uses Minecraft's `KeyMapping`.
        // MacroInputHandler.canReceiveInput(0): needs the game, so it's in game_side.rs.
        // MacroInputHandler.isWheelBindingDown(0): not translated, it uses Minecraft's `KeyMapping`.
        // MacroInputHandler.checkInputs(1): not translated, it calls MacroInputHandler.checkOpenGui, which isn't translated.
        // MacroInputHandler.handleKeyAction(3): not translated, it calls MacroInputHandler.checkOpenGui, which isn't translated.
        // MacroInputHandler.handleMouseAction(2): not translated, it calls MacroInputHandler.checkOpenGui, which isn't translated.
        // MacroInputHandler.handleInput(4): not translated, its body couldn't be read: a labelled statement.
        // MacroInputHandler.getEnabledMacroKeys(0): not translated, it uses IMacro, which isn't translated (it's an interface).
        pub fn modifier_from_mask(modifiers: i32) -> macro_modifier::MacroModifier {
            if (modifiers & 3) != 0 {
                return macro_modifier::MacroModifier::Shift;
            } else if (modifiers & 768) != 0 {
                return macro_modifier::MacroModifier::Alt;
            } else {
                return if (modifiers & 192) != 0 { macro_modifier::MacroModifier::Control } else { macro_modifier::MacroModifier::None };
            }
            #[allow(unreachable_code)]
            unreachable!()
        }
        // MacroInputHandler.getPressedModifier(0): not translated, it calls MacroInputHandler.isPressed, which isn't translated.
        // MacroInputHandler.isPressed(2): not translated, it calls Minecraft's `InputConstants`.isKeyDown.
        // MacroInputHandler.isStoredKeyDown(1): not translated, it calls Minecraft's `InputConstants`.isKeyDown.
        // MacroInputHandler.isMouseButtonDown(1): not translated, `null` (Rust has no null: this needs an Option).
        pub fn is_keyboard_key(key: i32) -> bool {
            return (key >= 1000) && (key < 2000);
        }
        pub fn is_mouse_key(key: i32) -> bool {
            return key >= 2000;
        }
        pub fn encode_keyboard_key(key: i32) -> i32 {
            return 1000 + key;
        }
        pub fn encode_mouse_button(button: i32) -> i32 {
            return 2000 + button;
        }
        // MacroInputHandler.migrateLegacyKey(1): not translated, it uses Minecraft's `InputConstants`.UNKNOWN.
    }

    /// `MacroSessionManager` (com/thomas7520/macrokeybinds/util/MacroSessionManager.java)
    pub mod macro_session_manager {
        use super::*;
        #[derive(Clone, Debug)]
        pub struct MacroSessionManager {
        }
        impl MacroSessionManager {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MacroSessionManager.connectToServer(1): not translated, it calls MacroUtil.initServerMacros, which isn't translated.
        // MacroSessionManager.disconnectFromServer(0): not translated, it uses `UUID` (java.util).
        // MacroSessionManager.resetRuntimeState(0): not translated, it uses IMacro, which isn't translated (it's an interface).
    }

    /// `MacroUtil` (com/thomas7520/macrokeybinds/util/MacroUtil.java)
    pub mod macro_util {
        use super::*;
        // keybinds: not translated, it uses `UUID` (java.util).
        // serverKeybinds: not translated, it uses `UUID` (java.util).
        // wheel: not translated, it uses Wheel, which isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
        pub static SERVER_IP: LazyLock<JCell<Option<String>>> = LazyLock::new(|| JCell::new(Some("".to_string())));
        // guiBinding: not translated, it uses Minecraft's `KeyMapping`.
        // wheelBinding: not translated, it uses Minecraft's `KeyMapping`.
        #[derive(Clone, Debug)]
        pub struct MacroUtil {
        }
        impl MacroUtil {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // MacroUtil.initMacroDirectories(0): not translated, it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)).
        // MacroUtil.initServerMacros(1): not translated, it uses `File` (java.io).
        // MacroUtil.getGlobalKeybindsMap(0): not translated, it uses `UUID` (java.util).
        // MacroUtil.getServerKeybinds(0): not translated, it uses `UUID` (java.util).
        // MacroUtil.getWheel(0): not translated, it uses Wheel, which isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
        // MacroUtil.setWheel(1): not translated, it uses Wheel, which isn't translated (it uses WheelSlot, which isn't translated (it uses `UUID` (java.util))).
        // MacroUtil.getAllMacros(0): not translated, it uses IMacro, which isn't translated (it's an interface).
        // MacroUtil.getMacro(1): not translated, it uses IMacro, which isn't translated (it's an interface).
        pub fn get_server_ip() -> Option<String> {
            return macro_util::SERVER_IP.get();
        }
        // MacroUtil.getGlobalMacroDirectory(0): not translated, it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)).
        // MacroUtil.getServerMacrosDirectory(0): not translated, it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)).
        // MacroUtil.getServerMacroDirectory(0): not translated, String.getBytes with 1 values.
        pub fn migrate_legacy_macro_directory(legacy_directory: &std::path::Path, directory: &std::path::Path) -> std::io::Result<()> {
            if legacy_directory.is_dir() && (!directory.exists()) {
                std::fs::rename(&legacy_directory, &directory)?;
                // log: "Migrated macro directory from {} to {}"
            }
            Ok(())
        }
        // MacroUtil.migrateLegacyServerDirectory(3): not translated, a try/catch (InvalidPathException).
        pub fn set_server_ip(ip: &str) {
            macro_util::SERVER_IP.set(Some(ip.to_string()));
        }
        // MacroUtil.isNumeric(1): not translated, it creates `BigInteger` (java.math).
        // MacroUtil.isCombinationAssigned(1): not translated, it uses IMacro, which isn't translated (it's an interface).
        // MacroUtil.isCombinationAssigned(3): not translated, it uses IMacro, which isn't translated (it's an interface).
        // MacroUtil.isCombinationAssigned(2): not translated, it calls MacroUtil.getServerKeybinds, which isn't translated.
    }

    /// `WheelFlow` (com/thomas7520/macrokeybinds/util/WheelFlow.java)
    pub mod wheel_flow {
        use super::*;
        pub static GSON: LazyLock<JGson> = LazyLock::new(|| JGson { nulls: true, ..(JGson { pretty: true, ..JGson::default() }) });
        #[derive(Clone, Debug)]
        pub struct WheelFlow {
        }
        impl WheelFlow {
            pub fn new() -> Self {
                let mut s = Self {};
                s
            }
        }
        // WheelFlow.getWheel(0): not translated, it creates a Wheel with a constructor that isn't translated.
        // WheelFlow.writeWheel(1): not translated, a try-with-resources without a catch (its errors go up).
        // WheelFlow.getWheelFile(0): not translated, it uses Services.PLATFORM, which isn't translated (it uses IPlatformHelper, which isn't translated (it's an interface)).
    }

    /// `ButtonImageWidget` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java)
    pub mod button_image_widget {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractButton`. Its static parts are below.
        pub const DEFAULT_WIDTH_SMALL: i32 = 120;
        pub const DEFAULT_WIDTH: i32 = 150;
        pub const DEFAULT_HEIGHT: i32 = 20;
        pub const FIELD_46856: i32 = 8;
        // DEFAULT_NARRATION_SUPPLIER: not translated, it uses NarrationSupplier, which isn't translated (it's an interface).
        // ButtonImageWidget.builder(2): not translated, it uses Minecraft's `Component`.
        // ButtonImageWidget.ButtonImageWidget(8): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // ButtonImageWidget.createNarrationMessage(0): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // ButtonImageWidget.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // ButtonImageWidget.onPress(1): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // ButtonImageWidget.extractContents(4): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
    }

    /// `ButtonImageWidget.Builder` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java)
    pub mod button_image_widget_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // ButtonImageWidget.Builder.Builder(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.position(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.width(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.size(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.dimensions(4): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.icon(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.tooltip(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.narrationSupplier(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // ButtonImageWidget.Builder.build(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `ButtonImageWidget.NarrationSupplier` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java)
    pub mod button_image_widget_narration_supplier {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ButtonImageWidget.NarrationSupplier.createNarrationMessage(1): not translated, it has no body.
    }

    /// `ButtonImageWidget.PressAction` (com/thomas7520/macrokeybinds/util/widget/ButtonImageWidget.java)
    pub mod button_image_widget_press_action {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // ButtonImageWidget.PressAction.onPress(1): not translated, it has no body.
    }

    /// `CheckboxEdited` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java)
    pub mod checkbox_edited {
        use super::*;
        // The class itself isn't translated: it extends Minecraft's `AbstractButton`. Its static parts are below.
        pub static SELECTED_HIGHLIGHTED_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "widget/checkbox_selected_highlighted"));
        pub static SELECTED_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "widget/checkbox_selected"));
        pub static HIGHLIGHTED_TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "widget/checkbox_highlighted"));
        pub static TEXTURE: LazyLock<JIdentifier> = LazyLock::new(|| JIdentifier::new("minecraft", "widget/checkbox"));
        pub const TEXT_COLOR: i32 = 14737632;
        pub const FIELD_47105: i32 = 4;
        pub const FIELD_47106: i32 = 8;
        // CheckboxEdited.CheckboxEdited(6): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // CheckboxEdited.builder(2): not translated, it uses Minecraft's `Component`.
        // CheckboxEdited.getSize(1): needs the game, so it's in game_side.rs.
        // CheckboxEdited.isChecked(0): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // CheckboxEdited.updateWidgetNarration(1): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // CheckboxEdited.onPress(1): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
        // CheckboxEdited.extractContents(4): not translated, its class isn't translated (it extends Minecraft's `AbstractButton`).
    }

    /// `CheckboxEdited.Builder` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java)
    pub mod checkbox_edited_builder {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Component`. Its static parts are below.
        // CheckboxEdited.Builder.Builder(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CheckboxEdited.Builder.pos(2): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CheckboxEdited.Builder.callback(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CheckboxEdited.Builder.checked(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CheckboxEdited.Builder.option(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CheckboxEdited.Builder.tooltip(1): not translated, its class isn't translated (it uses Minecraft's `Component`).
        // CheckboxEdited.Builder.build(0): not translated, its class isn't translated (it uses Minecraft's `Component`).
    }

    /// `CheckboxEdited.Callback` (com/thomas7520/macrokeybinds/util/widget/CheckboxEdited.java)
    pub mod checkbox_edited_callback {
        use super::*;
        // The class itself isn't translated: it's an interface. Its static parts are below.
        // CheckboxEdited.Callback.onValueChange(2): not translated, it has no body.
    }

    /// `MacroCMDSuggestor` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java)
    pub mod macro_cmd_suggestor {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Minecraft`. Its static parts are below.
        pub static WHITESPACE_PATTERN: LazyLock<JRegex> = LazyLock::new(|| JRegex::new("(\\s+)"));
        // ERROR_STYLE: not translated, it uses Minecraft's `Style`.
        // INFO_STYLE: not translated, it uses Minecraft's `Style`.
        // HIGHLIGHT_STYLES: not translated, it uses Minecraft's `Style`.
        // MacroCMDSuggestor.MacroCMDSuggestor(9): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.setWindowActive(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.setCanLeave(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.keyPressed(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.mouseScrolled(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.mouseClicked(3): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.show(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.isOpen(0): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.getSuggestionUsageNarrationText(0): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.clearWindow(0): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.sortSuggestions(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.refresh(0): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.getStartOfCurrentWord(1): not translated, it calls `Strings` (com.google.common.base).isNullOrEmpty.
        // MacroCMDSuggestor.formatException(1): not translated, it uses Minecraft's `FormattedCharSequence`.
        // MacroCMDSuggestor.showCommandSuggestions(0): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.showUsages(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.provideRenderText(2): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        pub fn get_suggestion_suffix(original: &str, suggestion: &str) -> Option<String> {
            return if suggestion.starts_with(&original) { Some(j_substring(&suggestion, j_len(&original), None)) } else { None };
        }
        // MacroCMDSuggestor.highlight(3): not translated, it uses Minecraft's `FormattedCharSequence`.
        // MacroCMDSuggestor.render(3): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.tryRenderWindow(3): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.renderMessages(1): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
        // MacroCMDSuggestor.getNarration(0): not translated, its class isn't translated (it uses Minecraft's `Minecraft`).
    }

    /// `MacroCMDSuggestor.SuggestionWindow` (com/thomas7520/macrokeybinds/util/widget/MacroCMDSuggestor.java)
    pub mod macro_cmd_suggestor_suggestion_window {
        use super::*;
        // The class itself isn't translated: it uses Minecraft's `Rect2i`. Its static parts are below.
        // MacroCMDSuggestor.SuggestionWindow.SuggestionWindow(5): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.render(3): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.mouseClicked(3): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.mouseScrolled(1): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.keyPressed(1): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.scroll(1): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.select(1): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.complete(0): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
        // MacroCMDSuggestor.SuggestionWindow.getNarration(0): not translated, its class isn't translated (it uses Minecraft's `Rect2i`).
    }

    
    // ---- Java regular expressions (java.util.regex), a small backtracking engine ----
    // Bundles can't add the regex crate, so this covers what mods use: literals, `.`, classes with ranges and
    // \d \s \w, groups (capturing, `(?:`, `(?<name>`), lookahead, alternation, greedy/lazy quantifiers, ^ $ \b.
    // Positions are in chars, like the translated String methods.
    
    #[derive(Clone, Debug)]
    enum JClassItem {
        Range(char, char),
        Digit(bool),
        Space(bool),
        Word(bool),
    }
    
    #[derive(Clone, Debug)]
    enum JNode {
        Char(char),
        Any,
        Class(Vec<JClassItem>, bool),
        Start,
        End,
        WordB(bool),
        Group(Box<JNode>, Option<usize>),
        Look(Box<JNode>, bool),
        Alt(Vec<JNode>),
        Concat(Vec<JNode>),
        Repeat(Box<JNode>, u32, u32, bool),
        Backref(usize),
    }
    
    type JCaps = Vec<Option<(usize, usize)>>;
    
    /// A compiled `Pattern`.
    #[derive(Clone, Debug)]
    pub struct JRegex {
        node: JNode,
        groups: usize,
        names: Vec<(String, usize)>,
        ci: bool,
    }
    
    fn j_is_word(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }
    
    fn j_is_space(c: char) -> bool {
        matches!(c, ' ' | '\t' | '\n' | '\u{0B}' | '\u{0C}' | '\r')
    }
    
    struct JRegexParser<'a> {
        p: &'a [char],
        i: usize,
        groups: usize,
        names: Vec<(String, usize)>,
    }
    
    impl JRegexParser<'_> {
        fn peek(&self) -> Option<char> {
            self.p.get(self.i).copied()
        }
        fn eat(&mut self, c: char) -> bool {
            if self.peek() == Some(c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn alt(&mut self) -> JNode {
            let mut arms = vec![self.concat()];
            while self.eat('|') {
                arms.push(self.concat());
            }
            if arms.len() == 1 { arms.pop().unwrap() } else { JNode::Alt(arms) }
        }
        fn concat(&mut self) -> JNode {
            let mut items = Vec::new();
            while let Some(c) = self.peek() {
                if c == '|' || c == ')' {
                    break;
                }
                let atom = self.atom();
                items.push(self.quantified(atom));
            }
            JNode::Concat(items)
        }
        fn number(&mut self) -> Option<u32> {
            let start = self.i;
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.i += 1;
            }
            if start == self.i { None } else { self.p[start..self.i].iter().collect::<String>().parse().ok() }
        }
        fn quantified(&mut self, atom: JNode) -> JNode {
            let (min, max) = match self.peek() {
                Some('*') => (0, u32::MAX),
                Some('+') => (1, u32::MAX),
                Some('?') => (0, 1),
                Some('{') => {
                    let save = self.i;
                    self.i += 1;
                    let Some(lo) = self.number() else {
                        self.i = save;
                        return atom;
                    };
                    let hi = if self.eat(',') { self.number().unwrap_or(u32::MAX) } else { lo };
                    if !self.eat('}') {
                        panic!("bad repetition in regex");
                    }
                    self.i -= 1;
                    (lo, hi)
                }
                _ => return atom,
            };
            self.i += 1;
            let greedy = !self.eat('?');
            if greedy {
                self.eat('+');
            }
            self.quantified(JNode::Repeat(Box::new(atom), min, max, greedy))
        }
        fn escape(&mut self, in_class: bool) -> JClassItem {
            let c = self.peek().expect("regex ends with a backslash");
            self.i += 1;
            let lit = |c: char| JClassItem::Range(c, c);
            match c {
                'd' => JClassItem::Digit(true),
                'D' => JClassItem::Digit(false),
                's' => JClassItem::Space(true),
                'S' => JClassItem::Space(false),
                'w' => JClassItem::Word(true),
                'W' => JClassItem::Word(false),
                't' => lit('\t'),
                'n' => lit('\n'),
                'r' => lit('\r'),
                'f' => lit('\u{0C}'),
                'e' => lit('\u{1B}'),
                '0' => lit('\0'),
                'u' => {
                    let hex: String = self.p[self.i..self.i + 4].iter().collect();
                    self.i += 4;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\u escape")).unwrap_or('\u{FFFD}'))
                }
                'x' => {
                    let hex: String = self.p[self.i..self.i + 2].iter().collect();
                    self.i += 2;
                    lit(char::from_u32(u32::from_str_radix(&hex, 16).expect("bad \\x escape")).unwrap_or('\u{FFFD}'))
                }
                c if !in_class && c.is_ascii_alphabetic() => panic!("unsupported regex escape \\{c}"),
                c => lit(c),
            }
        }
        fn class(&mut self) -> JNode {
            let negated = self.eat('^');
            let mut items = Vec::new();
            let mut first = true;
            loop {
                let c = self.peek().expect("unclosed [ in regex");
                if c == ']' && !first {
                    self.i += 1;
                    break;
                }
                first = false;
                if c == '[' {
                    panic!("nested character classes aren't supported");
                }
                self.i += 1;
                let item = if c == '\\' { self.escape(true) } else { JClassItem::Range(c, c) };
                // a-z
                if let JClassItem::Range(lo, _) = item
                    && self.peek() == Some('-')
                    && self.p.get(self.i + 1).is_some_and(|&n| n != ']')
                {
                    self.i += 1;
                    let hc = self.peek().unwrap();
                    self.i += 1;
                    let hi = if hc == '\\' {
                        match self.escape(true) {
                            JClassItem::Range(h, _) => h,
                            _ => panic!("bad range in regex"),
                        }
                    } else {
                        hc
                    };
                    items.push(JClassItem::Range(lo, hi));
                    continue;
                }
                items.push(item);
            }
            JNode::Class(items, negated)
        }
        fn atom(&mut self) -> JNode {
            let c = self.peek().unwrap();
            self.i += 1;
            match c {
                '.' => JNode::Any,
                '^' => JNode::Start,
                '$' => JNode::End,
                '[' => self.class(),
                '(' => {
                    let mut index = None;
                    let mut look = None;
                    if self.eat('?') {
                        if self.eat(':') {
                        } else if self.eat('=') {
                            look = Some(false);
                        } else if self.eat('!') {
                            look = Some(true);
                        } else if self.eat('<') {
                            let start = self.i;
                            while self.peek().is_some_and(|c| c != '>') {
                                self.i += 1;
                            }
                            let name: String = self.p[start..self.i].iter().collect();
                            self.i += 1;
                            self.groups += 1;
                            self.names.push((name, self.groups));
                            index = Some(self.groups);
                        } else {
                            panic!("unsupported regex group (?{:?}", self.peek());
                        }
                    } else {
                        self.groups += 1;
                        index = Some(self.groups);
                    }
                    let inner = self.alt();
                    if !self.eat(')') {
                        panic!("unclosed ( in regex");
                    }
                    match look {
                        Some(neg) => JNode::Look(Box::new(inner), neg),
                        None => JNode::Group(Box::new(inner), index),
                    }
                }
                '\\' => match self.peek() {
                    Some('b') => {
                        self.i += 1;
                        JNode::WordB(true)
                    }
                    Some('B') => {
                        self.i += 1;
                        JNode::WordB(false)
                    }
                    Some('A') => {
                        self.i += 1;
                        JNode::Start
                    }
                    Some('z') | Some('Z') => {
                        self.i += 1;
                        JNode::End
                    }
                    Some(d) if d.is_ascii_digit() && d != '0' => {
                        self.i += 1;
                        JNode::Backref(d as usize - '0' as usize)
                    }
                    Some('Q') => {
                        self.i += 1;
                        let mut lits = Vec::new();
                        while self.i < self.p.len() && !(self.p[self.i] == '\\' && self.p.get(self.i + 1) == Some(&'E')) {
                            lits.push(JNode::Char(self.p[self.i]));
                            self.i += 1;
                        }
                        self.i = (self.i + 2).min(self.p.len());
                        JNode::Concat(lits)
                    }
                    _ => match self.escape(false) {
                        JClassItem::Range(c, _) => JNode::Char(c),
                        other => JNode::Class(vec![other], false),
                    },
                },
                c => JNode::Char(c),
            }
        }
    }
    
    impl JRegex {
        /// `Pattern.compile(p)`. Java throws PatternSyntaxException on a bad pattern; this panics.
        pub fn new(p: &str) -> Self {
            Self::with_flags(p, 0)
        }
        /// `Pattern.compile(p, flags)`: CASE_INSENSITIVE (2) is honoured.
        pub fn with_flags(p: &str, flags: i32) -> Self {
            let chars: Vec<char> = p.chars().collect();
            let (chars, ci) = match chars.strip_prefix(&['(', '?', 'i', ')']) {
                Some(rest) => (rest.to_vec(), true),
                None => (chars, flags & 2 != 0),
            };
            let mut parser = JRegexParser { p: &chars, i: 0, groups: 0, names: Vec::new() };
            let node = parser.alt();
            if parser.i != chars.len() {
                panic!("unmatched ) in regex {p:?}");
            }
            JRegex { node, groups: parser.groups, names: parser.names, ci }
        }
    
        fn same(&self, a: char, b: char) -> bool {
            a == b || self.ci && (j_lower_char(a) == j_lower_char(b) || j_upper_char(a) == j_upper_char(b))
        }
    
        fn in_class(&self, items: &[JClassItem], c: char) -> bool {
            items.iter().any(|it| match *it {
                JClassItem::Range(lo, hi) => {
                    (lo..=hi).contains(&c) || self.ci && ((lo..=hi).contains(&j_lower_char(c)) || (lo..=hi).contains(&j_upper_char(c)))
                }
                JClassItem::Digit(yes) => c.is_ascii_digit() == yes,
                JClassItem::Space(yes) => j_is_space(c) == yes,
                JClassItem::Word(yes) => j_is_word(c) == yes,
            })
        }
    
        fn m(&self, n: &JNode, s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match n {
                JNode::Char(c) => i < s.len() && self.same(s[i], *c) && k(i + 1, caps),
                JNode::Any => i < s.len() && !matches!(s[i], '\n' | '\r' | '\u{85}' | '\u{2028}' | '\u{2029}') && k(i + 1, caps),
                JNode::Class(items, neg) => i < s.len() && self.in_class(items, s[i]) != *neg && k(i + 1, caps),
                JNode::Start => i == 0 && k(i, caps),
                JNode::End => (i == s.len() || i + 1 == s.len() && s[i] == '\n') && k(i, caps),
                JNode::WordB(yes) => {
                    let before = i > 0 && j_is_word(s[i - 1]);
                    let after = i < s.len() && j_is_word(s[i]);
                    (before != after) == *yes && k(i, caps)
                }
                JNode::Group(inner, index) => match *index {
                    None => self.m(inner, s, i, caps, k),
                    Some(g) => self.m(inner, s, i, caps, &mut |j, caps: &mut JCaps| {
                        let prev = caps[g];
                        caps[g] = Some((i, j));
                        if k(j, caps) {
                            return true;
                        }
                        caps[g] = prev;
                        false
                    }),
                },
                JNode::Look(inner, neg) => {
                    let mut probe = caps.clone();
                    let found = self.m(inner, s, i, &mut probe, &mut |_, _| true);
                    if found == *neg {
                        return false;
                    }
                    if !neg {
                        *caps = probe;
                    }
                    k(i, caps)
                }
                JNode::Alt(arms) => arms.iter().any(|a| self.m(a, s, i, caps, k)),
                JNode::Concat(items) => self.seq(items, s, i, caps, k),
                JNode::Repeat(inner, min, max, greedy) => self.rep(inner, *min, *max, *greedy, 0, s, i, caps, k),
                JNode::Backref(g) => match caps.get(*g).copied().flatten() {
                    Some((a, b)) => {
                        let len = b - a;
                        i + len <= s.len() && (0..len).all(|d| self.same(s[a + d], s[i + d])) && k(i + len, caps)
                    }
                    None => false,
                },
            }
        }
    
        fn seq(&self, items: &[JNode], s: &[char], i: usize, caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            match items.split_first() {
                None => k(i, caps),
                Some((first, rest)) => self.m(first, s, i, caps, &mut |j, caps: &mut JCaps| self.seq(rest, s, j, caps, k)),
            }
        }
    
        #[allow(clippy::too_many_arguments)]
        fn rep(&self, n: &JNode, min: u32, max: u32, greedy: bool, count: u32, s: &[char], i: usize, caps: &mut JCaps,
               k: &mut dyn FnMut(usize, &mut JCaps) -> bool) -> bool {
            let more = |caps: &mut JCaps, k: &mut dyn FnMut(usize, &mut JCaps) -> bool| {
                count < max
                    && self.m(n, s, i, caps, &mut |j, caps: &mut JCaps| {
                        // An empty pass can't make progress: stop repeating once the minimum is met.
                        (j != i || count < min) && self.rep(n, min, max, greedy, count + 1, s, j, caps, k)
                    })
            };
            if count < min {
                return more(caps, k);
            }
            if greedy { more(caps, k) || k(i, caps) } else { k(i, caps) || more(caps, k) }
        }
    
        /// A match starting exactly at `i`; `whole`: it must also end at the end of the text.
        fn at(&self, s: &[char], i: usize, whole: bool) -> Option<JCaps> {
            let mut caps: JCaps = vec![None; self.groups + 1];
            let mut end = None;
            let ok = self.m(&self.node, s, i, &mut caps, &mut |j, _| {
                if whole && j != s.len() {
                    return false;
                }
                end = Some(j);
                true
            });
            if !ok {
                return None;
            }
            caps[0] = Some((i, end.unwrap()));
            Some(caps)
        }
    
        fn find_from(&self, s: &[char], from: usize) -> Option<JCaps> {
            (from..=s.len()).find_map(|i| self.at(s, i, false))
        }
    
        /// `String.matches` / `Pattern.matches`: the whole text.
        pub fn matches(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.at(&cs, 0, true).is_some()
        }
    
        /// `Pattern.matcher(s).find()` as a test.
        pub fn is_found(&self, s: &str) -> bool {
            let cs: Vec<char> = s.chars().collect();
            self.find_from(&cs, 0).is_some()
        }
    
        fn expand(&self, s: &[char], caps: &JCaps, rep: &str, out: &mut String) {
            let r: Vec<char> = rep.chars().collect();
            let mut i = 0;
            while i < r.len() {
                match r[i] {
                    '\\' if i + 1 < r.len() => {
                        out.push(r[i + 1]);
                        i += 2;
                    }
                    '$' if i + 1 < r.len() && r[i + 1].is_ascii_digit() => {
                        let g = r[i + 1] as usize - '0' as usize;
                        if let Some((a, b)) = caps.get(g).copied().flatten() {
                            out.extend(&s[a..b]);
                        } else if g > self.groups {
                            panic!("No group {g}");
                        }
                        i += 2;
                    }
                    '$' if r.get(i + 1) == Some(&'{') => {
                        let close = r[i..].iter().position(|&c| c == '}').expect("bad ${name} in replacement") + i;
                        let name: String = r[i + 2..close].iter().collect();
                        let g = self.names.iter().find(|(n, _)| *n == name).map(|(_, g)| *g).expect("no group with that name");
                        if let Some((a, b)) = caps[g] {
                            out.extend(&s[a..b]);
                        }
                        i = close + 1;
                    }
                    c => {
                        out.push(c);
                        i += 1;
                    }
                }
            }
        }
    
        fn replace(&self, s: &str, rep: &str, all: bool) -> String {
            let cs: Vec<char> = s.chars().collect();
            let mut out = String::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                out.extend(&cs[pos..a]);
                self.expand(&cs, &caps, rep, &mut out);
                pos = b;
                from = if b == a { b + 1 } else { b };
                if b == a && a < cs.len() {
                    out.push(cs[a]);
                    pos = a + 1;
                }
                if !all {
                    break;
                }
            }
            if pos <= cs.len() {
                out.extend(&cs[pos..]);
            }
            out
        }
    
        /// `String.replaceAll` / `Matcher.replaceAll`, with Java's `$1` and `\$` in the replacement.
        pub fn replace_all(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, true)
        }
    
        pub fn replace_first(&self, s: &str, rep: &str) -> String {
            self.replace(s, rep, false)
        }
    
        /// `String.split(regex, limit)` / `Pattern.split`, with Java's rules for empty parts.
        pub fn split(&self, s: &str, limit: i32) -> Vec<String> {
            let cs: Vec<char> = s.chars().collect();
            let mut parts: Vec<String> = Vec::new();
            let (mut pos, mut from) = (0, 0);
            while from <= cs.len() {
                if limit > 0 && parts.len() as i32 == limit - 1 {
                    break;
                }
                let Some(caps) = self.find_from(&cs, from) else { break };
                let (a, b) = caps[0].unwrap();
                if b == 0 {
                    // A zero-width match at the start makes no leading empty part.
                    from = 1;
                    continue;
                }
                if a == b && a >= cs.len() {
                    break;
                }
                parts.push(cs[pos..a].iter().collect());
                pos = b;
                from = if a == b { b + 1 } else { b };
            }
            parts.push(cs[pos.min(cs.len())..].iter().collect());
            if limit == 0 {
                while parts.len() > 1 && parts.last().is_some_and(|p| p.is_empty()) {
                    parts.pop();
                }
                if parts.len() == 1 && parts[0].is_empty() && !s.is_empty() {
                    parts.clear();
                }
            }
            parts
        }
    }
    
    /// A `Matcher`: a regex over one text, with the last match.
    #[derive(Clone, Debug)]
    pub struct JMatcher {
        re: JRegex,
        text: Vec<char>,
        from: usize,
        caps: Option<JCaps>,
    }
    
    impl JMatcher {
        pub fn new(re: &JRegex, s: &str) -> Self {
            JMatcher { re: re.clone(), text: s.chars().collect(), from: 0, caps: None }
        }
        pub fn find(&mut self) -> bool {
            if self.from > self.text.len() {
                self.caps = None;
                return false;
            }
            self.caps = self.re.find_from(&self.text, self.from);
            if let Some((a, b)) = self.caps.as_ref().and_then(|c| c[0]) {
                self.from = if a == b { b + 1 } else { b };
                true
            } else {
                self.from = self.text.len() + 1;
                false
            }
        }
        pub fn find_at(&mut self, start: i32) -> bool {
            self.from = start as usize;
            self.find()
        }
        pub fn matches(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, true);
            self.caps.is_some()
        }
        pub fn looking_at(&mut self) -> bool {
            self.caps = self.re.at(&self.text, 0, false);
            self.caps.is_some()
        }
        fn span(&self, g: usize) -> Option<(usize, usize)> {
            let caps = self.caps.as_ref().expect("No match found");
            assert!(g < caps.len(), "No group {g}");
            caps[g]
        }
        /// `group(n)`: null (None) when that group took no part in the match.
        pub fn group(&self, g: i32) -> Option<String> {
            self.span(g as usize).map(|(a, b)| self.text[a..b].iter().collect())
        }
        pub fn group_named(&self, name: &str) -> Option<String> {
            let g = self.re.names.iter().find(|(n, _)| n == name).map(|(_, g)| *g).expect("No group with that name");
            self.group(g as i32)
        }
        pub fn start(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(a, _)| a as i32)
        }
        pub fn end(&self, g: i32) -> i32 {
            self.span(g as usize).map_or(-1, |(_, b)| b as i32)
        }
        pub fn group_count(&self) -> i32 {
            self.re.groups as i32
        }
        pub fn replace_all(&self, rep: &str) -> String {
            self.re.replace_all(&self.text.iter().collect::<String>(), rep)
        }
    }
    
    #[cfg(test)]
    mod jregex_tests {
        use super::*;
    
        #[test]
        fn groups_and_lazy() {
            let re = JRegex::new("^(?:\\S )?(?:VERY SPECIAL|COMMON|RARE|EPIC)(?: DUNGEON)?(?: (.+?))?(?: \\S)?$");
            let mut m = JMatcher::new(&re, "EPIC DUNGEON SWORD");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("SWORD"));
            let mut m = JMatcher::new(&re, "a RARE FISHING ROD b");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("FISHING ROD"));
            let mut m = JMatcher::new(&re, "COMMON");
            assert!(m.matches());
            assert_eq!(m.group(1), None);
        }
    
        #[test]
        fn classes_counts_find() {
            let re = JRegex::new("server/[0-9A-Fa-f]{8}/(.+)");
            let mut m = JMatcher::new(&re, "server/0aF3b2c1/pack.zip");
            assert!(m.matches());
            assert_eq!(m.group(1).as_deref(), Some("pack.zip"));
            assert!(!JRegex::new("server/[0-9A-Fa-f]{8}/(.+)").matches("server/xyz/a"));
            let re = JRegex::new("\"id\"\\s*:\\s*\"([a-z0-9_.-]+:[a-z0-9_./-]+)\"");
            let mut m = JMatcher::new(&re, "{\"id\": \"minecraft:item/a\", \"id\":\"x:y\"}");
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("minecraft:item/a"));
            assert!(m.find());
            assert_eq!(m.group(1).as_deref(), Some("x:y"));
            assert!(!m.find());
        }
    
        #[test]
        fn split_and_replace() {
            assert_eq!(JRegex::new("\\s*,\\s*").split("a , b,c,,", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new(",").split("a,b,c", 2), vec!["a", "b,c"]);
            assert_eq!(JRegex::new("").split("abc", 0), vec!["a", "b", "c"]);
            assert_eq!(JRegex::new("(\\w+)@(\\w+)").replace_all("x a@b y c@d", "$2.$1"), "x b.a y d.c");
            assert_eq!(JRegex::new("a*").replace_all("baaac", "-"), "-b--c-");
            assert!(JRegex::with_flags("abc", 2).matches("AbC"));
        }
    }

    
    // ---- Gson (com.google.gson): JSON values, a reader and Gson's writer ----
    // JsonElement, JsonObject, JsonArray and JsonPrimitive are all one Rust type. Numbers keep their text, as
    // Gson's LazilyParsedNumber does. Objects keep their insertion order, as Gson's LinkedTreeMap does.
    
    #[derive(Clone, Debug, PartialEq)]
    pub enum JJson {
        Null,
        Bool(bool),
        Num(String),
        Str(String),
        Arr(Vec<JJson>),
        Obj(Vec<(String, JJson)>),
    }
    
    /// A float or double as Java prints it (Float.toString / Double.toString).
    pub fn j_java_num(x: f64, float: bool) -> String {
        if x.is_nan() {
            return "NaN".into();
        }
        if x.is_infinite() {
            return if x > 0.0 { "Infinity".into() } else { "-Infinity".into() };
        }
        let a = x.abs();
        if x == 0.0 {
            return if x.is_sign_negative() { "-0.0".into() } else { "0.0".into() };
        }
        let digits = if float { format!("{:e}", x as f32) } else { format!("{:e}", x) };
        let (mant, exp) = digits.split_once('e').unwrap();
        let exp: i32 = exp.parse().unwrap();
        if (1e-3..1e7).contains(&a) {
            let plain = if float { format!("{}", x as f32) } else { format!("{}", x) };
            if plain.contains('.') { plain } else { plain + ".0" }
        } else {
            let mant = if mant.contains('.') { mant.to_string() } else { format!("{mant}.0") };
            format!("{mant}E{exp}")
        }
    }
    
    struct JJsonReader<'a> {
        s: &'a [char],
        i: usize,
    }
    
    impl JJsonReader<'_> {
        fn ws(&mut self) {
            while self.i < self.s.len() && self.s[self.i].is_whitespace() {
                self.i += 1;
            }
        }
        fn fail(&self, what: &str) -> ! {
            panic!("JsonSyntaxException: {what} at {}", self.i)
        }
        fn value(&mut self) -> JJson {
            self.ws();
            match self.s.get(self.i) {
                Some('{') => {
                    self.i += 1;
                    let mut members = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&'}') {
                        self.i += 1;
                        return JJson::Obj(members);
                    }
                    loop {
                        self.ws();
                        let k = match self.s.get(self.i) {
                            Some('"') | Some('\'') => self.string(),
                            _ => self.bare(),
                        };
                        self.ws();
                        if !matches!(self.s.get(self.i), Some(':') | Some('=')) {
                            self.fail("expected ':'");
                        }
                        self.i += 1;
                        let v = self.value();
                        // A repeated name: Gson keeps the last one.
                        members.retain(|(n, _): &(String, JJson)| *n != k);
                        members.push((k, v));
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some('}') => {
                                self.i += 1;
                                return JJson::Obj(members);
                            }
                            _ => self.fail("expected ',' or '}'"),
                        }
                    }
                }
                Some('[') => {
                    self.i += 1;
                    let mut items = Vec::new();
                    self.ws();
                    if self.s.get(self.i) == Some(&']') {
                        self.i += 1;
                        return JJson::Arr(items);
                    }
                    loop {
                        items.push(self.value());
                        self.ws();
                        match self.s.get(self.i) {
                            Some(',') | Some(';') => self.i += 1,
                            Some(']') => {
                                self.i += 1;
                                return JJson::Arr(items);
                            }
                            _ => self.fail("expected ',' or ']'"),
                        }
                    }
                }
                Some('"') | Some('\'') => JJson::Str(self.string()),
                Some(_) => {
                    let word = self.bare();
                    match word.as_str() {
                        "true" => JJson::Bool(true),
                        "false" => JJson::Bool(false),
                        "null" => JJson::Null,
                        w if w.parse::<f64>().is_ok() => JJson::Num(w.to_string()),
                        // Gson is lenient: an unquoted word is a string.
                        w => JJson::Str(w.to_string()),
                    }
                }
                None => self.fail("end of input"),
            }
        }
        fn bare(&mut self) -> String {
            let start = self.i;
            while self.i < self.s.len() && !matches!(self.s[self.i], ',' | ':' | '=' | ';' | '}' | ']' | '{' | '[' | '"' | '\'') && !self.s[self.i].is_whitespace() {
                self.i += 1;
            }
            if start == self.i {
                self.fail("expected a value");
            }
            self.s[start..self.i].iter().collect()
        }
        fn string(&mut self) -> String {
            let q = self.s[self.i];
            self.i += 1;
            let mut out = String::new();
            loop {
                let Some(&c) = self.s.get(self.i) else { self.fail("unterminated string") };
                self.i += 1;
                match c {
                    c if c == q => return out,
                    '\\' => {
                        let Some(&e) = self.s.get(self.i) else { self.fail("unterminated string") };
                        self.i += 1;
                        match e {
                            'n' => out.push('\n'),
                            't' => out.push('\t'),
                            'r' => out.push('\r'),
                            'b' => out.push('\u{8}'),
                            'f' => out.push('\u{c}'),
                            'u' => {
                                let hex: String = self.s[self.i..(self.i + 4).min(self.s.len())].iter().collect();
                                self.i += 4;
                                let unit = u16::from_str_radix(&hex, 16).unwrap_or_else(|_| self.fail("bad \\u escape"));
                                // A surrogate pair: two escapes make one char.
                                if (0xD800..0xDC00).contains(&unit) && self.s.get(self.i) == Some(&'\\') && self.s.get(self.i + 1) == Some(&'u') {
                                    let hex2: String = self.s[self.i + 2..(self.i + 6).min(self.s.len())].iter().collect();
                                    if let Ok(low) = u16::from_str_radix(&hex2, 16) {
                                        self.i += 6;
                                        out.extend(char::decode_utf16([unit, low]).map(|r| r.unwrap_or('\u{FFFD}')));
                                        continue;
                                    }
                                }
                                out.push(char::from_u32(unit as u32).unwrap_or('\u{FFFD}'));
                            }
                            other => out.push(other),
                        }
                    }
                    c => out.push(c),
                }
            }
        }
    }
    
    impl JJson {
        /// JsonParser.parseString: panics (Java throws JsonSyntaxException) on bad JSON.
        pub fn parse(s: &str) -> JJson {
            let chars: Vec<char> = s.chars().collect();
            let mut r = JJsonReader { s: &chars, i: 0 };
            let v = r.value();
            r.ws();
            if r.i != chars.len() {
                r.fail("more after the value");
            }
            v
        }
        pub fn object() -> JJson {
            JJson::Obj(Vec::new())
        }
        pub fn array() -> JJson {
            JJson::Arr(Vec::new())
        }
        pub fn int(v: i64) -> JJson {
            JJson::Num(v.to_string())
        }
        pub fn float(v: f32) -> JJson {
            JJson::Num(j_java_num(v as f64, true))
        }
        pub fn double(v: f64) -> JJson {
            JJson::Num(j_java_num(v, false))
        }
        pub fn str(v: &str) -> JJson {
            JJson::Str(v.to_string())
        }
        pub fn is_json_object(&self) -> bool {
            matches!(self, JJson::Obj(_))
        }
        pub fn is_json_array(&self) -> bool {
            matches!(self, JJson::Arr(_))
        }
        pub fn is_json_null(&self) -> bool {
            matches!(self, JJson::Null)
        }
        pub fn is_json_primitive(&self) -> bool {
            matches!(self, JJson::Bool(_) | JJson::Num(_) | JJson::Str(_))
        }
        pub fn is_string(&self) -> bool {
            matches!(self, JJson::Str(_))
        }
        pub fn is_number(&self) -> bool {
            matches!(self, JJson::Num(_))
        }
        pub fn is_boolean(&self) -> bool {
            matches!(self, JJson::Bool(_))
        }
        /// getAsJsonObject() / getAsJsonArray(): Java throws IllegalStateException on the wrong kind.
        pub fn as_object(&self) -> JJson {
            assert!(self.is_json_object(), "Not a JSON Object: {self}");
            self.clone()
        }
        pub fn as_array(&self) -> JJson {
            assert!(self.is_json_array(), "Not a JSON Array: {self}");
            self.clone()
        }
        fn members(&self) -> &Vec<(String, JJson)> {
            match self {
                JJson::Obj(m) => m,
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        /// The (name, value) pairs of an object, as entrySet() walks them.
        pub fn entries(&self) -> impl Iterator<Item = (&String, &JJson)> {
            self.members().iter().map(|(k, v)| (k, v))
        }
        pub fn keys(&self) -> impl Iterator<Item = &String> {
            self.members().iter().map(|(k, _)| k)
        }
        pub fn elements(&self) -> impl Iterator<Item = &JJson> {
            match self {
                JJson::Arr(a) => a.iter(),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn has(&self, k: &str) -> bool {
            self.members().iter().any(|(n, _)| n == k)
        }
        /// JsonObject.get: None (null) when absent.
        pub fn get(&self, k: &str) -> Option<JJson> {
            self.members().iter().find(|(n, _)| n == k).map(|(_, v)| v.clone())
        }
        /// JsonObject.getAsJsonObject(name) / getAsJsonArray(name): null when absent, a cast that throws on the wrong kind.
        pub fn member_object(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_object())
        }
        pub fn member_array(&self, k: &str) -> Option<JJson> {
            self.get(k).map(|v| v.as_array())
        }
        /// JsonArray.get(i).
        pub fn at(&self, i: i32) -> JJson {
            match self {
                JJson::Arr(a) => a.get(i as usize).cloned().unwrap_or_else(|| panic!("Index {i} out of bounds for length {}", a.len())),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        pub fn size(&self) -> i32 {
            match self {
                JJson::Arr(a) => a.len() as i32,
                JJson::Obj(m) => m.len() as i32,
                _ => panic!("size() of a JSON primitive"),
            }
        }
        pub fn is_empty(&self) -> bool {
            self.size() == 0
        }
        /// The single value of a one-element array, as Gson's JsonArray.getAsX does.
        fn single(&self) -> &JJson {
            match self {
                JJson::Arr(a) if a.len() == 1 => &a[0],
                JJson::Arr(_) => panic!("Array must have size 1"),
                other => other,
            }
        }
        pub fn as_string(&self) -> String {
            match self.single() {
                JJson::Str(s) => s.clone(),
                JJson::Num(n) => n.clone(),
                JJson::Bool(b) => b.to_string(),
                other => panic!("UnsupportedOperationException: {other} as a string"),
            }
        }
        fn number_text(&self) -> String {
            match self.single() {
                JJson::Num(n) | JJson::Str(n) => n.trim().to_string(),
                other => panic!("UnsupportedOperationException: {other} as a number"),
            }
        }
        pub fn as_f64(&self) -> f64 {
            let t = self.number_text();
            t.parse().unwrap_or_else(|_| panic!("NumberFormatException: {t}"))
        }
        pub fn as_float(&self) -> f32 {
            self.as_f64() as f32
        }
        pub fn as_double(&self) -> f64 {
            self.as_f64()
        }
        /// getAsInt / getAsLong: a whole number, else the number cut to one.
        pub fn as_long(&self) -> i64 {
            let t = self.number_text();
            t.parse::<i64>().unwrap_or_else(|_| t.parse::<f64>().map(|f| f as i64).unwrap_or_else(|_| panic!("NumberFormatException: {t}")))
        }
        pub fn as_int(&self) -> i32 {
            self.as_long() as i32
        }
        pub fn as_bool(&self) -> bool {
            match self.single() {
                JJson::Bool(b) => *b,
                // Gson: Boolean.parseBoolean of the text
                other => other.as_string().eq_ignore_ascii_case("true"),
            }
        }
        pub fn add(&mut self, k: &str, v: JJson) {
            match self {
                JJson::Obj(m) => {
                    m.retain(|(n, _)| n != k);
                    m.push((k.to_string(), v));
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn remove(&mut self, k: &str) -> Option<JJson> {
            match self {
                JJson::Obj(m) => {
                    let i = m.iter().position(|(n, _)| n == k)?;
                    Some(m.remove(i).1)
                }
                _ => panic!("Not a JSON Object: {self}"),
            }
        }
        pub fn push(&mut self, v: JJson) {
            match self {
                JJson::Arr(a) => a.push(v),
                _ => panic!("Not a JSON Array: {self}"),
            }
        }
        fn write(&self, out: &mut String, pretty: bool, html: bool, depth: usize) {
            let nl = |out: &mut String, d: usize| {
                if pretty {
                    out.push('\n');
                    out.push_str(&"  ".repeat(d));
                }
            };
            match self {
                JJson::Null => out.push_str("null"),
                JJson::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
                JJson::Num(n) => out.push_str(n),
                JJson::Str(s) => j_json_quote(s, html, out),
                JJson::Arr(a) => {
                    out.push('[');
                    for (i, v) in a.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !a.is_empty() {
                        nl(out, depth);
                    }
                    out.push(']');
                }
                JJson::Obj(m) => {
                    out.push('{');
                    for (i, (k, v)) in m.iter().enumerate() {
                        if i > 0 {
                            out.push(',');
                        }
                        nl(out, depth + 1);
                        j_json_quote(k, html, out);
                        out.push(':');
                        if pretty {
                            out.push(' ');
                        }
                        v.write(out, pretty, html, depth + 1);
                    }
                    if !m.is_empty() {
                        nl(out, depth);
                    }
                    out.push('}');
                }
            }
        }
    }
    
    fn j_json_quote(s: &str, html: bool, out: &mut String) {
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                '\u{8}' => out.push_str("\\b"),
                '\u{c}' => out.push_str("\\f"),
                '<' | '>' | '&' | '=' | '\'' if html => out.push_str(&format!("\\u{:04x}", c as u32)),
                '\u{2028}' | '\u{2029}' => out.push_str(&format!("\\u{:04x}", c as u32)),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
    }
    
    /// JsonElement.toString: compact, without Gson's HTML escaping.
    impl std::fmt::Display for JJson {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let mut out = String::new();
            self.write(&mut out, false, false, 0);
            f.write_str(&out)
        }
    }
    
    /// A Gson (from `new Gson()` or a GsonBuilder): how it writes.
    #[derive(Clone, Copy, Debug)]
    pub struct JGson {
        pub pretty: bool,
        pub html: bool,
        pub nulls: bool,
    }
    
    impl Default for JGson {
        fn default() -> Self {
            JGson { pretty: false, html: true, nulls: false }
        }
    }
    
    impl JGson {
        /// Gson.toJson.
        pub fn to_json(&self, v: &JJson) -> String {
            let mut out = String::new();
            self.strip(v).write(&mut out, self.pretty, self.html, 0);
            out
        }
        /// Without serializeNulls, Gson leaves out null fields of objects.
        fn strip(&self, v: &JJson) -> JJson {
            match v {
                JJson::Obj(m) => JJson::Obj(m.iter().filter(|(_, x)| self.nulls || !x.is_json_null()).map(|(k, x)| (k.clone(), self.strip(x))).collect()),
                JJson::Arr(a) => JJson::Arr(a.iter().map(|x| self.strip(x)).collect()),
                other => other.clone(),
            }
        }
    }
    
    #[cfg(test)]
    mod jjson_tests {
        use super::*;
    
        #[test]
        fn read_and_write() {
            let j = JJson::parse("{\"a\": 1, \"b\": [1.5, \"x\", true, null], \"c\": {\"d\": -2e3}}");
            assert!(j.has("a") && !j.has("z"));
            assert_eq!(j.get("a").unwrap().as_int(), 1);
            assert_eq!(j.member_array("b").unwrap().at(0).as_float(), 1.5);
            assert_eq!(j.member_array("b").unwrap().size(), 4);
            assert_eq!(j.member_object("c").unwrap().get("d").unwrap().as_double(), -2000.0);
            assert_eq!(j.get("z"), None);
            assert_eq!(j.to_string(), "{\"a\":1,\"b\":[1.5,\"x\",true,null],\"c\":{\"d\":-2e3}}");
            let g = JGson { pretty: true, ..JGson::default() };
            assert_eq!(g.to_json(&JJson::parse("{\"a\":[1,2],\"b\":{},\"n\":null,\"s\":\"<x>\"}")),
                       "{\n  \"a\": [\n    1,\n    2\n  ],\n  \"b\": {},\n  \"s\": \"\\u003cx\\u003e\"\n}");
        }
    
        #[test]
        fn java_numbers() {
            assert_eq!(j_java_num(1.0, true), "1.0");
            assert_eq!(j_java_num(0.1, true), "0.1");
            assert_eq!(j_java_num(1.0e-4, false), "1.0E-4");
            assert_eq!(j_java_num(12345678.0, false), "1.2345678E7");
            assert_eq!(j_java_num(-0.75, false), "-0.75");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_set() {
        assert_eq!(ID, "macrokeybinds");
        assert!(!NAME.is_empty());
        assert!(KEYS.iter().all(|[n, _, d]| !n.is_empty() && d.starts_with("key.")));
    }

    #[test]
    fn macro_store_matches_key_and_queues_actions() {
        let mut store = MacroStore::default();
        let mut macro_def = Macro::new("Hello", "key.keyboard.h");
        macro_def.actions.push(MacroAction::SendMessage("hello".into()));
        macro_def.actions.push(MacroAction::ExecuteCommand("time set day".into()));
        store.add(macro_def);

        assert_eq!(store.trigger_key("key.keyboard.h"), 1);
        assert_eq!(store.pop_action(), Some(MacroAction::SendMessage("hello".into())));
        assert_eq!(store.pop_action(), Some(MacroAction::ExecuteCommand("time set day".into())));
        assert_eq!(store.pop_action(), None);
    }

    #[test]
    fn macro_store_round_trips_json() {
        let text = r#"{"macros":[{"name":"Home","key":"key.keyboard.h","actions":[{"type":"command","value":"tp 0 100 0"},{"type":"local","value":"Returning home"}]}]}"#;
        let store = MacroStore::parse_json(text).unwrap();
        assert_eq!(store.macros.len(), 1);
        assert_eq!(store.macros[0].name, "Home");

        let json = store.to_json();
        let written = json.to_string();
        let reparsed = MacroStore::parse_json(&written).unwrap();
        assert_eq!(reparsed, store);
    }
}

# Launchers

Launchers are the backbone for Sherlock's widget engine. Each displayed widget
is owned by a launcher. For example:

```
[Weather Launcher]
    [Widget] Weather Display
[App Launcher] 
    [Widget] App 1
    [Widget] App 2
    [Widget] App 3
```

A launcher's widgets will share the same behavious based on the shared launcher configuration.

## Shared Launcher Configuration

### Fields

| Field        | Type              | Description                                              |
| ------------ | ----------------- | -------------------------------------------------------- |
| `name`       | `string?`         | Label shown in the UI                                    |
| `alias`      | `string?`         | Keyword that activates focused/alias mode                |
| `home`       | `HomeType`        | Controls home screen visibility                          |
| `priority`   | `integer`         | Sort weight — lower appears first                        |
| `type`       | `LauncherVariant` | Launcher kind (e.g. `app`, `weather`)                   |
| `args`       | `{}` | Launcher-specific configuration options.                   |
| `async`      | `bool`            | Enable async widget updates                              |
| `exit`       | `bool`            | Close Sherlock after execution                           |
| `spawn_focus`| `bool`            | Allow item to receive automatic focus                    |
| `shortcut`   | `bool`            | Show a keybind shortcut for this launcher                |
| `binds`      | `Bind[]?`         | Keyboard shortcuts                                       |
| `actions`    | `Action[]?`       | Context menu actions (overwrites `.desktop` file actions)|
| `add_actions`| `Action[]?`       | Extra actions appended to the context menu               |
| `variables`  | `ExecVariable[]?` | Variables injected into spawned commands                 |

---

### Bind

A keyboard shortcut attached to a launcher's widgets.

| Field      | Type      | Description                                              |
| ---------- | --------- | -------------------------------------------------------- |
| `key`      | `string`  | Key to bind (e.g. `ctrl+c`)                             |
| `callback` | `string`  | Action to invoke (e.g. `inner.copy`, `app_launcher`)    |
| `exit`     | `bool`    | Close Sherlock after this bind executes                  |

---

### Action

A context menu entry attached to a launcher's widgets.

| Field    | Type      | Description                                              |
| -------- | --------- | -------------------------------------------------------- |
| `name`   | `string`  | Label shown in the context menu                         |
| `icon`   | `string?`  | The icon name for the application action                         |
| `method` | `string`  | Action to invoke (e.g. `app_launcher`, `web_launcher`)  |
| `exec`   | `string?` | Command or URL to execute                               |
| `exit`   | `bool`    | Close Sherlock after this action executes, (default = `true`)               |

---

## ExecVariable

A variable injected into a command at spawn time. Defined as a tagged variant.

| Variant          | Value Type    | Description                                              |
| ---------------- | ------------- | -------------------------------------------------------- |
| `string_input`   | `string`      | Prompts the user for a plain text input                  |
| `password_input` | `string`      | Prompts the user for a masked/hidden input               |
| `path_input`     | `PathData`    | Prompts the user to select a file or directory path      |
| `command_input`  | `CommandData` | Prompts the user for input passed to a shell command     |

> [!TIP]
> `path_input` and `command_input` have autocompletion. The `path_input` will
> assume to search in the home directory if the search query does not start
> with `/`

**Example:**

```json
variables = [
    { "string_input": "location" },
    { "password_input": "token" },
]
```

### Example Usage

```json
{
    "name": "Spotify",
    "type": "music_player",
    "args": {},
    "async": true,
    "priority": 2,
    "home": "OnlyHome",
    "spawn_focus": false,
    "exit": false,
    "binds": [
        {
            "bind": "ctrl-l",
            "callback": "next",
            "exit": false
        },
        {
            "bind": "ctrl-h",
            "callback": "previous",
            "exit": false
        }
    ],
    "actions": [
        {
            "name": "Skip",
            "icon": "media-seek-forward",
            "method": "inner.next",
            "exit": false
        },
        {
            "name": "Previous",
            "icon": "media-seek-backward",
            "method": "inner.previous",
            "exit": false
        }
    ]
}
```

---

## App Launcher

Searches and launches installed applications from `.desktop` files.

```json
{
    "name": "App Launcher",
    "alias": "app",
    "type": "apps",
    "args": {
        "use_keywords": true
    },
    "priority": 2,
    "home": "Home"
}
```

### args

| Field | Required | Description |
|---|---|---|
| `use_keywords` | no | If `true`, also searches the `Keywords` field from the `.desktop` file |

---

## Bookmark Launcher

Finds and launches browser bookmarks.

```json
{
    "name": "Bookmarks",
    "type": "bookmarks",
    "args": {},
    "priority": 3,
    "home": "Search"
}
```

### Supported browsers

| Browser | Config value in `default_apps` |
|---|---|
| Zen | `zen`, `zen-browser`, `/opt/zen-browser-bin/zen-bin %u` |
| Firefox | `firefox`, `/usr/lib/firefox/firefox %u` |
| Brave | `brave`, `brave %u` |
| Chrome | `chrome`, `google-chrome`, `/usr/bin/google-chrome-stable %u` |
| Thorium | `thorium`, `/usr/bin/thorium-browser %u` |

The browser is matched against the `exec` string in your config, so both the short name and the full path are accepted.

---

## Calculator

Evaluates math expressions and unit conversions. On return, copies the result to the clipboard.

```json
{
    "name": "Calculator",
    "type": "calculation",
    "args": {
        "capabilities": [
            "calc.math",
            "calc.units"
        ]
    },
    "priority": 1
}
```

### args

| Field | Required | Description |
|---|---|---|
| `capabilities` | no | List of enabled features. Defaults to `calc.math` and `calc.units` |

### **Supported Unit Conversions**

Sherlock supports natural language conversions across the following categories:

| Category | Supported Units & Aliases |
| --- | --- |
| `calc.math` | Basic and advanced mathematical expressions. |
| `colors` | Conversion between Hex, RGB, HSL, and other color formats. |
| `calc.currencies` | USD ($), EUR (€), JPY (¥), GBP (£), AUD (A$), CAD (C$),
CHF, CNY (¥), NZD, SEK (kr), NOK, MXN, SGD, HKD, KRW (₩), PLN (zł), PEN (S/). |
| `calc.length` | mm, cm, m, km, inch ("), feet ('), yard, mile, nautical mile. |
| `calc.volume` | ml, cl, l, kl, cubic meter, tsp, tbsp, fl oz, cup, pint, quart, gallon, imperial gallon. |
| `calc.weight` | mg, g, kg, metric ton, oz, lb, stone, US ton, imperial ton, troy ounce. |
| `calc.temperature` | Celsius (°C), Fahrenheit (°F). |
| `calc.pressure` | Pascal, kPa, bar, atmosphere, psi, Torr (mmHg). |
| `calc.digital` | bit, kb, Mb, Gb, Byte, KB, MB, GB, TB, PB. |
| `calc.time` | ms, seconds, minutes, hours, days, weeks, months, years. |
| `calc.area` | square meter, square kilometer, square foot, square inch, acre, hectare. |
| `calc.speed` | m/s, km/h, mph, knots. |

### Usage Tips

* **Group Activation:** You can use `calc.units` to enable all physical measurement units at once.
* **Case Insensitive:** You can type `KG`, `Kg`, or `kg` interchangeably.
* **Natural Language:** Supports full names (`kilograms`) as well as shorthand (`kg`).
* **Symbols:** Recognizes standard symbols like `$` for currency, `'` for feet, and `"` for inches.

---

## Category Launcher

Groups launchers or commands under a single tile. Activating the tile switches into that launcher's mode.

```json
{
    "name": "Categories",
    "alias": "cat",
    "type": "categories",
    "args": {
        "categories": {
            "Kill Processes": {
                "icon": "sherlock-process",
                "exec": "kill",
                "search_string": "terminate;kill;process"
            },
            "Power Menu": {
                "icon": "battery-full-symbolic",
                "exec": "pm",
                "search_string": "powermenu;"
            }
        }
    },
    "priority": 3,
    "home": "Home"
}
```

### args

**`categories`** (required) — a map of named entries:

| Field | Description |
|---|---|
| `icon` |  Icon name to display |
| `exec` |  Alias of the launcher to activate on return |
| `search_string` |  String used for fuzzy matching |
| `actions` |  The actions to be displayed in the context menu. |


---

## Command Launcher

Runs custom shell commands. Supports variable inputs and replacement variables.

```json
{
    "name": "Utilities",
    "alias": "ex",
    "type": "commands",
    "args": {
        "commands": {
            "NordVPN": {
                "icon": "nordvpn",
                "exec": "nordvpn c {variable:location}",
                "search_string": "nordvpn",
                "variables": [
                    { "string_input": "location" }
                ]
            }
        }
    },
    "priority": 5
}
```

### args

**`commands`** (required) — a map of named entries:

| Field | Required | Description |
|---|---|---|
| `exec` | yes | The command to run |
| `icon` | no | Icon name to display |
| `search_string` | no | String used for fuzzy matching |
| `variables` | no | Variable input fields — see [Variable Inputs](variable-inputs.md) |

---

## Music Player

Shows the currently playing track and controls playback via MPRIS over D-Bus.

```json
{
    "name": "Spotify",
    "type": "music_player",
    "args": {},
    "async": true,
    "priority": 1,
    "home": "Home",
    "spawn_focus": false,
    "actions": [
        {
            "name": "Skip",
            "icon": "media-seek-forward",
            "exec": "playerctl next",
            "method": "command"
        }
    ],
    "binds": [
        { "bind": "ctrl-p", "callback": "inner.playpause", "exit": false },
        { "bind": "ctrl-l", "callback": "inner.next", "exit": false },
        { "bind": "ctrl-h", "callback": "inner.previous", "exit": false }
    ]
}
```

### Inner functions

| Function | Description |
|---|---|
| `playpause` | Toggle playback |
| `next` | Skip to next track |
| `previous` | Go to previous track |
| `unbind` | Unbind a key (useful to unbind return) |

---

## Weather Launcher

Shows current weather conditions for a configured location.

```json
{
    "name": "Weather",
    "type": "weather",
    "args": {
        "location": "berlin",
        "update_interval": 60,
        "icon_theme": "Sherlock",
        "show_datetime": false
    },
    "priority": 1,
    "home": "OnlyHome",
    "async": true,
    "shortcut": false,
    "spawn_focus": false
}
```

### args

| Field | Required | Description |
|---|---|---|
| `location` | yes | City or region name |
| `update_interval` | no | Cache TTL in minutes |
| `icon_theme` | no | `Sherlock` to use bundled icons, omit for system theme |
| `show_datetime` | no | Show current date and time alongside weather |

---

## Web Launcher

Opens a search query in the browser using a configured search engine.

```json
{
    "name": "Web Search",
    "display_name": "Google Search",
    "alias": "gg",
    "type": "web_launcher",
    "args": {
        "search_engine": "google",
        "icon": "google"
    },
    "priority": 100
}
```

### args

| Field | Required | Description |
|---|---|---|
| `search_engine` | yes | Engine name or a custom URL containing `{keyword}` |
| `icon` | yes | Icon name to display |

### Built-in search engines

| Name | URL |
|---|---|
| `google` | `https://www.google.com/search?q={keyword}` |
| `bing` | `https://www.bing.com/search?q={keyword}` |
| `duckduckgo` | `https://duckduckgo.com/?q={keyword}` |
| `yahoo` | `https://search.yahoo.com/search?p={keyword}` |
| `ecosia` | `https://www.ecosia.org/search?q={keyword}` |
| `startpage` | `https://www.startpage.com/sp/search?q={keyword}` |
| `qwant` | `https://www.qwant.com/?q={keyword}` |
| `yandex` | `https://yandex.com/search/?text={keyword}` |
| Custom | Any URL with `{keyword}` as the query placeholder |

---

## Clipboard Launcher

> **Not yet implemented in this version**

Reads the clipboard and acts on its content — opening URLs, displaying colors, or evaluating expressions.

---

## Debug Launcher

> **Not yet implemented in this version**

Runs internal debug commands such as clearing the cache or resetting launch counts.

---

## Emoji Picker

> **Not yet implemented in this version**

Searches and inserts emoji characters.

---

## Bulk Text

> **Not yet implemented in this version**

Runs an external script asynchronously and displays its output as a text widget.

---

## Teams Event

> **Not yet implemented in this version**

Shows upcoming Microsoft Teams meetings and joins them on return.

---

## Theme Picker

> **Not yet implemented in this version**

Lists available themes and applies them on selection.

---

## Process Terminator

> **Not yet implemented in this version**

Lists running user processes and terminates the selected one on return.

---

## Pomodoro Timer

> **Not yet implemented in this version**

Displays a Pomodoro focus timer. Requires the external [sherlock-pomodoro](https://github.com/Skxxtz/sherlock-pomodoro) client.

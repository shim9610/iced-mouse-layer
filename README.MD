# iced-mouse-layer

A small [iced](https://github.com/iced-rs/iced) widget library that makes it easy to render a widget that follows the mouse cursor. :contentReference[oaicite:1]{index=1}

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iced = "0.14"
iced-mouse-layer = { git = "https://github.com/shim9610/iced-mouse-layer" }
````

## Quick Start (copy & run)

Create `examples/quick_start.rs`:

```rust
use iced::{Element, Length, Task, Theme};
use iced::widget::{column, container, text};
use iced_mouse_layer::mouse_layer;

#[derive(Debug, Clone)]
enum Message {}

#[derive(Default)]
struct App;

impl App {
    fn title(&self) -> String {
        "iced-mouse-layer quick start".into()
    }

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let base = container(text("Move your mouse.")).padding(16);

        let ghost = mouse_layer(
            container(text("👻 MouseLayer"))
                .padding(10)
        )
        .offset(15.0, 15.0);

        column![base, ghost]
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title(App::title)
        .theme(Theme::Dark)
        .run()
}
```

Run:

```bash
cargo run --example quick_start
```

## API

### `mouse_layer(content)`

Renders `content` at the mouse cursor position.

```rust
mouse_layer(text("Hello"))
```

### `.offset(x, y)`

Sets the offset from the cursor. Default is `(0.0, 0.0)`.

```rust
mouse_layer(text("Hello"))
    .offset(10.0, 10.0)
```

## Notes

1. Avoid using `Length::Fill` inside the `mouse_layer` content. Prefer fixed sizes.
2. The layer may not be visible when the cursor is outside the window (platform/OS behavior).



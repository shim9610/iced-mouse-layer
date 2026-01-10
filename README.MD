# iced-mouse-layer

마우스 커서를 따라다니는 위젯을 쉽게 만들 수 있는 [iced](https://github.com/iced-rs/iced) 위젯 라이브러리입니다.

## 설치

`Cargo.toml`에 추가:
```toml
[dependencies]
iced = "0.14"
iced-mouse-layer = { git = "https://github.com/shim9610/iced-mouse-layer" }
```

## 기본 사용법
```rust
use iced::widget::{column, container, text};
use iced::Element;
use iced_mouse_layer::mouse_layer;

fn view(&self) -> Element<'_, Message> {
    let base_content = text("메인 컨텐츠");
    
    // 마우스를 따라다니는 위젯
    let floating = mouse_layer(
        container(text("마우스 따라다님!"))
            .padding(10)
    )
    .offset(15.0, 15.0);  // 커서로부터의 오프셋

    column![base_content, floating].into()
}
```

## 주요 기능

### `mouse_layer(content)`

마우스 커서 위치에 `content`를 렌더링합니다.
```rust
mouse_layer(text("Hello"))
```

### `.offset(x, y)`

커서로부터의 오프셋을 설정합니다. 기본값은 `(0.0, 0.0)`.
```rust
mouse_layer(text("Hello"))
    .offset(10.0, 10.0)  // 커서 우하단 10px 위치
```

## 예제: 드래그 앤 드롭
```rust
use iced::widget::{column, container, mouse_area, text};
use iced::{color, mouse, Element, Event, Point, Subscription, Task, Theme};
use iced_mouse_layer::mouse_layer;

#[derive(Debug, Clone)]
enum Message {
    DragStart { item: String },
    DragMove(Point),
    DragEnd,
}

struct App {
    items: Vec<String>,
    dragging: Option<DragState>,
}

struct DragState {
    item: String,
    cursor: Point,
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::DragStart { item } => {
                self.dragging = Some(DragState {
                    item,
                    cursor: Point::ORIGIN,
                });
            }
            Message::DragMove(position) => {
                if let Some(ref mut drag) = self.dragging {
                    drag.cursor = position;
                }
            }
            Message::DragEnd => {
                if let Some(drag) = self.dragging.take() {
                    // 드롭 처리
                }
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let items = column(
            self.items.iter().map(|item| {
                mouse_area(
                    container(text(item)).padding(8)
                )
                .on_press(Message::DragStart { item: item.clone() })
                .into()
            }).collect::<Vec<_>>()
        );

        // 드래그 중이면 고스트 표시
        if let Some(ref drag) = self.dragging {
            let ghost = mouse_layer(
                container(text(&drag.item))
                    .padding(10)
                    .style(ghost_style)
            )
            .offset(10.0, 10.0);

            column![items, ghost].into()
        } else {
            items.into()
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| {
            match event {
                Event::Mouse(mouse::Event::CursorMoved { position }) => {
                    Some(Message::DragMove(position))
                }
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    Some(Message::DragEnd)
                }
                _ => None,
            }
        })
    }
}

fn ghost_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(color!(0x4a90d9, 0.9).into()),
        border: iced::Border {
            radius: 6.0.into(),
            ..Default::default()
        },
        text_color: Some(color!(0xffffff)),
        ..Default::default()
    }
}
```

## 주의사항

1. **`Length::Fill` 사용 불가**: MouseLayer 내부 위젯에서 `width(Fill)` 또는 `height(Fill)` 사용 시 무한 크기로 확장됨. 고정 크기 사용 권장.

2. **창 밖 렌더링 불가**: 마우스가 창 밖으로 나가면 위젯이 보이지 않음 (OS 제한).

3. **커서 추적**: 드래그 앤 드롭 구현 시 `subscription`으로 마우스 위치를 별도로 추적해야 함.

## 라이선스

MIT
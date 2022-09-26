use iced::{button, Button, Column, Element, Sandbox, Settings, Text};

#[derive(Default)]
struct MyButton {
    /// Button の状態を保持する必要がある
    button_state: button::State,
    /// button_stateがtrueになったら文字列を付与する
    test_text: String,
}

#[derive(Debug, Clone)]
enum Message {
    ButtonPressed,
}

impl Sandbox for MyButton {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Button")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ButtonPressed => {
                self.test_text = String::from("Pushed!");
                println!("Button pressed");
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        // Button Widget を生成し、
        // Button が押されたら Message を送信する
        // この Widget を `into()` で Element<Element> に変換して返すことで描画される
        Column::new().push(Button::new(&mut self.button_state, Text::new("Button"))
            .on_press(Message::ButtonPressed)
        ).push(Text::new(&self.test_text))
            .padding(20).into()
    }
}

fn main() -> iced::Result {
    // Sandbox を実装した State (Counter) を実行する
    // Settings を変更すれば、ウィンドウサイズ等の設定が変更可能
    let mut set = Settings::default();
    set.window.size = (512, 384);
    set.window.resizable = false;
    MyButton::run(set)
}
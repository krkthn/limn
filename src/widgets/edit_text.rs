use widget::{WidgetBuilder, EventHandler, EventArgs};
use widget::property::PropChangeHandler;
use input::keyboard::{WidgetFocusHandler, WidgetReceivedCharacter};
use drawable::rect::RectDrawable;
use drawable::text::TextDrawable;

pub struct EditTextKeyboardHandler {
    text: String,
}
impl EditTextKeyboardHandler {
    pub fn new() -> Self {
        EditTextKeyboardHandler {
            text: "".to_owned(),
        }
    }
}
impl EventHandler<WidgetReceivedCharacter> for EditTextKeyboardHandler {
    fn handle(&mut self, event: &WidgetReceivedCharacter, args: EventArgs) {
        let &WidgetReceivedCharacter(char) = event;
        match char {
            '\u{8}' => {
                self.text.pop();
            }
            _ => {
                self.text.push(char);
                let drawable = args.widget.drawable::<TextDrawable>().unwrap();
                if !drawable.text_fits(&self.text, args.widget.layout.bounds()) {
                    self.text.pop();
                }
            }
        }
        args.widget.update(|state: &mut TextDrawable| state.text = self.text.clone());
    }
}


pub struct EditTextBuilder {
    pub widget: WidgetBuilder,
}
impl EditTextBuilder {
    pub fn new() -> Self {

        let mut widget = WidgetBuilder::new()
            .set_drawable(RectDrawable::new())
            .add_handler(WidgetFocusHandler)
            .add_handler(PropChangeHandler);

        let text_widget = WidgetBuilder::new()
            .set_drawable(TextDrawable::new())
            .add_handler(EditTextKeyboardHandler::new())
            .add_handler(PropChangeHandler);
        
        widget.add_child(text_widget);
        EditTextBuilder {
            widget: widget,
        }
    }
}
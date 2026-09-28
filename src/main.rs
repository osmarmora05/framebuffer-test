slint::slint! {
    export component App inherits Window {
        width: 480px;
        height: 272px;

        Text {
            text: "Hello Vita!";
            font-size: 32px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    app.run()
}

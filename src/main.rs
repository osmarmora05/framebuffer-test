slint::slint! {
    export component App inherits Window {
        width: 480px;
        height: 272px;

        Image {
            source: @image-url("img/avatar.jpg", nine-slice(30 30 30 30));
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    app.run()
}

macro_rules! assets {
    ($($name:ident => $path:literal),* $(,)?) => {
        pub struct Assets {
            $(pub $name: tetra::graphics::Texture,)*
        }

        impl Assets {
            pub fn new(ctx: &mut tetra::Context) -> tetra::Result<Assets> {
                Ok(Assets {
                    $($name: tetra::graphics::Texture::new(ctx, $path)?,)*
                })
            }
        }
    };
}

assets! {
    backdrop => "./resources/backdrop.png",
    gate_back => "./resources/gate_back.png",
    gate_front => "./resources/gate_front.png",
    heart => "./resources/heart.png",
    heart_broken => "./resources/heart_broken.png",
    orb => "./resources/orb.png",
    star => "./resources/star.png",
}

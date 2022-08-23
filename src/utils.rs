// Message embed macro

#[macro_export]
macro_rules! embed {
    ( $($attr:ident : $value:expr),* ) => {
        {
            let mut embed = CreateEmbed::default();
            embed$( .$attr($value) )*;
            embed
        }
    }
}
#[macro_use]
extern crate literal_length;

macro_rules! literal_length {
    ( $($name:ident = $value:expr),* $(,)* ) => {
        #[allow(dead_code)]
        #[derive(LiteralLength)]
        #[Literals(
            $( $name = $value ),*
        )]
        struct __LiteralLength;
    }
}

literal_length! {
    A = "Hello, World!",
    ETC = "More strings to work with!",
}

#[test]
fn test() {
    assert_eq!(A, "Hello, World!");
    assert_eq!(A_LEN, 13);

    assert_eq!(ETC, "More strings to work with!");
    assert_eq!(ETC_LEN, 26);
}
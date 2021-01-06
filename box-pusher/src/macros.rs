#![macro_use]

macro_rules! map {
    ( $( $name:expr => $value:expr ),* ) => {{
        let mut h = HashMap::new();
        $( h.insert($name.to_string(), $value); )*
        h
    }};
}

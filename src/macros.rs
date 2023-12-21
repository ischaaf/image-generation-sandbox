#[macro_export]
macro_rules! make_regions {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec: Vec<Box<dyn Region>> = Vec::new();
            $(
                temp_vec.push(Box::new($x));
            )*
            temp_vec
        }
    };
}

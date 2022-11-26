#[cfg(test)]
mod tests {
    use concoction_macro::FromInner;

    #[test]
    fn it_can_derive_from_inner() {
        #[derive(FromInner)]
        struct SomeNewtype(Vec<i32>);

        let inner_data: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13];

        let some_newtype_instance: SomeNewtype = inner_data.into();

        assert_eq!(vec![1, 1, 2, 3, 5, 8, 13], some_newtype_instance.0);
    }
}

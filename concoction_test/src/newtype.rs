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

    #[test]
    fn it_can_derive_from_inner_when_it_has_lifetimes_before_inner() {
        #[derive(FromInner)]
        #[from_inner(lifetimes = 'a)]
        struct SomeNewtype<'a>(&'a Vec<i32>);

        let inner_data: Vec<i32> = vec![1, 1, 2, 3, 5, 8, 13];

        let some_newtype_instance: SomeNewtype = (&inner_data).into();

        assert_eq!(&vec![1, 1, 2, 3, 5, 8, 13], some_newtype_instance.0);
    }

    #[test]
    fn it_can_derive_from_inner_when_it_has_lifetimes_inside_inner() {
        #[derive(FromInner)]
        #[from_inner(lifetimes = 'a)]
        struct SomeNewtype<'a>(Vec<&'a str>);

        let inner_data: Vec<&str> = vec!["the", "quick", "brown", "fox"];

        let some_newtype_instance: SomeNewtype = inner_data.into();

        assert_eq!(
            vec!["the", "quick", "brown", "fox"],
            some_newtype_instance.0
        );
    }

    #[test]
    fn it_can_derive_from_inner_when_it_has_lifetimes_before_and_inside_inner() {
        #[derive(FromInner)]
        #[from_inner(lifetimes = 'a, 'b)]
        struct SomeNewtype<'a, 'b>(&'a Vec<&'b str>);

        let inner_data: Vec<&str> = vec!["the", "quick", "brown", "fox"];

        let some_newtype_instance: SomeNewtype = (&inner_data).into();

        assert_eq!(
            &vec!["the", "quick", "brown", "fox"],
            some_newtype_instance.0
        );
    }
}

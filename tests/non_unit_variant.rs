mod utils;

mod struct_variant {
    use enum_rotate::EnumRotate;

    #[test]
    fn variant_struct_empty() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A {},
        }

        let a = Enum::A {};

        assert_eq!(a.next(), Enum::A {});
    }

    #[test]
    fn variant_struct_empty_second() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A,
            B {},
            C,
        }

        let mut item = Enum::A;

        assert_eq!(item.rotate_next(), &Enum::B {});
        assert_eq!(item.rotate_next(), &Enum::C);
        assert_eq!(item.rotate_next(), &Enum::A);

        assert_eq!(item, Enum::A);

        assert_eq!(item.rotate_prev(), &Enum::C);
        assert_eq!(item.rotate_prev(), &Enum::B {});
        assert_eq!(item.rotate_prev(), &Enum::A);
    }

    #[test]
    fn variant_struct_nonempty() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A { field: usize },
        }

        let mut item = Enum::A { field: 0 };

        assert_eq!(item.rotate_next(), &Enum::A { field: 0 });

        item = Enum::A { field: 55 };

        assert_eq!(item.rotate_next(), &Enum::A { field: 0 });
    }

    #[test]
    fn variant_struct_nonempty_second() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A,
            B { field: usize },
            C,
        }

        let mut item = Enum::B { field: 42 };

        assert_eq!(item.rotate_next(), &Enum::C);
        assert_eq!(item.rotate_next(), &Enum::A);
        assert_eq!(item.rotate_next(), &Enum::B { field: 0 });
    }
}

mod tuple_variant {
    use enum_rotate::EnumRotate;

    #[test]
    fn variant_tuple_empty() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A(),
        }

        let a = Enum::A();

        assert_eq!(a.next(), Enum::A());
    }

    #[test]
    fn variant_tuple_empty_second() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A,
            B(),
            C,
        }

        let mut item = Enum::A;

        assert_eq!(item.rotate_next(), &Enum::B());
        assert_eq!(item.rotate_next(), &Enum::C);
        assert_eq!(item.rotate_next(), &Enum::A);

        assert_eq!(item, Enum::A);

        assert_eq!(item.rotate_prev(), &Enum::C);
        assert_eq!(item.rotate_prev(), &Enum::B());
        assert_eq!(item.rotate_prev(), &Enum::A);
    }

    #[test]
    fn variant_tuple_nonempty() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A(usize),
        }

        let mut item = Enum::A(0);

        assert_eq!(item.rotate_next(), &Enum::A(0));

        item = Enum::A(0);

        assert_eq!(item.rotate_next(), &Enum::A(0));
    }

    #[test]
    fn variant_tuple_nonempty_second() {
        #[derive(Debug, EnumRotate, PartialEq)]
        enum Enum {
            A,
            B(usize),
            C,
        }

        let mut item = Enum::B(42);

        assert_eq!(item.rotate_next(), &Enum::C);
        assert_eq!(item.rotate_next(), &Enum::A);
        assert_eq!(item.rotate_next(), &Enum::B(0));
    }
}

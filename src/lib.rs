pub mod binary_search_tree;
pub mod linked_list_01;
pub mod linked_list_02;
pub mod unique;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[derive(Debug, PartialEq)]
    struct Element {
        pub value: i32,
    }

    impl Element {
        pub fn new(value: i32) -> Self {
            Element { value: value }
        }
    }

    struct Value {
        pub value: Option<i32>,
    }

    impl Value {
        pub fn new() -> Self {
            Value {
                value: Option::None,
            }
        }

        pub fn value(value: i32) -> Self {
            Value {
                value: Option::Some(value),
            }
        }
    }

    #[test]
    pub fn vec_00() {
        let xs = vec![Element::new(42), Element::new(43)];

        assert_eq!(xs[0], Element::new(42));
    }

    #[test]
    pub fn vec_01() {
        let mut xs = vec![Element::new(42), Element::new(43)];

        for x in &mut xs {
            x.value = x.value + 1;
        }

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[1], Element::new(44));
    }

    #[test]
    pub fn vec_02() {
        let mut xs = vec![Element::new(42), Element::new(43)];

        for x in xs.iter_mut() {
            x.value = x.value + 1
        }

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[1], Element::new(44));
    }

    #[test]
    pub fn opt_00() {
        let value = Value::new();

        assert_eq!(value.value, Option::None);
    }

    #[test]
    pub fn opt_01() {
        let xs = vec![Value::value(42), Value::value(43)];

        assert_eq!(xs[0].value, Option::Some(42));
    }

    #[test]
    pub fn opt_02() {
        let mut xs = vec![Value::value(42), Value::new(), Value::value(43)];

        for x in &mut xs {
            x.value = match x.value {
                Option::None => Option::Some(44),
                Option::Some(a) => Option::Some(a),
            }
        }

        assert_eq!(xs[0].value, Option::Some(42));
        assert_eq!(xs[1].value, Option::Some(44));
    }

    #[test]
    pub fn hash_map_00() {
        let mut xs: HashMap<i32, String> = HashMap::new();

        xs.insert(1, String::from("hello"));
        xs.insert(2, String::from("world"));

        for entry in xs {
            if entry.0 == 1 {
                assert_eq!("hello", entry.1);
            }
            if entry.0 == 2 {
                assert_eq!("world", entry.1);
            }
        }
    }
}

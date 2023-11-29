// A macro to create various collections from a list of elements,
// similar to the vec! macro but for other collections as well.
#[macro_export]
macro_rules! collect {
        ($($k:expr => $v:expr),* $(,)?) => {{
            core::convert::From::from([$(($k, $v),)*])
        }};
        ($($v:expr),* $(,)?) => {{
            core::convert::From::from([$($v,)*])
        }};
    }

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    #[test]
    fn test_collect_for_map() {
        let map: HashMap<&str, u32> = collect!["a" => 1, "b" => 2];
        assert_eq!(map["a"], 1);
        assert_eq!(map["b"], 2);
    }

    #[test]
    fn test_collect_for_vec() {
        let vec: Vec<u32> = collect![1, 2, 3];
        assert_eq!(vec[0], 1);
        assert_eq!(vec[1], 2);
        assert_eq!(vec[2], 3);
    }

    #[test]
    fn test_collect_for_tuple() {
        let tuple: (u32, u32, u32) = collect![1, 2, 3];
        assert_eq!(tuple.0, 1);
        assert_eq!(tuple.1, 2);
        assert_eq!(tuple.2, 3);
    }

    #[test]
    fn test_collect_for_tuple_of_tuples() {
        let tuple: ((u32, u32), (u32, u32), (u32, u32)) = collect![(1, 2), (3, 4), (5, 6)];
        assert_eq!(tuple.0, (1, 2));
        assert_eq!(tuple.1, (3, 4));
        assert_eq!(tuple.2, (5, 6));
    }

    #[test]
    fn test_collect_for_tuple_of_tuples_of_tuples() {
        let tuple: (((u32, u32), (u32, u32)), ((u32, u32), (u32, u32))) = collect![((1, 2), (3, 4)), ((5, 6), (7, 8))];
        assert_eq!(tuple.0, ((1, 2), (3, 4)));
        assert_eq!(tuple.1, ((5, 6), (7, 8)));
    }


    #[test]
    fn test_for_b_tree() {
        let b_tree: BTreeMap<&str, u32> = collect!["a" => 1, "b" => 2];
        assert_eq!(b_tree["a"], 1);
        assert_eq!(b_tree["b"], 2);
    }
}

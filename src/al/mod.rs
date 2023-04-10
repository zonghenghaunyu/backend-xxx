pub mod dp;
pub mod easy;
pub mod hard;
pub mod hs;
pub mod mid;
pub mod tree;
mod tx;
mod list_node;

mod xxx {
    use chrono::{DateTime, Utc};
    use std::rc::Rc;

    #[derive(Debug)]
    pub struct SomeThing {
        pub id: i32,

        pub name: Rc<String>,

        pub time: DateTime<Utc>,
    }
}

#[cfg(test)]
mod tests {
    use super::xxx::SomeThing;
    use std::collections::{HashMap, LinkedList};
    use std::rc::Rc;
    use std::time::Duration;
    fn use_vec(list: LinkedList<SomeThing>) -> Vec<SomeThing> {
        let s = list.iter().map(|a| a.id).collect::<Vec<_>>();
        println!("{:?}", s);
        println!("-----------------");
        let s4 = vec![999, 888, 777];
        let map = s.iter().zip(s4.iter()).collect::<HashMap<_, _>>();
        // let s1 = list.iter().map(|a| a.name.clone()).collect::<Vec<_>>();
        map.iter()
            .for_each(|(&k, &v)| println!("{}----------{}", k, v));
        vec![]
    }

    #[test]
    fn it_works() {
        let s1 = SomeThing {
            id: 1,
            name: Rc::new("aaa".to_string()),
            time: chrono::Utc::now(),
        };
        std::thread::sleep(Duration::from_secs(1));
        let s2 = SomeThing {
            id: 3,
            name: Rc::new("ccc".to_string()),
            time: chrono::Utc::now(),
        };
        let s3 = SomeThing {
            id: 3,
            name: Rc::new("ccc".to_string()),
            time: chrono::Utc::now(),
        };
        let mut arr = LinkedList::new();
        arr.push_front(s1);
        arr.push_front(s2);
        arr.push_front(s3);

        use_vec(arr);
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
struct Rectangle {
    x: i32,
    y: i32,
    side_x: i32,
    side_y: i32,
}

pub fn main() {
    let mut xs = vec![
        Rectangle {
            x: 0,
            y: 0,
            side_x: 10,
            side_y: 20,
        },
        Rectangle {
            x: 3,
            y: 9,
            side_x: 4,
            side_y: 8,
        },
    ];

    xs.sort_by(|lft, rgt| lft.side_x.cmp(&rgt.side_x));

    println!("{:#?}", xs);
}

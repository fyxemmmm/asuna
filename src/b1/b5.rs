fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {  // 解构
        if item > largest {
            largest = item
        }
    }
    largest
}

fn main() {
    let number_list = [3,1,2,4123,4,1,123,211,32];
    let result = largest(&number_list); // 传递的是i32的切片
    println!("largest: {}", result);

    let number_list = vec![3,1,2,4123,4,1,123,211,32];
    let result = largest(&number_list); // 传递的是i32的切片
    println!("largest: {}", result);
}



// fanxing
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point{
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point{x: 5, y:4};
    let p2 = Point{x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    // p3.x = 5, p3.y = c
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}


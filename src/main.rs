/* use warp::Filter;
#[tokio::main]
async fn main() {
   simple_api().await;
}


async fn simple_api() {
    let hello = warp::path!("hello" / String)
    .map(|name| format!("Hello, {}!", name));

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
} */


fn main() {
    let number_list = vec![34, 50, 25, 100, 101];
    let result = largest(&number_list);

    print!("The largest number is {}", result);
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest() {
        let number_list = vec![34, 50, 25, 100, 65];
        assert_eq!(largest(&number_list), 100);

        let char_list = vec!['y', 'm', 'a', 'q'];
        assert_eq!(largest(&char_list), 'y');
    }
}
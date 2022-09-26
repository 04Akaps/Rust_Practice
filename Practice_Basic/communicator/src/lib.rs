mod network;
mod client;

/*

mod는 모듈을 정의하는 공간 입니다.

사용할떄에는 network::connect로 사용합니다.

*/


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

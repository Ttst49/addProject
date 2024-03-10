use rand;

pub fn add_one(x:i64)->i64{
    x+1
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn it_work(){
        assert_eq!(3,add_one(2))
    }
}
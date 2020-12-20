#[cfg(test)]
mod tests {
    use crate::coin;

    #[test]
    fn it_works() {

        let mut aimyon = coin::User::from("aimyon".to_string());    //계정생성
        let mut zaimy = coin::User::from("zaimy".to_string());      //계정등록
        let mut rustacean = coin::User::from("rustacean".to_string());  //계정등록


        let tr1 = aimyon.send(&zaimy, 0);    //1차 거래발생 -> 트랜잭션생성
        let blk = rustacean.mine(tr1.clone());    //제3자 체굴로 인한 거래완료 -> 블록생성

        //블록통신
        aimyon.get_block(blk.clone());
        zaimy.get_block(blk.clone());
        rustacean.get_block(blk.clone());

        println!("{:?}",aimyon);
        println!("{:?}",zaimy);
        println!("{:?}",rustacean);

        let tr2 = rustacean.send(&zaimy, 1);    //1차 거래발생 -> 트랜잭션생성
        let blk2 = aimyon.mine(tr2.clone());    //제3자 체굴로 인한 거래완료 -> 블록생성

        //블록통신
        aimyon.get_block(blk2.clone());
        zaimy.get_block(blk2.clone());
        rustacean.get_block(blk2.clone());

        println!("{:?}",aimyon);
        println!("{:?}",zaimy);
        println!("{:?}",rustacean);

        assert_eq!(aimyon.get_cnt(), 1);
        assert_eq!(zaimy.get_cnt(), 1);
        assert_eq!(rustacean.get_cnt(), 0);
    }
}
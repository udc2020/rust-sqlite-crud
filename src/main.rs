mod db;


fn main(){

    // remove row
    db::remove_one(33);

    //add new row
    // db::add_new(33, "three", "56451");

    // update row
    db::update_one(100, "new body", "545fcc");

   

    // find all rows in table
     match db::find_all(){
    //     ? res is the result of sql quiry
        Ok(res) => println!("{:?}",res),
        _=>println!("empty")
    };


    // find one row
    match db::find_one(250){
            Ok(res) => println!("{:?}",res[0]),
            _ =>println!("empty")
        };
   
}
use mysql::*;
use mysql::prelude::*;

pub fn mysqlConnect(){
    let url = "mysql://root:root@localhost:8888/blog_master";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();
}
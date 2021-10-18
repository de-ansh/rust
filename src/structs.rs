//struct Color{
  //  red:u8,
    //green: u8,
    //blue:u8
//}
// Tuple Struct 
struct Color(u8,u8,u8);
struct person{
    first_name: String, 
    last_name: String
}
impl person{
    fn new(first: &str, last:& str)->person{
        person{
            first_name: first.to_string(),
            last_name: last.to_string()
        }
    }
    fn full_name(&self)->String {
        format!("{} {}", self.first_name, self.last_name)
    }
    fn set_last_name(&mut self, last:&str){
        self.last_name=last.to_string();
    }
    fn to_tuple(self)->(String, String){
        (self.first_name, self.last_name)
    }
}

    
pub fn run(){
    /*let mut c= Color{
        red: 255,
        green: 0,
        blue: 0
    };
    c.red = 200;
    println!("Color:{} {} {}",c.red, c.green,c.blue );*/
    let mut c= Color(255,0,0);
    c.0=200;
    println!("Color:{} {} {}",c.0,c.1,c.2 );
    let mut p= person::new("ritwik", "Talks");
    p.set_last_name("Paul");

    println!("Person {}", p.full_name());
    println!("Person {:?}", p.to_tuple());

}
trait Animal
{
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self) {
        println!("{} cannot talk", self.name());
    }
}

trait Summable<T>{
    fn sum(&self) -> T;
}

pub fn traits(){

    struct Human{
        name: &'static str,
    }

    struct Cat{
        name: &'static str,
    }

    impl Summable<i32> for Vec<i32>{
        fn sum(&self) -> i32 {
            let mut res = 0;
            for x in self {
                res += *x;
            }
            res
        }
    }

    impl Animal for Human {
        fn create(name: &'static str) -> Human {
            Human { name }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) {
            println!("Hello, I'm {}", self.name);
        }
    }

    impl Animal for Cat{
        fn create(name: &'static str) -> Cat {
            Cat { name }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn talk(&self) {
            println!("*{}, the cat*: Meow!", self.name);
        }
    }


    let h = Human::create("John");
    h.talk();
    let c = Cat::create("Misty");
    c.talk();
    c.talk();

    let an_int_vec = vec![1,2,3,4,5,6,7,];
    println!("vec sum = {}", an_int_vec.sum());

}



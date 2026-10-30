struct Coffee {
    is_hot: bool,
    price: i32
}

impl Coffee {
    fn otro(self) {
        println!("print desde otro");
    }

    fn algo(mut self, _hol: i32, mut __otro: Self) {
        self.is_hot = false;
        println!("fn algo");
        self.otro();
    }

    fn otherone() {

    }
}

fn main() {
    

    let coffee: Coffee = Coffee {
        is_hot: true,
        price: 50
    };

    println!("{} {}", coffee.is_hot, coffee.price);

    fun();

}

fn fun() {
    let coffee: Coffee = Coffee {
        is_hot: false,
        price: 25
    };

    let coffee2: Coffee = Coffee {
        is_hot: false,
        price: 25
    };
    println!("{} {}", coffee.is_hot, coffee.price);
    coffee.algo(40, coffee2);


    Coffee::otherone();
}
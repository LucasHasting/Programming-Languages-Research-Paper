//add trait
use std::ops::Add;

fn main() {
    //test Fraction class with the overloaded + operator
    let fraction1 = Fraction { numerator : 5 , denomonator : 8};
    let fraction2 = Fraction { numerator : 5 , denomonator : 8};
    let fraction3 = fraction1 + fraction2;
    println!("{}/{}", fraction3.numerator, fraction3.denomonator)
}

//create basic Fraction class
struct Fraction {
    pub numerator : i16,
    pub denomonator: i16
}

//implement the add trait for the Fraction class
impl Add<Fraction> for Fraction{
    type Output = Fraction;

    fn add(mut self, rhs: Fraction) -> Fraction {
        self.numerator = self.numerator * rhs.denomonator;
        self.numerator = self.numerator + (rhs.numerator * self.denomonator);
        self.denomonator = self.denomonator * rhs.denomonator;

        return self;
    }
}
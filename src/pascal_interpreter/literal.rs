use std::ops::{Add, Sub, Mul, Div, Rem};

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool)
}

impl Literal
{
    pub fn from_str(text : String) -> Literal {
        Literal::Text(text)
    }

    pub fn from_int(num : i64) -> Literal {
        Literal::Int(num)
    }

    pub fn from_float(num : f64) -> Literal {
        Literal::Float(num)
    }

    pub fn from_bool(boolean : bool) -> Literal {
        Literal::Bool(boolean)
    }

    pub fn to_str(&self) -> Option<&str> {
        match &self {
            Literal::Text(s) => Some(s),
            _ => None,
        }
    }

    pub fn to_int(&self) -> Option<i64> {
        match &self {
            Literal::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn to_float(&self) -> Option<f64>{
        match &self {
            Literal::Float(f) => Some(*f),
            Literal::Int(i) => Some(*i as f64),
            _ => None,
        }
    }

}

impl Add for Literal {
    type Output = Literal;

    fn add(self, rhs: Literal) -> Literal {
        match (&self, &rhs) {
            (Literal::Text(lhs), Literal::Text(rhs)) => {
                let mut result = lhs.to_string();
                result.push_str(rhs);
                return Literal::Text(result);
            },
            (Literal::Int(x),   Literal::Int(y))    => return Literal::from_int(x + y),
            (Literal::Float(x), Literal::Float(y))  => return Literal::from_float(x + y), 
            (Literal::Int(x),   Literal::Float(y))  => return Literal::from_float((*x as f64) + y),
            (Literal::Float(x), Literal::Int(y))    => return Literal::from_float(x + (*y as f64)),
            _ => panic!("not implemented for {:?} and {:?}", &self, &rhs),
        }
    }
}

impl Sub for Literal {
    type Output = Literal;

    fn sub(self, rhs: Literal) -> Literal {
        match (&self, &rhs) {
            (Literal::Int(x),   Literal::Int(y))    => return Literal::from_int(x - y),
            (Literal::Float(x), Literal::Float(y))  => return Literal::from_float(x + y), 
            (Literal::Int(x),   Literal::Float(y))  => return Literal::from_float((*x as f64) + y),
            (Literal::Float(x), Literal::Int(y))    => return Literal::from_float(x + (*y as f64)),
            _ => panic!("not implemented for {:?} and {:?}", &self, &rhs),
        }
    }
}


impl Mul for Literal {
    type Output = Literal;

    fn mul(self, rhs: Literal) -> Literal {
        match (&self, &rhs) {
            (Literal::Int(x),   Literal::Int(y))    => return Literal::from_int(x * y),
            (Literal::Float(x), Literal::Float(y))  => return Literal::from_float(x * y), 
            (Literal::Int(x),   Literal::Float(y))  => return Literal::from_float((*x as f64) * y),
            (Literal::Float(x), Literal::Int(y))    => return Literal::from_float(x * (*y as f64)),
            _ => panic!("not implemented for {:?} and {:?}", &self, &rhs),
        }
    }
}

impl Div for Literal {
    type Output = Literal;

    fn div(self, rhs: Literal) -> Literal {
        match (&self, &rhs) {
            (Literal::Int(x),   Literal::Int(y))    => return Literal::from_float((*x as f64) / (*y as f64)),
            (Literal::Float(x), Literal::Float(y))  => return Literal::from_float(x / y), 
            (Literal::Int(x),   Literal::Float(y))  => return Literal::from_float((*x as f64) / y),
            (Literal::Float(x), Literal::Int(y))    => return Literal::from_float(x / (*y as f64)),
            _ => panic!("not implemented for {:?} and {:?}", &self, &rhs),
        }
    }
}

impl Rem for Literal {
    type Output = Literal;

    fn rem(self, rhs: Literal) -> Literal {
        match (&self, &rhs) {
            (Literal::Int(x),   Literal::Int(y))    => return Literal::from_int(x % y),
            _ => panic!("not implemented for {:?} and {:?}", &self, &rhs),
        }
    }
}
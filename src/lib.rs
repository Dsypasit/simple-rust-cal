use std::error::Error;
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn sub(a: i32, b: i32) -> i32 {
    a - b
}

pub fn mul(a: i32, b: i32) -> i32 {
    a * b
}

pub fn div(a: i32, b: i32) -> i32 {
    a / b
}

pub struct MathFormula {
    pub formular: String,
}

pub fn run(s: Vec<String>) {
    let a: String = s[1].split("").filter(|s| s.ne(&"")).collect();
    let result = MathFormula::new(a);
    let a = result.process();
    println!("result = {}", a)
}

fn print_type<T>(_: T) {
    println!("{}", std::any::type_name::<T>())
}

impl MathFormula {
    pub fn new(formular: String) -> Self {
        Self { formular }
    }

    pub fn process(&self) -> i32 {
        let mut stack: Vec<String> = Vec::new();
        let mut tmp = String::new();
        for c in self.formular.chars() {
            if c.ne(&')') {
                if c.is_numeric() {
                    tmp.push_str(&c.to_string());
                } else if c != '(' && !tmp.is_empty() {
                    stack.push(tmp.clone());
                    stack.push(c.to_string());
                    tmp.clear();
                } else {
                    stack.push(c.to_string());
                }
                if self.formular.chars().last().unwrap().eq(&c) {
                    stack.push(tmp.clone());
                    tmp.clear();
                }
            } else {
                stack.push(tmp.clone());
                tmp.clear();
                loop {
                    if let Ok(result) = self.cal(&mut stack) {
                        if stack.last().unwrap().eq("(") {
                            stack.pop();
                            stack.push(result.to_string());
                            break;
                        }
                        stack.push(result.to_string());
                    }
                }
            }
        }
        loop {
            if stack.len() == 1 {
                let result = stack.pop().unwrap();
                return result.parse().unwrap();
            }
            if let Ok(result) = self.cal(&mut stack) {
                stack.push(result.to_string());
            }
        }
    }

    fn cal<'b>(&self, stack: &mut Vec<String>) -> Result<i32, Box<dyn Error>> {
        let b: i32 = stack.pop().unwrap().parse()?;
        let op = stack.pop().unwrap();
        let a: i32 = stack.pop().unwrap().parse()?;
        self.match_op(a, b, op)
    }

    fn match_op(&self, a: i32, b: i32, op: String) -> Result<i32, Box<dyn Error>> {
        match op.as_str() {
            "+" => Ok(add(a, b)),
            "-" => Ok(sub(a, b)),
            "*" => Ok(mul(a, b)),
            "/" => Ok(div(a, b)),
            _ => panic!("Invalid operation"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_test() {
        let a = 1;
        let b = 2;
        assert_eq!(add(a, b), a + b)
    }

    #[test]
    fn sub_test() {
        let a = 1;
        let b = 2;
        assert_eq!(sub(a, b), a - b)
    }

    #[test]
    fn mul_test() {
        let a = 1;
        let b = 2;
        assert_eq!(mul(a, b), a * b)
    }

    #[test]
    fn div_test() {
        let a = 1;
        let b = 2;
        assert_eq!(div(a, b), a / b)
    }
}

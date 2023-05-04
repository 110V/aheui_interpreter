use std::string;

pub struct Stdin{
    pub input: String,
}

impl Stdin{
    pub fn new(input: &str) -> Self{
        Self{
            input: input.to_string()
        }
    }
    pub fn read_number(&mut self) -> i32{
        let mut buffer = string::String::new();
        self. input = self.input.trim_start().to_string();
        while self.input.len() != 0{
            if let Some(c) = self.input.chars().nth(0){
                if c.is_digit(10)||c=='-'{
                    buffer.push(c);
                    self.input.remove(0);
                }
                else{
                    break;
                }
            }
            else{
                return -1;
            }
        }
        
        buffer.parse().unwrap_or(-1)
    }

    pub fn read_char(&mut self)->i32{
        if self.input.len() == 0{
            return -1;
        }
        let c = self.input.remove(0);
        c as i32
    }
}


#[test]
fn test_read_number(){
    let mut stdin = Stdin::new("123");
    assert_eq!(stdin.read_number(),123);
}

#[test]
fn test_read_char(){
    let mut stdin = Stdin::new("abc");
    assert_eq!(stdin.read_char(),'a' as i32);
    assert_eq!(stdin.read_char(),'b' as i32);
    assert_eq!(stdin.read_char(),'c' as i32);
    assert_eq!(stdin.read_char(),-1);
    assert_eq!(stdin.read_char(),-1);
}
use crate::{Task,Tasks};

pub struct taskrow {
    taskscolumns: Vec<Option<Vec<u8>>>,
}
#[derive(Debug)]
pub enum DbError {
    Null,
    InvalidValue,
    InvalidUtf8,
    Io(std::io::Error),
}
impl taskrow {
    pub fn geti32(&self, index : usize) -> Result<i32 , DbError>{
            match self.taskscolumns[index].as_ref() {
                Some(value) => {
                    let bytes = value;
                    let bytes : [u8 ; 4 ] = bytes.as_slice().try_into().map_err(|_| DbError::InvalidValue)?;
                    return Ok(i32::from_be_bytes(bytes))
                }
                None => {return Err(DbError::InvalidValue)}
            
            }
            
    }
    pub fn getstring(&self , index : usize)->Result<String , DbError>{
        match self.taskscolumns[index].as_ref() {
            Some(value) => {
                return Ok(String::from_utf8(value.clone()).map_err(|_| DbError::InvalidValue)?)
            },
            None => {
                return Err(DbError::InvalidValue)
            }
        }
    }
    pub fn getbool(&self , index : usize)->Result<bool , DbError>{
         match self.taskscolumns[index].as_ref() {
            Some(value) => {
                match value.as_slice() {
                    [0] => Ok(false),
                    [1] => Ok(true) ,
                    _=> Err(DbError::InvalidValue)
                }
            },
            None => {
                return Err(DbError::InvalidValue)
            }
        }
    }

    pub fn extract(&self)->Result<Task , DbError>{
          Ok(Task {
            name : self.getstring(0)?, 
            id: self.geti32(1)? ,
            done :self.getbool(2)?  
        })
    }

}

enum Value {
    I16(i16),
    I32(i32),
    I64(i64),
    String(String),
    Bool(bool),
}

struct Feild {
    name : String , 
    type_IOD : u32 , 
}
struct Description {
    feilds : Vec<Feild>
}
struct Row {
    values : Vec<Value>
}

pub trait extract_details {
    fn extract_message(&mut self,  start : &mut usize)-> Vec<Option<Vec<u8>>>;
}


impl extract_details for Vec<u8> {
    //this transform the standart sql message (type , length , body) from u8 to vec<u8>
    fn extract_message(&mut self, start: &mut usize) -> Vec<Option<Vec<u8>>> {
    let mut message = Vec::new();

    // 1. Message type
    message.push(Some(vec![self[*start]]));
    if self[*start] == b'C' {return vec![] };
    // 2. Length
    let length_bytes = self[*start + 1..*start + 5].to_vec();

    let length = i32::from_be_bytes(
        length_bytes.clone().try_into().unwrap()
    );

    message.push(Some(length_bytes));

    // 3. Body
    let body_start = *start + 5;
    let body_end = body_start + (length as usize - 4);

    let body = self[body_start..body_end].to_vec();

    message.push(Some(body));

    // Move start to the beginning of the next message
    *start = body_end;

    message
}
    
}

trait trait_message {
    fn handle_description(& self ) -> Option<Description>;
    fn handle_row(&mut self , description : &Description ) -> Option<Row>;
}
impl trait_message for Vec<Option<Vec<u8>>> {
    fn handle_description(& self ) -> Option<Description>{
        match &self[0].as_ref().unwrap()[0] {
           b'T' => {
                 let mut new_desc = Description {
                    feilds : Vec::new()
                };

                let body = self[2].as_ref().unwrap();
                 
               let bytes = self[1].as_ref().unwrap();

                let length = i32::from_be_bytes(
                    bytes.as_slice().try_into().unwrap()
                );
                let mut pos = 0;

                let feilds_num = i16::from_be_bytes(
                    body[pos..pos + 2].try_into().ok()?
                );
                pos += 2;

               
                for i in 0..feilds_num {
                    let mut new_feild = Feild { name : "".to_string() , type_IOD : 0 };
                    loop {
                        if  body[pos] == b'\0' {break;}
                        new_feild.name.push(body[pos] as char);
                        pos += 1 ; 
                    }
                    pos += 1;
                    new_feild.type_IOD = u32 :: from_be_bytes(body[pos .. pos+4].try_into().ok()?);
                    pos += 4 ;
                    pos += 14 ; 
                    new_desc.feilds.push(new_feild);

                }
                return Some(new_desc)
            } , 
            _=> return None
        }
    }
  fn handle_row(&mut self, description: &Description) -> Option<Row> {
    if self[0].as_ref()?.first()? != &b'D' {
        return None;
    }

    let body = self[2].as_ref()?;
    let mut pos = 0;

    let fields_num = u16::from_be_bytes(
        body[pos..pos + 2].try_into().ok()?
    );

    pos += 2;

    if fields_num as usize != description.feilds.len() {
        return None;
    }

    let mut values = Vec::new();

    for field in &description.feilds {
        let value_length = i32::from_be_bytes(
            body[pos..pos + 4].try_into().ok()?
        );

        pos += 4;

        // NULL
        if value_length == -1 {
            return None; // your Value enum currently has no NULL variant
        }

        if value_length < 0 {
            return None;
        }

        let value_length = value_length as usize;

        if pos + value_length > body.len() {
            return None;
        }

        let raw_value = &body[pos..pos + value_length];

        let value = match field.type_IOD {
            21 => {
                // int2
                let bytes: [u8; 2] = raw_value.try_into().ok()?;
                Value::I16(i16::from_be_bytes(bytes))
            }

            23 => {
                // int4
                let bytes: [u8; 4] = raw_value.try_into().ok()?;
                Value::I32(i32::from_be_bytes(bytes))
            }

            20 => {
                // int8
                let bytes: [u8; 8] = raw_value.try_into().ok()?;
                Value::I64(i64::from_be_bytes(bytes))
            }

            25 | 1043 => {
                // text / varchar
                let string = String::from_utf8(raw_value.to_vec()).ok()?;
                Value::String(string)
            }

            16 => {
                // bool
                match raw_value {
                    [b't'] => Value::Bool(true),
                    [b'f'] => Value::Bool(false),
                    _ => return None,
                }
            }

            _ => {
                // Unsupported PostgreSQL type
                return None;
            }
        };

        values.push(value);

        pos += value_length;
    }

    Some(Row { values })
}

}
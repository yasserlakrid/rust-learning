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

pub enum Value {
    I16(i16),
    I32(i32),
    I64(i64),
    String(String),
    Bool(bool),
    Null,
    Bytes(Vec<u8>),
}

pub struct Feild {
    pub name : String , 
    pub type_IOD : u32 , 
}
pub struct Description {
    pub feilds : Vec<Feild>
}
pub struct Row {
    pub values : Vec<Value>
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

pub trait trait_message {
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
                    // Table OID (4) and attribute number (2) precede the
                    // column's type OID in a RowDescription field.
                    pos += 6;
                    new_feild.type_IOD = u32::from_be_bytes(
                        body.get(pos..pos + 4)?.try_into().ok()?
                    );
                    pos += 4;
                    // Type size (2), type modifier (4), format code (2).
                    pos += 8;
                    new_desc.feilds.push(new_feild);

                }
                return Some(new_desc)
            } , 
            _=> return None
        }
    }
 fn handle_row(&mut self, description: &Description) -> Option<Row> {
    // 1. Verify that this is a DataRow message ('D')
    if self.first()?.as_ref()?.first()? != &b'D' {
        return None;
    }

    // 2. Get the message body
    let body = self.get(2)?.as_ref()?;
    let mut pos = 0;

    // 3. Read the number of columns (Int16)
    let fields_num = u16::from_be_bytes(
        body.get(pos..pos + 2)?.try_into().ok()?
    );

    pos += 2;

    // 4. Verify that the number of columns matches RowDescription
    if fields_num as usize != description.feilds.len() {
        return None;
    }

    let mut values = Vec::new();

    // 5. Decode each column
    for field in &description.feilds {

        // Read the value length (Int32)
        let value_length =
            i32::from_be_bytes(body.get(pos..pos + 4)?.try_into().ok()?);

        pos += 4;

        // SQL NULL
        if value_length == -1 {
            values.push(Value::Null);
            continue;
        }

        if value_length < 0 {
            return None;
        }

        let value_length = value_length as usize;

        // Read the actual value bytes
        let raw_value = body.get(pos..pos.checked_add(value_length)?)?;

        // 6. Decode according to PostgreSQL type OID
        let value = match field.type_IOD {

            21 => {
                // INT2
                let s = std::str::from_utf8(raw_value).ok()?;
                Value::I16(s.parse().ok()?)
            }

            23 => {
                // INT4
                let s = std::str::from_utf8(raw_value).ok()?;
                Value::I32(s.parse().ok()?)
            }

            20 => {
                // INT8
                let s = std::str::from_utf8(raw_value).ok()?;
                Value::I64(s.parse().ok()?)
            }

            25 | 1043 => {
                // TEXT / VARCHAR
                let s = String::from_utf8(raw_value.to_vec()).ok()?;
                Value::String(s)
            }

            16 => {
                // BOOLEAN
                match raw_value {
                    [b't'] => Value::Bool(true),
                    [b'f'] => Value::Bool(false),
                    _ => return None,
                }
            }

            _ => Value::Bytes(raw_value.to_vec()),
        };

        values.push(value);

        // Move to the next column
        pos += value_length;
    }
    
    // 7. Return the decoded row
    Some(Row { values })
}
}
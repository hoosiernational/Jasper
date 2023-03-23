use std::collections::{HashMap, VecDeque};

#[derive(Clone, Eq, PartialEq, Debug)]
enum JsonToken {
    BeginObject,
    EndObject,
    BeginArray,
    EndArray,
    EndKey,
    EndValue,
    QuoteMark,
    QuotedValue(u8),
    LiteralValue(u8),
    TerminalError,
}

impl JsonToken {
    fn to_json_tokens(input: Vec<u8>) -> VecDeque<JsonToken> {
        let mut in_quote = false;
        let mut in_escape = false;
        let mut output = VecDeque::<JsonToken>::new();
        let mut i = 0_usize;
        loop {
            if i >= input.len() {
                break;
            } else {
                if in_quote {
                    if in_escape {
                        match input[i] {
                            0x22 => {
                                output.push_back(JsonToken::QuotedValue(0x22));
                            }
                            0x5C => {
                                output.push_back(JsonToken::QuotedValue(0x5C));
                            }
                            0x2F => {
                                output.push_back(JsonToken::QuotedValue(0x2F));
                            }
                            0x62 => {
                                output.push_back(JsonToken::QuotedValue(0x08));
                            }
                            0x66 => {
                                output.push_back(JsonToken::QuotedValue(0x0C));
                            }
                            0x6E => {
                                output.push_back(JsonToken::QuotedValue(0x0A));
                            }
                            0x72 => {
                                output.push_back(JsonToken::QuotedValue(0x0D));
                            }
                            0x74 => {
                                output.push_back(JsonToken::QuotedValue(0x09));
                            }
                            0x75 => {
                                i += 4;
                                if i >= input.len() {
                                    output.push_back(JsonToken::TerminalError);
                                } else {
                                    let mut val = 0_u16;
                                    let hex = [input[i - 3], input[i - 2], input[i - 1], input[i]];
                                    for j in 0..4 {
                                        val <<= 4;
                                        val |= match hex[j] {
                                            48 => 0,
                                            49 => 1,
                                            50 => 2,
                                            51 => 3,
                                            52 => 4,
                                            53 => 5,
                                            54 => 6,
                                            55 => 7,
                                            56 => 8,
                                            57 => 9,
                                            65 | 97 => 10,
                                            66 | 98 => 11,
                                            67 | 99 => 12,
                                            68 | 100 => 13,
                                            69 | 101 => 14,
                                            70 | 102 => 15,
                                            _ => {
                                                output.push_back(JsonToken::TerminalError);
                                                i = input.len();
                                                0
                                            }
                                        }
                                    }
                                    output.push_back(JsonToken::QuotedValue((val >> 8) as u8));
                                    output.push_back(JsonToken::QuotedValue((val & 255) as u8));
                                }
                            }
                            _ => {
                                output.push_back(JsonToken::TerminalError);
                                i = input.len()
                            }
                        }
                        in_escape = false;
                    } else {
                        match input[i] {
                            0x22 => {
                                in_quote = false;
                                output.push_back(JsonToken::QuoteMark);
                            }
                            0x5C => {
                                in_escape = true;
                            }
                            r => {
                                output.push_back(JsonToken::QuotedValue(r));
                            }
                        }
                    }
                } else {
                    if in_escape {
                        match input[i] {
                            0x22 => {
                                output.push_back(JsonToken::LiteralValue(0x22));
                            }
                            0x5C => {
                                output.push_back(JsonToken::LiteralValue(0x5C));
                            }
                            0x2F => {
                                output.push_back(JsonToken::LiteralValue(0x2F));
                            }
                            0x62 => {
                                output.push_back(JsonToken::LiteralValue(0x08));
                            }
                            0x66 => {
                                output.push_back(JsonToken::LiteralValue(0x0C));
                            }
                            0x6E => {
                                output.push_back(JsonToken::LiteralValue(0x0A));
                            }
                            0x72 => {
                                output.push_back(JsonToken::LiteralValue(0x0D));
                            }
                            0x74 => {
                                output.push_back(JsonToken::LiteralValue(0x09));
                            }
                            0x75 => {
                                i += 4;
                                if i >= input.len() {
                                    {
                                        output.push_back(JsonToken::TerminalError);
                                        i = input.len()
                                    }
                                } else {
                                    let mut val = 0_u16;
                                    let hex = [input[i - 3], input[i - 2], input[i - 1], input[i]];
                                    for j in 0..4 {
                                        val <<= 4;
                                        val |= match hex[j] {
                                            48 => 0,
                                            49 => 1,
                                            50 => 2,
                                            51 => 3,
                                            52 => 4,
                                            53 => 5,
                                            54 => 6,
                                            55 => 7,
                                            56 => 8,
                                            57 => 9,
                                            65 | 97 => 10,
                                            66 | 98 => 11,
                                            67 | 99 => 12,
                                            68 | 100 => 13,
                                            69 | 101 => 14,
                                            70 | 102 => 15,
                                            _ => {
                                                output.push_back(JsonToken::TerminalError);
                                                i = input.len();
                                                0
                                            }
                                        }
                                    }
                                    output.push_back(JsonToken::LiteralValue((val >> 8) as u8));
                                    output.push_back(JsonToken::LiteralValue((val & 255) as u8));
                                }
                            }
                            _ => {
                                output.push_back(JsonToken::TerminalError);
                                i = input.len();
                            }
                        }
                        in_escape = false;
                    } else {
                        match input[i] {
                            0x20 | 0x09 | 0x0A | 0x0D => {}
                            0x7B => {
                                output.push_back(JsonToken::BeginObject);
                            }
                            0x7D => {
                                output.push_back(JsonToken::EndObject);
                            }
                            0x5B => {
                                output.push_back(JsonToken::BeginArray);
                            }
                            0x5D => {
                                output.push_back(JsonToken::EndArray);
                            }
                            0x3A => {
                                output.push_back(JsonToken::EndKey);
                            }
                            0x2C => {
                                output.push_back(JsonToken::EndValue);
                            }
                            0x22 => {
                                in_quote = true;
                                output.push_back(JsonToken::QuoteMark);
                            }
                            0x5C => {
                                in_escape = true;
                            }
                            r => {
                                output.push_back(JsonToken::LiteralValue(r));
                            }
                        }
                    }
                }
                i += 1;
            }
        }
        output
    }
}

///A structure for holding a DateTime, having been parsed or intended to parsed to standard
///JavaScript format for dates (YYYY-MM-DDTHH:mm:ss)
pub struct JsonDate {
    ///The actual year
    year: u16,

    ///The actual month (1 is January, 12 is December)
    month: u8,

    ///The actual date (e.g. 31 for January 31st)
    date: u8,

    ///The hour on a 24-hour scale (where 0 is midnight and 23 is 11 PM)
    hour: u8,

    ///The minute, zero-indexed
    minute: u8,

    ///The second, zero-indexed
    second: u8,
}

impl JsonDate {
    ///Generates a JsonDate from a typical JavaScript date formatted string. It
    ///is tolerant of missing values (for example, 2023-06-06 will be parsed just fine)
    pub fn from_str(input: &str) -> Option<JsonDate> {
        let datetimesplit = input.split("T").collect::<Vec<&str>>();
        let datesplit = datetimesplit[0].split("-").collect::<Vec<&str>>();
        let timesplit = datetimesplit
            .get(1)
            .unwrap_or(&"")
            .split(":")
            .collect::<Vec<&str>>();
        let year_string = datesplit.get(0).unwrap_or(&"");
        let month_string = datesplit.get(1).unwrap_or(&"");
        let date_string = datesplit.get(2).unwrap_or(&"");
        let (hour_string, minute_string, second_string) = (
            timesplit.get(0).unwrap_or(&""),
            timesplit.get(1).unwrap_or(&""),
            timesplit.get(2).unwrap_or(&""),
        );
        match (
            year_string.parse::<u16>(),
            month_string.parse::<u8>(),
            date_string.parse::<u8>(),
        ) {
            (Ok(year), Ok(month), Ok(date)) => {
                let hour = hour_string.parse::<u8>().unwrap_or(0);
                let minute = minute_string.parse::<u8>().unwrap_or(0);
                let second = second_string.parse::<u8>().unwrap_or(0);
                Some(Self {
                    year,
                    month,
                    date,
                    hour,
                    minute,
                    second,
                })
            }
            _ => None,
        }
    }

    fn four_digits(input: u16) -> String {
        let input = input % 10000;
        if input > 999 {
            format!("{}", input)
        } else if input > 99 {
            format!("0{}", input)
        } else if input > 9 {
            format!("00{}", input)
        } else {
            format!("000{}", input)
        }
    }

    fn two_digits(input: u8) -> String {
        let input = input % 100;
        if input > 9 {
            format!("{}", input)
        } else {
            format!("0{}", input)
        }
    }

    ///Converts this date to a typical JavaScript DateTime formatted string
    pub fn to_string(&self) -> String {
        format!(
            "{}-{}-{}T{}:{}:{}",
            Self::four_digits(self.year),
            Self::two_digits(self.month),
            Self::two_digits(self.date),
            Self::two_digits(self.hour),
            Self::two_digits(self.minute),
            Self::two_digits(self.second)
        )
    }
}

struct JsonParserUtilities();

impl JsonParserUtilities {
    fn parse_quote(input: &mut VecDeque<JsonToken>) -> Option<JsonValue> {
        let mut bytes: Vec<u8> = vec![];
        loop {
            match input.pop_front() {
                Some(JsonToken::QuotedValue(u)) => {
                    bytes.push(u);
                }
                Some(JsonToken::QuoteMark) => {
                    break;
                }
                _ => {
                    return None;
                }
            }
        }
        match String::from_utf8(bytes) {
            Ok(str) => Some(JsonValue::String(str)),
            _ => None,
        }
    }

    fn parse_literal(input: &mut VecDeque<JsonToken>, first_value: u8) -> Option<JsonValue> {
        let mut bytes: Vec<u8> = vec![first_value];
        loop {
            match input.front() {
                Some(JsonToken::LiteralValue(u)) => {
                    bytes.push(*u);
                    input.pop_front().unwrap();
                }
                _ => {
                    break;
                }
            }
        }
        match String::from_utf8(bytes) {
            Ok(str) => {
                let str = str.to_lowercase();
                if str == "true" {
                    Some(JsonValue::Boolean(true))
                } else if str == "false" {
                    Some(JsonValue::Boolean(false))
                } else if str == "null" {
                    Some(JsonValue::Null)  
                } else {
                    match str.parse::<f64>() {
                        Ok(f) => Some(JsonValue::Number(f)),
                        _ => None,
                    }
                }
            }
            _ => None,
        }
    }

    fn parse_array(input: &mut VecDeque<JsonToken>) -> Option<JsonValue> {
        let mut v: Vec<JsonValue> = Vec::new();
        loop {
            match input.pop_front() {
                None => {
                    return None;
                }
                Some(JsonToken::BeginArray) => match Self::parse_array(input) {
                    None => {
                        return None;
                    }
                    Some(arr) => {
                        v.push(arr);
                    }
                },
                Some(JsonToken::BeginObject) => match Self::parse_object(input) {
                    None => {
                        return None;
                    }
                    Some(obj) => {
                        v.push(obj);
                    }
                },
                Some(JsonToken::LiteralValue(u)) => match Self::parse_literal(input, u) {
                    None => {
                        return None;
                    }
                    Some(litval) => {
                        v.push(litval);
                    }
                },
                Some(JsonToken::QuoteMark) => match Self::parse_quote(input) {
                    None => {
                        return None;
                    }
                    Some(quote) => {
                        v.push(quote);
                    }
                },
                Some(JsonToken::EndArray) => {
                    break;
                }
                Some(JsonToken::EndValue) => {}
                _ => {
                    return None;
                }
            }
        }
        if input.front() == Some(&JsonToken::EndValue) {
            input.pop_front();
        }
        Some(JsonValue::Array(v))
    }

    fn parse_object(input: &mut VecDeque<JsonToken>) -> Option<JsonValue> {
        let mut v: HashMap<String, JsonValue> = HashMap::new();
        loop {
            let key: String;
            match input.pop_front() {
                Some(JsonToken::EndObject) => {
                    break;
                }
                Some(JsonToken::QuoteMark) => {
                    let mut bytes: Vec<u8> = vec![];
                    loop {
                        match input.pop_front() {
                            Some(JsonToken::QuotedValue(u)) => {
                                bytes.push(u);
                            }
                            Some(JsonToken::QuoteMark) => {
                                break;
                            }
                            _ => {
                                return None;
                            }
                        }
                    }
                    match String::from_utf8(bytes) {
                        Ok(str) => key = str,
                        _ => {
                            return None;
                        }
                    }
                }
                _ => {
                    return None;
                }
            }
            if input.pop_front() != Some(JsonToken::EndKey) {
                return None;
            }
            let value = match input.pop_front() {
                Some(JsonToken::BeginArray) => Self::parse_array(input),
                Some(JsonToken::BeginObject) => Self::parse_object(input),
                Some(JsonToken::LiteralValue(u)) => Self::parse_literal(input, u),
                Some(JsonToken::QuoteMark) => Self::parse_quote(input),
                _ => None,
            };
            if input.front() == Some(&JsonToken::EndValue) {
                input.pop_front().unwrap();
            }
            if v.contains_key(&key) {
                return None;
            } else {
                match value {
                    Some(value) => {
                        v.insert(key, value);
                    }
                    _ => {
                        return None;
                    }
                }
            }
        }
        Some(JsonValue::Object(v))
    }
}

///A JSON value
#[derive(Clone, Debug)]
pub enum JsonValue {
    ///A null value
    Null,

    ///A boolean value
    Boolean(bool),

    ///A number value, which is a decimal (f64 in Rust)
    Number(f64),

    ///A string value
    String(String),

    ///An array value
    Array(Vec<JsonValue>),

    ///An object value
    Object(std::collections::HashMap<String, JsonValue>),
}

impl JsonValue {
    ///Creates an object instance from a list of String,JsonValue (key,value) tuples.
    pub fn build_object(input: Vec<(String, JsonValue)>) -> Self {
        Self::Object(input.into_iter().collect::<HashMap<String, JsonValue>>())
    }

    fn stringify_actual_string(input: &str) -> String {
        let mut v: Vec<u8> = vec![34];
        input.as_bytes().clone().into_iter().for_each(|x| match x {
            0x22 => {
                v.push(0x5C);
                v.push(0x22);
            }
            0x5C => {
                v.push(0x5C);
                v.push(0x5C);
            }
            0x2F => {
                v.push(0x5C);
                v.push(0x2F);
            }
            0x08 => {
                v.push(0x5C);
                v.push(0x62);
            }
            0x0C => {
                v.push(0x5C);
                v.push(0x66);
            }
            0x0A => {
                v.push(0x5C);
                v.push(0x6E);
            }
            0x0D => {
                v.push(0x5C);
                v.push(0x72);
            }
            0x09 => {
                v.push(0x5C);
                v.push(0x74);
            }
            r => {
                v.push(*r);
            }
        });
        v.push(34);
        String::from_utf8(v).unwrap()
    }

    ///Works same as the stringify method for objects in JavaScript
    pub fn stringify(&self) -> String {
        match self {
            Self::Null => "null".to_owned(),
            Self::Boolean(true) => "true".to_owned(),
            Self::Boolean(false) => "false".to_owned(),
            Self::Number(the_f64) => format!("{}", the_f64),
            Self::String(str) => Self::stringify_actual_string(str),
            Self::Array(arr) => {
                format!(
                    "[{}]",
                    arr.clone()
                        .into_iter()
                        .map(|x| x.stringify())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Self::Object(obj) => {
                format!(
                    "{{{}}}",
                    obj.clone()
                        .into_iter()
                        .map(|(x, y)| format!(
                            "{}: {}",
                            Self::stringify_actual_string(&x),
                            y.stringify()
                        ))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
        }
    }

    ///If the value is a boolean, returns Some(that boolean), else None
    pub fn get_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(b) => Some(*b),
            Self::Number(n) => Some(*n != 0_f64),
            Self::String(s) => Some(s != "0" && s != ""),
            Self::Array(a) => {
                if a.len() == 1 {
                    Self::get_boolean(&a[0])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    ///If the value is a number, returns Some(that number), else None
    pub fn get_number(&self) -> Option<f64> {
        match self {
            Self::Boolean(b) => Some(if *b { 1_f64 } else { 0_f64 }),
            Self::Number(n) => Some(*n),
            Self::String(s) => match s.parse::<f64>() {
                Ok(n) => Some(n),
                _ => None,
            },
            Self::Array(a) => {
                if a.len() == 1 {
                    Self::get_number(&a[0])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    ///If the value is a string, returns Some(that string), else None. Note that
    ///the string is cloned, so the original value is not borrowed
    pub fn get_string(&self) -> Option<String> {
        match self {
            Self::Boolean(b) => Some(if *b {
                "true".to_owned()
            } else {
                "false".to_owned()
            }),
            Self::Number(n) => Some(format!("{}", n)),
            Self::String(s) => Some(s.clone()),
            Self::Array(a) => {
                if a.len() == 1 {
                    Self::get_string(&a[0])
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    ///If the value is a array, returns a clone of that array; else returns an
    ///empty array
    pub fn get_array(&self) -> Vec<JsonValue> {
        match self {
            Self::Null => vec![JsonValue::Null],
            Self::Boolean(b) => vec![JsonValue::Boolean(*b)],
            Self::Number(n) => vec![JsonValue::Number(*n)],
            Self::String(s) => vec![JsonValue::String(s.clone())],
            Self::Array(arr) => arr.clone(),
            Self::Object(obj) => obj
                .values()
                .clone()
                .into_iter()
                .map(|x| x.clone())
                .collect::<Vec<JsonValue>>(),
        }
    }

    ///If the value is an object, returns Some(a clone of that object's inner string ->
    ///JsonValue hashmap); else returns None
    pub fn get_object(&self) -> Option<HashMap<String, JsonValue>> {
        match self {
            Self::Object(obj) => Some(obj.clone()),
            _ => None,
        }
    }

    ///Returns TRUE if and only if the value is null
    pub fn is_null(&self) -> bool {
        match self {
            Self::Null => true,
            _ => false,
        }
    }

    ///Returns Some(a u64 integer) if the value can be parsed as such; else None
    pub fn get_integer(&self) -> Option<u64> {
        match self.get_string() {
            None => match self.get_number(){
                Some(f) => if ((f as u64) as f64) == f {
                    Some(f as u64)
                } else {
                    None
                }
                _ => None,
            },
            Some(s) => match s.parse::<u64>() {
                Ok(u) => Some(u),
                _ => None,
            },
        }
    }

    ///Returns Some(a JsonDate) if the value can be parsed as such; else None. Note
    ///that this only works with String values that approximate the expected format.
    ///(See `JsonDate`)
    pub fn get_json_date(&self) -> Option<JsonDate> {
        match self {
            Self::String(str) => JsonDate::from_str(str),
            _ => None,
        }
    }

    ///Parses a JsonValue from an input array of bytes
    pub fn parse(input: Vec<u8>) -> Option<JsonValue> {
        let mut tokens = JsonToken::to_json_tokens(input);
        match tokens.pop_front() {
            Some(JsonToken::BeginArray) => JsonParserUtilities::parse_array(&mut tokens),
            Some(JsonToken::BeginObject) => JsonParserUtilities::parse_object(&mut tokens),
            Some(JsonToken::LiteralValue(u)) => JsonParserUtilities::parse_literal(&mut tokens, u),
            Some(JsonToken::QuoteMark) => JsonParserUtilities::parse_quote(&mut tokens),
            _ => None,
        }
    }
}

///JsonValidators hold validation parameters for parsing JSON inputs. For example,
///JsonValidator::String(|x| t.len() == 1) can be used to validate that a string
///is only of length 1
#[derive(Clone)]
pub enum JsonValidator {
    ///Validates any Json Value
    RubberStamp,

    ///Validates only null Json Values
    Null,

    ///Validates only boolean values that fit a functional criteria
    Boolean(fn(&bool) -> bool),

    ///Validates only float values that fit a functional criteria
    Number(fn(&f64) -> bool),

    ///Validates only string values that fit a functional criteria
    String(fn(&str) -> bool),

    ///Validates only integer values (may be in form of string or number) that fit a functional
    ///criteria
    Integer(fn(&u64) -> bool),
    
    ///Validates only DateTime values (strings) that fit a functional criteria
    DateTime(fn(&JsonDate) -> bool),

    ///Validates an array based on whether every value within it is validated by its internal
    ///JsonValidator. Note that this internal JsonValidator must be boxed. Also note that
    ///an Array validator will automatically validate an empty array
    Array(Box<JsonValidator>),

    ///Validates an object based on whether its key,value pairs are validated by the corresponding
    ///key,value validators. Key,value pairs not mentioned in the validator are not checked
    Object(Vec<(String, JsonValidator)>),

    ///Holds multiple validators, and will return true if any of its internal members return true
    Or(Vec<JsonValidator>),
}

impl JsonValidator {
    ///Validates a JsonValue
    pub fn validate(&self, input: &JsonValue) -> bool {
        match self {
            JsonValidator::RubberStamp => true,
            JsonValidator::Null => input.is_null(),
            JsonValidator::Boolean(f) => match input.get_boolean() {
                None => false,
                Some(b) => f(&b),
            },
            JsonValidator::Number(f) => match input.get_number() {
                None => false,
                Some(n) => f(&n),
            },
            JsonValidator::String(f) => match input.get_string() {
                None => false,
                Some(s) => f(&s),
            },
            JsonValidator::Integer(f) => match input.get_integer() {
                None => false,
                Some(i) => f(&i),
            },
            JsonValidator::DateTime(f) => match input.get_json_date() {
                None => false,
                Some(d) => f(&d),
            },
            JsonValidator::Array(f) => {
                input
                    .get_array()
                    .into_iter()
                    .filter(|x| !f.validate(x))
                    .count()
                    == 0
            }
            JsonValidator::Object(v) => {
                let mut hm = HashMap::with_capacity(v.len());
                for i in 0..v.len() {
                    hm.insert(v[i].0.clone(), v[i].1.clone());
                }
                match input.get_object() {
                    None => false,
                    Some(tbt) => {
                        if hm
                            .keys()
                            .into_iter()
                            .filter(|x| !tbt.contains_key(x.to_owned()))
                            .count()
                            > 0
                        {
                            false
                        } else {
                            hm.into_iter()
                                .map(|(x, y)| y.validate(tbt.get(&x).unwrap()))
                                .filter(|z| !z)
                                .count()
                                == 0
                        }
                    }
                }
            }
            JsonValidator::Or(v) => v.clone().into_iter().filter(|x| x.validate(input)).count() > 0,
        }
    }
}

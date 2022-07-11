enum NumberTypes {
    Integer(i64),
    Float(f64)
}

enum JsonTypes {
    Number(NumberTypes),
    String,
    Object,
    Vector
}

fn match_first_literal<'a>(expected: &'static str) 
-> impl Fn(&'a str) -> Result<&'a str, &str>
{
    /*
        como esse valor do expected pode ser usado em qualquer
        momento por essa closure, então esse valor precisa ter um lifetime
        static, pra que ele não seja movido ou liberado da
        memória antes de ser usado por essa função
    */
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(v) if v == expected =>
            Ok(&input[expected.len()..]),
        _ => Err("teste")
    }
}

fn zero_or_more<'a, T>(parser: T) 
    -> impl Fn(&'a str) -> Result<&'a str, &str>
    where
        T: Fn(char) -> bool
{
    move |input: &'a str| {
        let mut result = 0;
 
        for c in input.chars() {
            match parser(c) {
                true => result += 1,
                false => break
            }
        }

        Ok(&input[result..])
    }
}

pub fn attribute_name(input: &str) -> Result<String, &str> {
    // TODO write the code to get the JSON attribute name 

    let match_dbquote = match_first_literal("\"");
    let mut attr_name: Vec<char> = Vec::new();    

    if let Ok(v) = match_dbquote(input) {
        for c in v.chars() {
            if !c.is_alphanumeric() {
                break;
            }

            attr_name.push(c);
        }
    } else {
        return Err("Error trying to parse the input");
    }

    Ok(attr_name.into_iter().collect())
}

fn verify_attr_type() {
    // TODO make this to see which is the type of JSON attribute
}

fn create_struct(input: String) {
    // ? transforms

    // * Element

    // TODO create the structs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_literal() {
        let parser = match_first_literal("\"");

        assert_eq!(Ok("user\""), parser("\"user\""))
    }

    #[test]
    fn test_attribute_name() {
        assert_eq!(
            Ok(String::from("user")),
            attribute_name("\"user\"")
        )
    }

    #[test]
    fn test_zero_or_more() {
        assert_eq!(
            Ok("anything"),
            zero_or_more(|c| c.is_whitespace())("   anything")
        )
    }

}

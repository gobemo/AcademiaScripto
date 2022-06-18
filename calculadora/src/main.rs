use regex::Regex;

fn main() {
    
    //Declarando las expresiones xs
    let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();
    let re_res = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();
    let re_div = Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
    let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();
    
    
    //Ingreso datos del usuario

    println!("Ingrese la expresion matematica");
    let mut expression = String::new();
    std::io::stdin().read_line(&mut expression).unwrap();

    // División
    loop{
        let caps = re_div.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let operation: i32 = left_value / rigth_value;
        expression = expression.replace(cap_expression, &operation.to_string());
    }

    // Multiplicación
    loop{
        let caps = re_mul.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let operation: i32 = left_value * rigth_value;
        expression = expression.replace(cap_expression, &operation.to_string());
    }

    // Resta
    loop{
        let caps = re_res.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let operation: i32 = left_value - rigth_value;
        expression = expression.replace(cap_expression, &operation.to_string());
    }

    // Suma
    loop{
        let caps = re_add.captures(expression.as_str());
        if caps.is_none() {
            break;
        }
        let caps = caps.unwrap();
        let cap_expression = caps.get(0).unwrap().as_str();
        let left_value : i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let rigth_value : i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        println!("{:?} izq: {} der: {}",caps,left_value,rigth_value);
        let operation: i32 = left_value + rigth_value;
        expression = expression.replace(cap_expression, &operation.to_string());
    }

    // Mostrar resultados
    println!("Resultado: {}", expression);
}


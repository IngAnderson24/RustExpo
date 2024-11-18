use std::str::FromStr;
fn main() {
    println!("Bienvenido al Programa del Triángulo");
    println!("
Ingrese 3 lados e identificaremos el tipo de triángulo.");
    println!("{}",determinar_tipo());
    }

    fn determinar_tipo() -> String {
        println!("Valor del primer lado: ");
        let l1: f32 = leer_lado();
        println!("Valor del segundo lado: ");
        let l2: f32 = leer_lado();
        println!("Valor del tercer lado: ");
        let l3: f32 = leer_lado();
        let mut resultado: String = "Este triángulo es ".to_owned();
        let mut tipo: String = "".to_owned();
        let rectangulo = (l1*l1 == l2*l2 + l3*l3) || (l2*l2 == l1*l1 + l3*l3) || (l3*l3 == l1*l1 + l2*l2);
            if l1 == l2  && l2 == l3 {
                tipo = tipo + "Equilatero";
            }
            else if l1 != l2 && l2 != l3 && l1 != l3{
                tipo = tipo + "Escaleno";
                }
            else {
                tipo = tipo + "Isósceles";
            }
    
            if rectangulo{
                tipo = tipo + " Rectángulo";
            }
            resultado.push_str(&tipo);
        resultado.to_string()
    }

fn leer_linea() -> String {
    let mut input = String::new();
    std::io::stdin().read_line( &mut input).expect("No se puede leer este dato.");
    input.trim().to_string()
}

fn leer_lado() -> f32 {
    let input = leer_linea();
    f32::from_str(&input).expect("Ha ocurrido un error, este lado no es un número.")
}
/*

A fórmula para calcular a área de uma circunferência é: area = π . raio2. 
Considerando para este problema que π = 3.14159:

- Efetue o cálculo da área, elevando o valor de raio ao quadrado e 
multiplicando por π.

Entrada
A entrada contém um valor de ponto flutuante (dupla precisão), no caso, 
a variável raio.

Saída
Apresentar a mensagem "A=" seguido pelo valor da variável area, 
conforme 
exemplo abaixo, com 4 casas após o ponto decimal.
Utilize variáveis de dupla precisão (double). Como todos os problemas,
 não esqueça de imprimir o fim de linha após o resultado, caso contrário, você receberá "Presentation Error".

*/

const PI: f64 = 3.14159;

pub fn circle_area(r: f64) -> String {
    let result = PI * r * r;

    format!("A={:.4}\n", result)
}

pub struct Circle {
    radius: f64
}

impl Circle {
    pub fn area(&self) -> f64 {
        f64::powf(self.radius, 2.0) * PI
    }

    pub fn string_area(&self) -> String {
        let result = self.area();

        format!("A={:.4}\n", result)
    }

    pub fn new(radius: f64) -> Result<Self, String> {
        if radius < 0.0 {
            Err("Radius should be equal or greater than 0".to_string())
        } else {
            Ok(Self { radius })
        }
    }
}

pub fn circle_area_value(circle: Circle) -> f64 {
    circle.area()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn area_radius_zero() {
        let result = circle_area(0.0);
        assert_eq!(result, "A=0.0000\n");
    }

    #[test]
    fn area_radius_one() {
        let result = circle_area(1.0);
        assert_eq!(result, "A=3.1416\n");
    }

    #[test]
    fn area_radius_float_value() {
        let result = circle_area(5.1234);
        assert_eq!(result, "A=82.4643\n");
    }

    #[test]
    fn area_radius_circle_area_value() {
        let circle = Circle{radius: 0.0};
        let result = circle_area_value(circle);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn area_radius_circle_area_string() {
        let circle = Circle{radius: 5.1234};
        let result = circle.string_area();
        assert_eq!(result, "A=82.4643\n");
    }

    #[test]
    fn test_new() {
        let circle = Circle::new(5.1234).expect("invalid circle");
        let result = circle.string_area();
        assert_eq!(result, "A=82.4643\n");
    }
}

use std::env;
use std::io::{self, Write};

#[derive(Clone, Copy)]
struct Unit {
    name: &'static str,
    size_m: f64,
    source: &'static str,
    show_whole_count: bool,
}

// Medidas con fuentes. Cambialas aca.
const UNITS: &[Unit] = &[
    Unit {
        name: "plátanos",
        size_m: 0.18,
        source: "USDA: plátano Cavendish mediano 17-20 cm",
        show_whole_count: true,
    },
    Unit {
        name: "manzanas",
        size_m: 0.075,
        source: "Wikipedia/Malus domestica: 7.0-8.3 cm diametro comercial",
        show_whole_count: true,
    },
    Unit {
        name: "Big Macs",
        size_m: 0.09,
        source: "WhatsNeue 2018: 6.9 cm alto x 10.4 cm diametro medido con calibre",
        show_whole_count: true,
    },
    Unit {
        name: "patitos de hule",
        size_m: 0.10,
        source: "Estandar comercial: 10 cm alto (4 pulgadas), fabricantes varios",
        show_whole_count: true,
    },
    Unit {
        name: "tiburones blancos",
        size_m: 4.5,
        source: "CSULB Shark Lab: adultos promedio 4.3-4.5 m (hembras 4.5-5 m)",
        show_whole_count: false,
    },
    Unit {
        name: "jirafas",
        size_m: 5.0,
        source: "Guinness/San Diego Zoo: machos adultos 4.6-5.5 m",
        show_whole_count: false,
    },
    Unit {
        name: "New Routemaster",
        size_m: 11.1,
        source: "TfL/Wikipedia: New Routemaster londinense 11.1 m de largo",
        show_whole_count: false,
    },
    Unit {
        name: "ballenas azules",
        size_m: 24.0,
        source: "Wikipedia/Monterey Bay Aquarium: adulto promedio 24-25 m",
        show_whole_count: false,
    },
    Unit {
        name: "campos de fútbol",
        size_m: 105.0,
        source: "Reglamento FIFA: 105 m de largo",
        show_whole_count: false,
    },
    Unit {
        name: "Torres Eiffel",
        size_m: 330.0,
        source: "Sitio oficial tour-eiffel.paris: 330 m con antena",
        show_whole_count: false,
    },
];

fn format_equivalent(value: f64) -> String {
    if value >= 0.01 {
        format!("{value:.2}")
    } else if value >= 0.0001 {
        format!("{value:.4}")
    } else {
        format!("{value:.2e}")
    }
}

fn parse_height(raw: &str) -> Result<f64, &'static str> {
    let normalized = raw.trim().replace(',', ".");
    if normalized.is_empty() {
        return Err("No recibí ninguna altura.");
    }

    let height = normalized
        .parse::<f64>()
        .map_err(|_| "No entendí la altura. Escríbela en metros, por ejemplo 1.60.")?;

    if !height.is_finite() || height <= 0.0 {
        return Err("La altura debe ser un número mayor que cero.");
    }

    Ok(height)
}

fn print_usage(program: &str) {
    println!("Uso: {program} [ALTURA_EN_METROS]");
    println!("Ejemplos: {program} 1.60");
    println!("          {program} 1,81");
    println!("Sin argumento, el programa te pedirá la altura.");
}

fn main() {
    let mut args = env::args();
    let program = args
        .next()
        .unwrap_or_else(|| "alturometro".to_string());
    let args: Vec<String> = args.collect();

    if args.len() == 1 && (args[0] == "--help" || args[0] == "-h") {
        print_usage(&program);
        return;
    }

    if args.len() > 1 {
        eprintln!("Solo acepta una altura en metros.");
        print_usage(&program);
        std::process::exit(2);
    }

    let raw_height = if let Some(argument) = args.first() {
        argument.clone()
    } else {
        print!("¿Cuánto mides en metros? (por ejemplo, 1.60): ");
        if let Err(error) = io::stdout().flush() {
            eprintln!("No pude mostrar el mensaje: {error}");
            std::process::exit(1);
        }

        let mut input = String::new();
        if let Err(error) = io::stdin().read_line(&mut input) {
            eprintln!("No pude leer la altura: {error}");
            std::process::exit(1);
        }
        input
    };

    let height_m = match parse_height(&raw_height) {
        Ok(height) => height,
        Err(message) => {
            eprintln!("Error: {message}");
            eprintln!("Usa, por ejemplo: {program} 1.60");
            std::process::exit(2);
        }
    };

    println!("\n=== ALTUROMETRO MEME ===");
    println!("Estatura: {height_m:.2} m ({:.0} cm)\n", height_m * 100.0);

    for unit in UNITS {
        let equivalent = height_m / unit.size_m;
        let amount = format_equivalent(equivalent);
        print!("* {amount} {} ({})", unit.name, unit.source);

        if unit.show_whole_count {
            let whole_units = equivalent.ceil() as u64;
            print!(" -> {whole_units} piezas para alcanzar o superar la altura");
        }

        println!();
    }

    println!("\nNota: todo es aproximado. Los tamanios varian y las equivalencias son para divertirse.");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acepta_punto_y_coma_decimal() {
        assert!((parse_height("1.60").unwrap() - 1.60).abs() < 1e-12);
        assert!((parse_height("1,60").unwrap() - 1.60).abs() < 1e-12);
    }

    #[test]
    fn rechaza_alturas_no_positivas_o_invalidas() {
        assert!(parse_height("0").is_err());
        assert!(parse_height("-1.7").is_err());
        assert!(parse_height("manzana").is_err());
    }

    #[test]
    fn no_redondea_a_cero_las_unidades_muy_grandes() {
        assert_eq!(format_equivalent(1.60 / 330.0), "0.0048");
    }

    #[test]
    fn calcula_las_equivalencias_de_160_m() {
        let banana = UNITS.iter().find(|u| u.name == "plátanos").unwrap();
        let apple = UNITS.iter().find(|u| u.name == "manzanas").unwrap();

        let bananas = 1.60 / banana.size_m;
        let apples = 1.60 / apple.size_m;

        // 1.60 / 0.18 = 8.888...
        assert!((bananas - 8.8888888889).abs() < 1e-8);
        assert_eq!(bananas.ceil() as u64, 9);
        // 1.60 / 0.075 = 21.333...
        assert!((apples - 21.3333333333).abs() < 1e-8);
        assert_eq!(apples.ceil() as u64, 22);
    }
}
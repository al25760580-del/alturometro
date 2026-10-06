slint::include_modules!();

const UNITS: &[(&str, &str, f64, bool, &str)] = &[
    ("\u{1F34C}", "platanos", 0.18, true, "USDA: platano Cavendish mediano 17-20 cm"),
    ("\u{1F34E}", "manzanas", 0.075, true, "Wikipedia: 7.0-8.3 cm diametro comercial"),
    ("\u{1F354}", "Big Macs", 0.09, true, "WhatsNeue 2018: 6.9 cm alto medido con calibre"),
    ("\u{1F986}", "patitos de hule", 0.10, true, "Estandar comercial: 10 cm alto (4 pulgadas)"),
    ("\u{1F988}", "tiburones blancos", 4.5, false, "CSULB Shark Lab: adultos promedio 4.3-4.5 m"),
    ("\u{1F992}", "jirafas", 5.0, false, "Guinness/San Diego Zoo: machos adultos 4.6-5.5 m"),
    ("\u{1F68C}", "New Routemaster", 11.1, false, "TfL: New Routemaster londinense 11.1 m"),
    ("\u{1F40B}", "ballenas azules", 24.0, false, "Wikipedia/Monterey Bay: adulto promedio 24-25 m"),
    ("\u{26BD}", "campos de futbol", 105.0, false, "Reglamento FIFA: 105 m de largo"),
    ("\u{1F5FC}", "Torres Eiffel", 330.0, false, "tour-eiffel.paris: 330 m con antena"),
];

fn fmt_eq(v: f64) -> String {
    if v >= 0.01 {
        format!("{:.2}", v)
    } else if v >= 0.0001 {
        format!("{:.4}", v)
    } else {
        format!("{:.2e}", v)
    }
}

fn units_for_height(height: f64) -> slint::ModelRc<UnitItem> {
    let items: Vec<UnitItem> = UNITS
        .iter()
        .map(|&(emoji, name, size, has_whole, source)| {
            let eq = height / size;
            let c = eq.ceil() as u64;
            let s = |n: u64| if n != 1 { "s" } else { "" };
            UnitItem {
                emoji: emoji.into(),
                name: name.into(),
                value: fmt_eq(eq).into(),
                source: source.into(),
                whole: format!(
                    "{} pieza{} necesaria{} para alcanzar la altura",
                    c, s(c), s(c)
                )
                .into(),
                has_whole: has_whole,
            }
        })
        .collect();
    slint::ModelRc::new(slint::VecModel::from(items))
}

fn copy_text(height: f64, show_source: bool, show_whole: bool) -> String {
    let s = |n: u64| if n != 1 { "s" } else { "" };
    UNITS
        .iter()
        .map(|&(_, name, size, is_small, source)| {
            let eq = height / size;
            let mut line = format!("* {} {}", fmt_eq(eq), name);
            if show_source {
                line.push_str(&format!(" ({})", source));
            }
            if is_small && show_whole {
                let c = eq.ceil() as u64;
                line.push_str(&format!(
                    " -> {} pieza{} necesaria{} para alcanzar la altura",
                    c, s(c), s(c)
                ));
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    let app_weak = app.as_weak();
    app.on_do_calculate(move || {
        let app = app_weak.unwrap();
        let raw = app.get_height_text().to_string();
        let normalized = raw.trim().replace(',', ".");
        if normalized.is_empty() {
            app.set_error_text("Escribi tu altura en metros.".into());
            app.set_has_results(false);
            return;
        }
        let height: f64 = match normalized.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                app.set_error_text("Altura invalida. Usa un numero positivo.".into());
                app.set_has_results(false);
                return;
            }
        };

        app.set_error_text("".into());
        app.set_height_cm(format!("{} cm", (height * 100.0) as i32).into());
        app.set_units(units_for_height(height));
        app.set_has_results(true);
    });

    let app_weak = app.as_weak();
    app.on_copy_results(move || {
        let app = app_weak.unwrap();
        let raw = app.get_height_text().to_string();
        let normalized = raw.trim().replace(',', ".");
        let height: f64 = match normalized.parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => return,
        };
        let text = copy_text(height, app.get_show_source(), app.get_show_whole());
        // Print to stdout as clipboard fallback
        // On Linux: pipe to xclip/xsel/wl-copy
        // The text is also printed so the user can redirect
        println!("{}", text);
        app.set_copy_feedback("Copiado (ver terminal)".into());
    });

    app.run()
}
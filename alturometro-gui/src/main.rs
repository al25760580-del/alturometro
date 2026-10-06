slint::include_modules!();

const UNITS: &[(&str, &str, f64, bool, &str)] = &[
    ("\u{1F34C}", "Platanos", 0.18, true, "USDA Cavendish 17-20 cm"),
    ("\u{1F34E}", "Manzanas", 0.075, true, "Wikipedia 7.0-8.3 cm diam"),
    ("\u{1F354}", "Big Macs", 0.09, true, "WhatsNeue 2018 6.9 cm tall"),
    ("\u{1F986}", "Rubber Ducks", 0.10, true, "Commercial 10 cm (4 in)"),
    ("\u{1F988}", "Great Whites", 4.5, false, "CSULB Shark Lab 4.3-4.5 m"),
    ("\u{1F992}", "Giraffes", 5.0, false, "Guinness/San Diego Zoo 4.6-5.5 m"),
    ("\u{1F68C}", "New Routemaster", 11.1, false, "TfL London bus 11.1 m"),
    ("\u{1F40B}", "Blue Whales", 24.0, false, "Monterey Bay 24-25 m avg"),
    ("\u{26BD}", "Football Fields", 105.0, false, "FIFA regulation 105 m"),
    ("\u{1F5FC}", "Eiffel Towers", 330.0, false, "Official 330 m with antenna"),
];

fn fmt(v: f64) -> String {
    if v >= 0.01 {
        format!("{:.2}", v)
    } else if v >= 0.0001 {
        format!("{:.4}", v)
    } else {
        format!("{:.2e}", v)
    }
}

fn units_for_height(h: f64) -> slint::ModelRc<UnitItem> {
    let items: Vec<UnitItem> = UNITS
        .iter()
        .map(|&(emoji, name, size, has_whole, source)| {
            let eq = h / size;
            let c = eq.ceil() as u64;
            let s = |n: u64| if n != 1 { "s" } else { "" };
            UnitItem {
                emoji: emoji.into(),
                name: name.into(),
                value: fmt(eq).into(),
                source: source.into(),
                whole: format!("{} piece{} to reach height", c, s(c)).into(),
                has_whole,
            }
        })
        .collect();
    slint::ModelRc::new(slint::VecModel::from(items))
}

fn copy_text(h: f64, show_src: bool, show_whole: bool) -> String {
    let s = |n: u64| if n != 1 { "s" } else { "" };
    UNITS
        .iter()
        .map(|&(_, name, size, is_small, source)| {
            let eq = h / size;
            let mut line = format!("  {} {}", fmt(eq), name);
            if show_src {
                line.push_str(&format!("  ({})", source));
            }
            if is_small && show_whole {
                let c = eq.ceil() as u64;
                line.push_str(&format!("  -> {} piece{} to reach height", c, s(c)));
            }
            line
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    let w = app.as_weak();
    app.on_do_calculate(move || {
        let app = w.unwrap();
        let raw = app.get_height_text().to_string().replace(',', ".");
        if raw.trim().is_empty() {
            app.set_error_text("Enter your height in meters.".into());
            app.set_has_results(false);
            return;
        }
        let h: f64 = match raw.trim().parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => {
                app.set_error_text("Invalid. Use a positive number.".into());
                app.set_has_results(false);
                return;
            }
        };
        app.set_error_text("".into());
        app.set_height_cm(format!("{} cm", (h * 100.0) as i32).into());
        app.set_units(units_for_height(h));
        app.set_has_results(true);
    });

    let w = app.as_weak();
    app.on_copy_results(move || {
        let app = w.unwrap();
        let raw = app.get_height_text().to_string().replace(',', ".");
        let h: f64 = match raw.trim().parse::<f64>() {
            Ok(v) if v.is_finite() && v > 0.0 => v,
            _ => return,
        };
        let txt = copy_text(h, app.get_show_source(), app.get_show_whole());
        println!("{}", txt);
    });

    app.run()
}
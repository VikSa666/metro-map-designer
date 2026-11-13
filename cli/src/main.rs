use std::{
    env, fs,
    io::{self, Write},
};

use metro_map_designer_core::{MetroMap, SvgPoint};

fn main() -> io::Result<()> {
    let mut args = env::args_os().skip(1);
    let mut metro_map = match args.next() {
        None => MetroMap::new_empty("asdf"),
        Some(arg) if arg == "--json" => {
            let path = args.next().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Usage: metro-map-designer-cli [--json <path>]",
                )
            })?;
            if args.next().is_some() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Usage: metro-map-designer-cli [--json <path>]",
                ));
            }

            let json = fs::read_to_string(path)?;
            metro_map_designer_core::deserialize(&json)
                .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?
        }
        Some(arg) => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unknown argument: {}", arg.to_string_lossy()),
            ));
        }
    };

    let stdin = io::stdin();
    let mut line = String::new();
    println!("Welcome to the Metro Map Creator CLI!");

    loop {
        print!("🚇 ");
        io::stdout().flush()?;

        line.clear();
        if stdin.read_line(&mut line)? == 0 {
            break;
        }

        let mut parts = line.split_whitespace();
        match parts.next() {
            Some("add-station") => match (parts.next(), parts.next(), parts.next()) {
                (Some(name), Some(x), Some(y)) => add_station(&mut metro_map, name, x, y)?,
                _ => println!(
                    "Usage: add-station <station-name> <svg-x-coordinate> <svg-y-coordinate>"
                ),
            },
            Some("add-track") => match (parts.next(), parts.next(), parts.next()) {
                (Some(id), Some(from), Some(to)) => add_track(&mut metro_map, id, from, to)?,
                _ => println!("Usage: add-track <track-id> <station-from-id> <station-to-id>"),
            },
            Some("add-line") => match (parts.next(), parts.next(), parts.next()) {
                (Some(id), Some(color), Some(tracks)) => {
                    let tracks = tracks.split(',').collect::<Vec<_>>();
                    add_single_route_line(&mut metro_map, id, color, &tracks)?;
                }
                _ => println!(
                    "Usage: add-line <line-id> <line-color-rgb> <track-id>,<track-id>[,<track-id>...]"
                ),
            },
            Some("see-map") => match parts.next() {
                Some(options) => match options {
                    "--all" => see_map(&metro_map),
                    "--lines" => {
                        let name_only = parts.next().unwrap_or_default();
                        see_lines(&metro_map, name_only == "--name-only")
                    }
                    "--stations" => {
                        let name_only = parts.next().unwrap_or_default();
                        see_stations(&metro_map, name_only == "--name-only")
                    }
                    "--help" => println!("Usage: see-map [view-options]"),
                    "" => see_map(&metro_map),
                    other => println!(
                        "Error: `{other}` is not an option.\nOptions are:\n\t--all\t\tTo see all the maps"
                    ),
                },
                None => see_map(&metro_map),
            },
            Some("see-line") => match parts.next() {
                Some(id) => see_line(&metro_map, id),
                None => println!("Usage: see-line <line-id>"),
            },
            Some("see-track") => match parts.next() {
                Some(id) => see_track(&metro_map, id),
                None => println!("Usage: see-track <track-id>"),
            },
            Some("export") => match parts.next() {
                Some("--format") => match parts.next() {
                    Some("json") => {
                        let output_path = match parts.next() {
                            Some(path) => path.into(),
                            None => {
                                let project_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                                    .parent()
                                    .unwrap_or_else(|| {
                                        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                                    });
                                project_root.join("out/json/metro-map.json")
                            }
                        };
                        if let Some(parent) = output_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        fs::write(&output_path, serialize(&metro_map)?)?;
                        println!("Exported metro map to {}", output_path.display());
                    }
                    Some(other) => println!("Error: `{other}` is not a supported format."),
                    None => println!("Usage: export --format <json> [out-path]"),
                },
                Some(other) => println!(
                    "Error: `{other}` is not an option. Options are\n\t--format\n\tTo choose the export format"
                ),
                None => println!("Usage: export --format <json> [out-path]"),
            },
            Some("exit") => break,
            None => {}
            Some(command) => println!("🚈 Unknown command: {command}"),
        }
    }

    Ok(())
}

fn add_station(metro_map: &mut MetroMap, name: &str, svg_x: &str, svg_y: &str) -> io::Result<()> {
    let x = str::parse::<f32>(svg_x).map_err(|err| io::Error::other(err))?;
    let y = str::parse::<f32>(svg_y).map_err(|err| io::Error::other(err))?;
    metro_map.create_station(name, name, SvgPoint::from((x, y)), None)?;
    println!("Successfully created station '{name}'.");
    Ok(())
}

fn add_track(metro_map: &mut MetroMap, id: &str, from: &str, to: &str) -> io::Result<()> {
    metro_map.create_track(id, from, to, vec![])?;
    println!("Successfully created track with id '{id}' from station '{from}' to station '{to}'");
    Ok(())
}

fn add_single_route_line(
    metro_map: &mut MetroMap,
    id: &str,
    color: &str,
    tracks: &[&str],
) -> io::Result<()> {
    let route = metro_map.create_route(id, None, tracks)?;
    metro_map.create_line(id, id, color, vec![route])?;
    println!("Successfully created line '{id}' with only one route with the given tracks");
    Ok(())
}

fn see_map(metro_map: &MetroMap) {
    println!("#### Metro Map ####");
    println!("{metro_map:#?}")
}

fn see_lines(metro_map: &MetroMap, name_only: bool) {
    println!("#### Metro Lines ####");
    if name_only {
        let lines = metro_map
            .lines()
            .iter()
            .map(|l| l.id().to_string())
            .collect::<Vec<String>>();
        println!("{lines:#?}")
    } else {
        let lines = metro_map.lines();
        println!("{lines:#?}")
    }
}

fn see_line(metro_map: &MetroMap, id: &str) {
    println!("### Line {id} ###");
    let Some(line) = metro_map.get_line(&id.into()) else {
        println!(
            "Line with id '{id}' does not exist.\nHint: use `see-map --lines` to see the available lines"
        );
        return;
    };
    println!("{line:#?}")
}

fn see_track(metro_map: &MetroMap, id: &str) {
    println!("### Line {id} ###");
    let Some(track) = metro_map.get_track(&id.into()) else {
        println!(
            "Line with id '{id}' does not exist.\nHint: use `see-map --tracks` to see the available tracks"
        );
        return;
    };
    println!("{track:#?}")
}

fn see_stations(metro_map: &MetroMap, name_only: bool) {
    println!("#### Metro Stations ####");
    if name_only {
        let stations = metro_map
            .stations()
            .iter()
            .map(|l| l.id().to_string())
            .collect::<Vec<String>>();
        println!("{stations:#?}")
    } else {
        let stations = metro_map.stations();
        println!("{stations:#?}")
    }
}

fn serialize(metro_map: &MetroMap) -> io::Result<String> {
    Ok(metro_map_designer_core::serialize(metro_map)?)
}

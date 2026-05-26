use std::path::PathBuf;

use adbimport::genealogy::{ArchivedEventKind, EventId, GenealogyStore, PersonId};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "query_rkyv",
    about = "Query an adbimport .rkyv genealogy archive"
)]
struct Args {
    /// Path to a `.rkyv` file produced by `adbimport --format rkyv ...`.
    archive: PathBuf,

    /// Person ID (u64) to lookup.
    #[arg(long)]
    person_id: Option<u64>,

    /// Event ID (u64) to lookup.
    #[arg(long)]
    event_id: Option<u64>,

    /// Name query (tokenized, AND semantics).
    #[arg(long)]
    name: Option<String>,

    /// Year to query events for.
    #[arg(long)]
    year: Option<i32>,

    /// Max records to print.
    #[arg(long, default_value_t = 10)]
    limit: usize,
}

#[derive(Debug, Clone)]
struct PersonRow {
    id: String,
    name: String,
    birth_date: String,
    birth_time: String,
    sun: String,
    moon: String,
    asc: String,
}

fn display_len(s: &str) -> usize {
    s.chars().count()
}

fn format_positions(sign: Option<&str>, degmin: Option<&str>) -> String {
    match (sign, degmin) {
        (None, None) => String::new(),
        (Some(sign), None) => sign.to_string(),
        (None, Some(degmin)) => degmin.to_string(),
        (Some(sign), Some(degmin)) => format!("{sign} {degmin}"),
    }
}

fn find_birth_event<'a>(
    store: &'a GenealogyStore,
    person: &rkyv::Archived<adbimport::genealogy::Person>,
) -> Result<Option<&'a rkyv::Archived<adbimport::genealogy::Event>>, Box<dyn std::error::Error>> {
    for event_id in person.events.iter() {
        let event = store.event(EventId(event_id.0.into()))?;
        let Some(event) = event else {
            continue;
        };

        if matches!(event.kind, ArchivedEventKind::Birth) {
            return Ok(Some(event));
        }
    }

    Ok(None)
}

fn person_row(
    store: &GenealogyStore,
    person: &rkyv::Archived<adbimport::genealogy::Person>,
) -> Result<PersonRow, Box<dyn std::error::Error>> {
    let id = person.id.0.to_string();
    let name = person
        .names
        .first()
        .map(|n| n.display.as_str())
        .unwrap_or("<no name>")
        .to_string();

    let (birth_date, birth_time, sun, moon, asc) = match find_birth_event(store, person)? {
        None => (
            String::new(),
            String::new(),
            String::new(),
            String::new(),
            String::new(),
        ),
        Some(event) => {
            let date = event
                .date
                .as_ref()
                .map(|d| d.original.as_str())
                .unwrap_or("")
                .to_string();

            let time = match (event.time.as_deref(), event.time_zone.as_deref()) {
                (None, None) => String::new(),
                (Some(t), None) => t.to_string(),
                (None, Some(tz)) => tz.to_string(),
                (Some(t), Some(tz)) => format!("{t} {tz}"),
            };

            let (sun, moon, asc) = match event.positions.as_ref() {
                None => (String::new(), String::new(), String::new()),
                Some(pos) => (
                    format_positions(pos.sun_sign.as_deref(), pos.sun_degmin.as_deref()),
                    format_positions(pos.moon_sign.as_deref(), pos.moon_degmin.as_deref()),
                    format_positions(pos.asc_sign.as_deref(), pos.asc_degmin.as_deref()),
                ),
            };

            (date, time, sun, moon, asc)
        }
    };

    Ok(PersonRow {
        id,
        name,
        birth_date,
        birth_time,
        sun,
        moon,
        asc,
    })
}

fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let mut widths: Vec<usize> = headers.iter().map(|h| display_len(h)).collect();

    for row in rows {
        for (i, cell) in row.iter().enumerate() {
            if i >= widths.len() {
                widths.push(display_len(cell));
            } else {
                widths[i] = widths[i].max(display_len(cell));
            }
        }
    }

    fn print_divider(widths: &[usize]) {
        print!("+");
        for w in widths {
            print!("{}+", "-".repeat(*w + 2));
        }
        println!();
    }

    print_divider(&widths);
    print!("|");
    for (h, w) in headers.iter().zip(widths.iter()) {
        print!(" {:<width$} |", *h, width = *w);
    }
    println!();
    print_divider(&widths);

    for row in rows {
        print!("|");
        for (cell, w) in row.iter().zip(widths.iter()) {
            print!(" {:<width$} |", cell, width = *w);
        }
        println!();
    }

    print_divider(&widths);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let store = GenealogyStore::from_file(&args.archive)?;

    if let Some(id) = args.person_id {
        let person = store.person(PersonId(id))?;
        println!("person lookup:");
        match person {
            None => {
                println!("  person_id={id} not found");
            }
            Some(p) => {
                let row = person_row(&store, p)?;
                print_table(
                    &[
                        "ID",
                        "Name",
                        "Birth date",
                        "Birth time",
                        "Sun",
                        "Moon",
                        "Asc",
                    ],
                    &[vec![
                        row.id,
                        row.name,
                        row.birth_date,
                        row.birth_time,
                        row.sun,
                        row.moon,
                        row.asc,
                    ]],
                );

                println!("events: {}", p.events.len());
                println!("families_as_spouse: {}", p.families_as_spouse.len());
                println!("families_as_child: {}", p.families_as_child.len());
                println!("notes: {}", p.notes.len());
            }
        }
    }

    if let Some(id) = args.event_id {
        let event = store.event(EventId(id))?;
        println!("event lookup:");
        match event {
            None => println!("  event_id={id} not found"),
            Some(e) => {
                let kind = format!("{:?}", e.kind);
                let date = e
                    .date
                    .as_ref()
                    .map(|d| d.original.as_str())
                    .unwrap_or("<no date>");
                let time = e.time.as_deref().unwrap_or("<no time>");
                let tz = e.time_zone.as_deref().unwrap_or("");

                let (sun, moon, asc) = match e.positions.as_ref() {
                    None => (String::new(), String::new(), String::new()),
                    Some(pos) => (
                        format_positions(pos.sun_sign.as_deref(), pos.sun_degmin.as_deref()),
                        format_positions(pos.moon_sign.as_deref(), pos.moon_degmin.as_deref()),
                        format_positions(pos.asc_sign.as_deref(), pos.asc_degmin.as_deref()),
                    ),
                };

                print_table(
                    &["ID", "Kind", "Date", "Time", "TZ", "Sun", "Moon", "Asc"],
                    &[vec![
                        id.to_string(),
                        kind,
                        date.to_string(),
                        time.to_string(),
                        tz.to_string(),
                        sun,
                        moon,
                        asc,
                    ]],
                );

                println!("participants: {}", e.participants.len());
            }
        }
    }

    if let Some(query) = args.name.as_deref() {
        let hits = store.search_people_by_name(query, args.limit)?;
        println!("name_query={query:?} hits={}", hits.len());

        let mut rows = Vec::with_capacity(hits.len());
        for p in hits {
            let row = person_row(&store, p)?;
            rows.push(vec![
                row.id,
                row.name,
                row.birth_date,
                row.birth_time,
                row.sun,
                row.moon,
                row.asc,
            ]);
        }

        if rows.is_empty() {
            println!("(no matches)");
        } else {
            print_table(
                &[
                    "ID",
                    "Name",
                    "Birth date",
                    "Birth time",
                    "Sun",
                    "Moon",
                    "Asc",
                ],
                &rows,
            );
        }
    }

    if let Some(year) = args.year {
        let events = store.events_by_year(year, args.limit)?;
        println!("events_by_year={year} hits={}", events.len());
        for (i, e) in events.iter().enumerate() {
            let id = e.id.0;
            let kind = match e.kind {
                ArchivedEventKind::Birth => "Birth".to_string(),
                ArchivedEventKind::Death => "Death".to_string(),
                ArchivedEventKind::Marriage => "Marriage".to_string(),
                ArchivedEventKind::Residence => "Residence".to_string(),
                ArchivedEventKind::Occupation => "Occupation".to_string(),
                ArchivedEventKind::Other(ref s) => s.to_string(),
            };
            let date = e
                .date
                .as_ref()
                .map(|d| d.original.as_str())
                .unwrap_or("<no date>");
            println!("  {i}: id={id} kind={kind} date={date}");
        }
    }

    if args.person_id.is_none()
        && args.event_id.is_none()
        && args.name.is_none()
        && args.year.is_none()
    {
        // If no query flags are provided, print a tiny summary.
        store.with_archived(|arch| {
            println!("archive summary:");
            println!("  people: {}", arch.people.len());
            println!("  events: {}", arch.events.len());
            println!("  families: {}", arch.families.len());
            println!("  places: {}", arch.places.len());
            println!("  notes: {}", arch.notes.len());
        })?;
    }

    Ok(())
}

use std::collections::HashMap;

use crate::{
    AstrodatabankExport, BirthData, BirthDataAlt, Country, Event as XmlEvent, Place as XmlPlace,
    parse_astrodatabank_export_file,
};

use kleio::*;

/// A lossless-to-the-extent-possible internal representation for the values
fn extract_number_tokens(s: &str) -> Vec<String> {
    // Extract contiguous digit/decimal tokens, splitting on anything else.
    // This allows parsing inputs like:
    // - "45n10" -> ["45", "10"]
    // - "45n10'" -> ["45", "10"]
    // - "45 n 10" -> ["45", "10"]
    // - "45n10:30" -> ["45", "10", "30"]
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();

    for ch in s.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            cur.push(ch);
        } else {
            if !cur.is_empty() {
                out.push(core::mem::take(&mut cur));
            }
        }
    }

    if !cur.is_empty() {
        out.push(cur);
    }

    out
}

fn parse_adb_coord(coord: &str, kind: CoordKind) -> Option<f64> {
    // ADB coordinates are often encoded like:
    // - "47n36" (47°36' N)
    // - "122w20" (122°20' W)
    // Sometimes only degrees are present (e.g. "40n").
    //
    // Real-world exports also sometimes include extra punctuation/spaces.
    // This is a best-effort parser; if it fails we treat coordinates as missing.
    let raw = coord.trim().to_ascii_lowercase();
    if raw.is_empty() {
        return None;
    }

    // Find the direction character.
    let dir_idx = raw.rfind(|c: char| matches!(c, 'n' | 's' | 'e' | 'w'))?;
    let dir = raw[dir_idx..].chars().next()?;

    // Replace the direction with a separator so numeric tokens become [deg, min, sec...].
    let mut normalized = raw.clone();
    normalized.replace_range(dir_idx..dir_idx + dir.len_utf8(), " ");

    let tokens = extract_number_tokens(&normalized);
    if tokens.is_empty() {
        return None;
    }

    let deg: f64 = tokens[0].parse().ok()?;

    // Minutes/seconds are optional.
    let (min, sec) = match tokens.as_slice() {
        [] => return None,
        [_deg] => (0.0, 0.0),
        [_deg, mm] => {
            // Heuristic: sometimes minutes+seconds are concatenated as mmss.
            // Only apply this if the token is exactly 4 digits.
            if mm.len() == 4 && mm.chars().all(|c| c.is_ascii_digit()) {
                let (m, s) = mm.split_at(2);
                (m.parse::<f64>().ok()?, s.parse::<f64>().ok()?)
            } else {
                (mm.parse::<f64>().ok()?, 0.0)
            }
        }
        [_deg, mm, ss, ..] => (mm.parse::<f64>().ok()?, ss.parse::<f64>().ok()?),
    };

    if !(0.0..60.0).contains(&min) || !(0.0..60.0).contains(&sec) {
        return None;
    }

    let mut value = deg + (min / 60.0) + (sec / 3600.0);

    // Validate direction.
    let is_neg = match (kind, dir) {
        (CoordKind::Lat, 'n') => false,
        (CoordKind::Lat, 's') => true,
        (CoordKind::Lon, 'e') => false,
        (CoordKind::Lon, 'w') => true,
        _ => return None,
    };

    if is_neg {
        value = -value;
    }

    Some(value)
}

#[derive(Clone, Copy, Debug)]
enum CoordKind {
    Lat,
    Lon,
}

// Source-specific values retained during import.
fn attr(key: &str, value: Option<&String>) -> Option<Attribute> {
    let value = value?.trim();
    if value.is_empty() {
        None
    } else {
        Some(Attribute {
            key: key.to_string(),
            value: value.to_string(),
        })
    }
}

fn extend_position_attributes(provenance: &mut Provenance, positions: Option<&crate::Positions>) {
    let Some(positions) = positions else {
        return;
    };

    provenance.attributes.extend(
        [
            attr("positions.sun.sign", positions.sun_sign.as_ref()),
            attr("positions.sun.degmin", positions.sun_degmin.as_ref()),
            attr("positions.moon.sign", positions.moon_sign.as_ref()),
            attr("positions.moon.degmin", positions.moon_degmin.as_ref()),
            attr("positions.asc.sign", positions.asc_sign.as_ref()),
            attr("positions.asc.degmin", positions.asc_degmin.as_ref()),
        ]
        .into_iter()
        .flatten(),
    );
}

fn parse_place_lat_lon(place: Option<&XmlPlace>) -> Option<(f64, f64)> {
    let place = place?;
    let lat_s = place.slati.as_deref()?;
    let lon_s = place.slong.as_deref()?;

    let lat = parse_adb_coord(lat_s, CoordKind::Lat)?;
    let lon = parse_adb_coord(lon_s, CoordKind::Lon)?;

    Some((lat, lon))
}

/// currently parsed from Astrodatabank XML.
///
/// This is intentionally "Rust-native" rather than mirroring the XML schema.
#[derive(Debug, Clone)]
pub struct AstrodatabankStructured {
    pub people: Vec<Person>,
    pub events: Vec<Event>,
    pub families: Vec<Family>,
    pub places: Vec<Place>,
    pub notes: Vec<Note>,

    /// Auxiliary mapping from Astrodatabank adb_id -> internal person id.
    pub person_id_by_adb_id: HashMap<u64, PersonId>,
}

/// Parse an Astrodatabank export file into the structured internal model.
pub fn parse_astrodatabank_export_file_structured(
    path: impl AsRef<std::path::Path>,
) -> Result<(AstrodatabankExport, AstrodatabankStructured), std::io::Error> {
    let export = parse_astrodatabank_export_file(path)?;
    let structured = parse_astrodatabank_export_structured(&export);
    Ok((export, structured))
}

/// Convert an already-parsed `AstrodatabankExport` into the structured internal model.
pub fn parse_astrodatabank_export_structured(
    export: &AstrodatabankExport,
) -> AstrodatabankStructured {
    let mut idgen = IdGen::default();

    let mut people: Vec<Person> = Vec::with_capacity(export.entries.len());
    let mut events: Vec<Event> = Vec::new();
    let mut families: Vec<Family> = Vec::new();
    let mut places: Vec<Place> = Vec::new();
    let mut notes: Vec<Note> = Vec::new();

    let mut person_id_by_adb_id: HashMap<u64, PersonId> = HashMap::new();
    let mut place_id_by_key: HashMap<String, PlaceId> = HashMap::new();

    // First pass: people + their intrinsic facts (birth, bio note, per-person events).
    for entry in &export.entries {
        let person_id = PersonId(entry.adb_id);
        person_id_by_adb_id.insert(entry.adb_id, person_id);

        let mut names = vec![Name {
            display: entry.public_data.name.clone(),
            given: None,
            surname: None,
            aliases: Vec::new(),
            provenance: Default::default(),
        }];

        if let Some(sflname) = entry.public_data.sflname.as_deref()
            && !sflname.trim().is_empty()
        {
            names[0].aliases.push(sflname.trim().to_string());
        }
        if let Some(birthname) = entry.public_data.birthname.as_deref()
            && !birthname.trim().is_empty()
        {
            names[0].aliases.push(birthname.trim().to_string());
        }

        let sex = entry
            .public_data
            .gender
            .as_ref()
            .map(|g| g.value.trim().to_ascii_uppercase())
            .and_then(|s| match s.as_str() {
                "M" | "MALE" => Some(Sex::Male),
                "F" | "FEMALE" => Some(Sex::Female),
                "X" | "NONBINARY" | "NON-BINARY" => Some(Sex::Other),
                "U" | "UNKNOWN" | "" => Some(Sex::Unknown),
                _ => None,
            });

        let mut person_event_ids: Vec<EventId> = Vec::new();

        // Primary birth data becomes a Birth event.
        if let Some(bdata) = entry.public_data.bdata.as_ref() {
            let (event, place_id) =
                map_birth_data(&mut idgen, &mut places, &mut place_id_by_key, bdata);
            let mut event = event;
            event.participants.push(person_id);
            if event.place.is_none() {
                event.place = place_id;
            }

            person_event_ids.push(event.id);
            events.push(event);
        }

        // Alternative birth blocks: map into Other events so no data is lost.
        for alt in &entry.public_data.bdata_alt {
            if let Some(ev) = map_birth_data_alt(
                &mut idgen,
                &mut places,
                &mut place_id_by_key,
                alt,
                person_id,
            ) {
                person_event_ids.push(ev.id);
                events.push(ev);
            }
        }

        // Text data: short biography becomes a note.
        let mut note_ids: Vec<NoteId> = Vec::new();
        if let Some(text) = entry.text_data.as_ref() {
            if let Some(bio) = text.shortbiography.as_ref().and_then(|t| t.value.as_ref()) {
                let bio = bio.trim();
                if !bio.is_empty() {
                    let nid = NoteId(idgen.next_note_id());
                    notes.push(Note {
                        id: nid,
                        text: bio.to_string(),
                        copyright: text
                            .shortbiography
                            .as_ref()
                            .and_then(|t| t.copyright.clone()),
                        provenance: Default::default(),
                    });
                    note_ids.push(nid);
                }
            }

            // Sourcenotes can be large: preserve as a note as well.
            if let Some(sn) = text.sourcenotes.as_ref().and_then(|t| t.value.as_ref()) {
                let sn = sn.trim();
                if !sn.is_empty() {
                    let nid = NoteId(idgen.next_note_id());
                    notes.push(Note {
                        id: nid,
                        text: sn.to_string(),
                        copyright: text.sourcenotes.as_ref().and_then(|t| t.copyright.clone()),
                        provenance: Default::default(),
                    });
                    note_ids.push(nid);
                }
            }
        }

        // Research events: per-person events (death etc.)
        if let Some(r) = entry.research_data.as_ref()
            && let Some(evs) = r.events.as_ref()
        {
            for ev in &evs.items {
                if let Some(mapped) = map_research_event(&mut idgen, ev, person_id) {
                    person_event_ids.push(mapped.id);
                    events.push(mapped);
                }
            }
        }

        people.push(Person {
            id: person_id,
            names,
            sex,
            events: person_event_ids,
            families_as_child: Vec::new(),
            families_as_spouse: Vec::new(),
            notes: note_ids,
            source_record: Some(kleio::attribution::SourceRef(format!(
                "adb:{}",
                entry.adb_id
            ))),
            provenance: Default::default(),
        });
    }

    // Second pass: relationships -> families.
    // ADB records include both directions (parent->child and child->parent) in samples.
    // We build a canonical family record per (parent(s), child) occurrence.
    // This is a minimal mapping; it will improve as we better understand relationship categories.
    let mut family_key_to_id: HashMap<(PersonId, PersonId), FamilyId> = HashMap::new();

    for entry in &export.entries {
        let Some(r) = entry.research_data.as_ref() else {
            continue;
        };
        let Some(rels) = r.relationships.as_ref() else {
            continue;
        };

        let this_person_id = PersonId(entry.adb_id);

        for rel in &rels.items {
            let text = rel.value.as_deref().unwrap_or("");
            if rel.relcat.as_deref() != Some("Family") {
                continue;
            }

            if text.contains("parent->child") {
                if let Some(other) = rel
                    .rel_adb_id
                    .and_then(|id| person_id_by_adb_id.get(&id).copied())
                {
                    // this_person is parent, other is child
                    let fam_id = *family_key_to_id
                        .entry((this_person_id, other))
                        .or_insert_with(|| {
                            let id = FamilyId(idgen.next_family_id());
                            families.push(Family {
                                id,
                                spouses: vec![this_person_id],
                                children: vec![other],
                                events: Vec::new(),
                                provenance: Default::default(),
                            });
                            id
                        });

                    attach_family(&mut people, fam_id, this_person_id, other);
                }
            } else if text.contains("child->parent")
                && let Some(other) = rel
                    .rel_adb_id
                    .and_then(|id| person_id_by_adb_id.get(&id).copied())
            {
                // this_person is child, other is parent
                let fam_id = *family_key_to_id
                    .entry((other, this_person_id))
                    .or_insert_with(|| {
                        let id = FamilyId(idgen.next_family_id());
                        families.push(Family {
                            id,
                            spouses: vec![other],
                            children: vec![this_person_id],
                            events: Vec::new(),
                            provenance: Default::default(),
                        });
                        id
                    });

                attach_family(&mut people, fam_id, other, this_person_id);
            }
        }
    }

    AstrodatabankStructured {
        people,
        events,
        families,
        places,
        notes,
        person_id_by_adb_id,
    }
}

#[derive(Debug, Default)]
struct IdGen {
    next_place_id: u64,
    next_event_id: u64,
    next_family_id: u64,
    next_note_id: u64,
}

impl IdGen {
    fn next_place_id(&mut self) -> u64 {
        self.next_place_id += 1;
        self.next_place_id
    }

    fn next_event_id(&mut self) -> u64 {
        self.next_event_id += 1;
        self.next_event_id
    }

    fn next_family_id(&mut self) -> u64 {
        self.next_family_id += 1;
        self.next_family_id
    }

    fn next_note_id(&mut self) -> u64 {
        self.next_note_id += 1;
        self.next_note_id
    }
}

fn place_key(place: Option<&XmlPlace>, country: Option<&Country>) -> Option<String> {
    let p = place?.value.trim();
    let c = country.map(|c| c.value.trim()).unwrap_or("");
    let key = match (p.is_empty(), c.is_empty()) {
        (true, true) => return None,
        (false, true) => p.to_string(),
        (true, false) => c.to_string(),
        (false, false) => format!("{p}, {c}"),
    };
    Some(key)
}

fn get_or_create_place(
    idgen: &mut IdGen,
    places: &mut Vec<Place>,
    place_id_by_key: &mut HashMap<String, PlaceId>,
    name: String,
    lat_lon: Option<(f64, f64)>,
) -> PlaceId {
    if let Some(id) = place_id_by_key.get(&name).copied() {
        // If we previously created this place without coordinates, but we now
        // have coordinates, fill them in.
        if let Some(lat_lon) = lat_lon
            && let Some(existing) = places.iter_mut().find(|p| p.id == id)
            && existing.lat_lon.is_none()
        {
            existing.lat_lon = Some(lat_lon);
        }

        return id;
    }

    let id = PlaceId(idgen.next_place_id());
    places.push(Place {
        id,
        name: name.clone(),
        lat_lon,
        geosuggest_id: None,
        provenance: Default::default(),
    });
    place_id_by_key.insert(name, id);
    id
}

fn map_birth_data(
    idgen: &mut IdGen,
    places: &mut Vec<Place>,
    place_id_by_key: &mut HashMap<String, PlaceId>,
    bdata: &BirthData,
) -> (Event, Option<PlaceId>) {
    let date = bdata
        .sbdate
        .as_ref()
        .and_then(|d| d.value.as_ref())
        .map(|s| DateValue::from_original(s.clone(), Default::default()));

    let place_name = place_key(bdata.place.as_ref(), bdata.country.as_ref());
    let place_id = place_name.clone().map(|name| {
        let lat_lon = parse_place_lat_lon(bdata.place.as_ref());
        get_or_create_place(idgen, places, place_id_by_key, name, lat_lon)
    });

    let time = bdata
        .sbtime
        .as_ref()
        .and_then(|t| t.value.as_ref())
        .cloned();

    let time_zone = bdata
        .sbtime
        .as_ref()
        .and_then(|t| t.sznabbr.as_ref())
        .cloned();

    let mut provenance = Provenance::default();
    extend_position_attributes(&mut provenance, bdata.positions.as_ref());

    (
        Event {
            id: EventId(idgen.next_event_id()),
            kind: EventKind::Birth,
            date,
            time,
            time_zone,
            place: place_id,
            description: None,
            participants: Vec::new(),
            provenance,
        },
        place_id,
    )
}

fn map_birth_data_alt(
    idgen: &mut IdGen,
    places: &mut Vec<Place>,
    place_id_by_key: &mut HashMap<String, PlaceId>,
    alt: &BirthDataAlt,
    person_id: PersonId,
) -> Option<Event> {
    let kind = alt
        .event_type
        .value
        .as_deref()
        .map(|v| EventKind::Other(v.to_string()))
        .unwrap_or_else(|| EventKind::Other("bdata_alt".to_string()));

    let date = alt
        .sbdate
        .as_ref()
        .and_then(|d| d.value.as_ref())
        .map(|s| DateValue::from_original(s.clone(), Default::default()));

    let place_name = place_key(alt.place.as_ref(), alt.country.as_ref());
    let place_id = place_name.clone().map(|name| {
        let lat_lon = parse_place_lat_lon(alt.place.as_ref());
        get_or_create_place(idgen, places, place_id_by_key, name, lat_lon)
    });

    Some(Event {
        id: EventId(idgen.next_event_id()),
        kind,
        date,
        time: None,
        time_zone: None,
        place: place_id,
        description: alt.event_notes.clone(),
        participants: vec![person_id],
        provenance: Default::default(),
    })
}

fn map_research_event(idgen: &mut IdGen, ev: &XmlEvent, person_id: PersonId) -> Option<Event> {
    let kind = ev
        .sevcode
        .as_deref()
        .map(|s| match s {
            "Death, Cause unspecified" => EventKind::Death,
            _ => EventKind::Other(s.to_string()),
        })
        .unwrap_or_else(|| EventKind::Other("event".to_string()));

    let date = ev
        .event_data
        .as_ref()
        .and_then(|d| d.sbdate.as_ref())
        .and_then(|d| d.value.as_ref())
        .map(|s| DateValue::from_original(s.clone(), Default::default()));

    Some(Event {
        id: EventId(idgen.next_event_id()),
        kind,
        date,
        time: None,
        time_zone: None,
        place: None,
        description: ev.evnotes.clone(),
        participants: vec![person_id],
        provenance: Default::default(),
    })
}

fn attach_family(people: &mut [Person], family_id: FamilyId, parent: PersonId, child: PersonId) {
    for p in people.iter_mut() {
        if p.id == parent && !p.families_as_spouse.contains(&family_id) {
            p.families_as_spouse.push(family_id);
        }
        if p.id == child && !p.families_as_child.contains(&family_id) {
            p.families_as_child.push(family_id);
        }
    }
}

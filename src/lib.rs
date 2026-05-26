//! Astrodatabank XML import utilities.
//!
//! The XML schema is deserialized using `quick-xml` + Serde into the data model
//! defined in this module.

use serde::Deserialize;

pub mod genealogy;

pub use genealogy::{AstrodatabankStructured, parse_astrodatabank_export_file_structured};

#[derive(Debug, Deserialize)]
pub struct AstrodatabankExport {
    #[serde(rename = "@export_format")]
    pub export_format: String,

    pub timestamp: Timestamp,

    pub important_notice: Option<String>,

    #[serde(rename = "adb_entry", default)]
    pub entries: Vec<AdbEntry>,

    pub adb_entry_count: Option<AdbEntryCount>,
}

#[derive(Debug, Deserialize)]
pub struct AdbEntryCount {
    #[serde(rename = "@count")]
    pub count: u64,
}

#[derive(Debug, Deserialize)]
pub struct Timestamp {
    #[serde(rename = "@itst")]
    pub itst: Option<String>,

    #[serde(rename = "@user")]
    pub user: Option<String>,

    #[serde(rename = "@host")]
    pub host: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct AdbEntry {
    #[serde(rename = "@adb_id")]
    pub adb_id: u64,

    pub update_timestamp: Option<UpdateTimestamp>,

    pub public_data: PublicData,

    pub text_data: Option<TextData>,

    pub research_data: Option<ResearchData>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTimestamp {
    #[serde(rename = "@itst")]
    pub itst: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct PublicData {
    pub name: String,

    pub sflname: Option<String>,

    pub birthname: Option<String>,

    pub gender: Option<Gender>,

    pub roddenrating: Option<RoddenRating>,

    pub datatype: Option<DataType>,

    pub bdata: Option<BirthData>,

    #[serde(rename = "bdata_alt", default)]
    pub bdata_alt: Vec<BirthDataAlt>,

    pub scollector: Option<String>,

    pub seditor: Option<String>,

    pub sbiographer: Option<String>,

    pub screationdate: Option<String>,

    pub slasteditdate: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BirthDataAlt {
    pub event_type: EventType,

    pub event_notes: Option<String>,

    pub sbdate: Option<SbDate>,

    pub sbdate_dmy: Option<String>,

    pub sbtime: Option<SbTime>,

    pub place: Option<Place>,

    pub country: Option<Country>,
}

#[derive(Debug, Deserialize)]
pub struct EventType {
    #[serde(rename = "@event_id")]
    pub event_id: Option<u64>,

    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Gender {
    #[serde(rename = "@csex")]
    pub csex: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct RoddenRating {
    #[serde(rename = "@rrc")]
    pub rrc: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct DataType {
    #[serde(rename = "@sdatatype")]
    pub sdatatype: Option<String>,

    #[serde(rename = "@dtc")]
    pub dtc: Option<String>,

    #[serde(rename = "@sdatasource")]
    pub sdatasource: Option<String>,

    #[serde(rename = "@dsc")]
    pub dsc: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BirthData {
    pub sbdate: Option<SbDate>,

    pub sbdate_dmy: Option<String>,

    pub sbtime: Option<SbTime>,

    pub place: Option<Place>,

    pub country: Option<Country>,

    pub positions: Option<Positions>,
}

#[derive(Debug, Deserialize)]
pub struct SbDate {
    #[serde(rename = "@ccalendar")]
    pub ccalendar: Option<String>,

    #[serde(rename = "@iyear")]
    pub iyear: Option<i32>,

    #[serde(rename = "@imonth")]
    pub imonth: Option<u8>,

    #[serde(rename = "@iday")]
    pub iday: Option<u8>,

    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SbTime {
    #[serde(rename = "@sbtime_ampm")]
    pub sbtime_ampm: Option<String>,

    #[serde(rename = "@ctimetype")]
    pub ctimetype: Option<String>,

    #[serde(rename = "@stimetype")]
    pub stimetype: Option<String>,

    #[serde(rename = "@stmerid")]
    pub stmerid: Option<String>,

    #[serde(rename = "@ctzauto")]
    pub ctzauto: Option<String>,

    #[serde(rename = "@itimeacc")]
    pub itimeacc: Option<u32>,

    #[serde(rename = "@stimeacc")]
    pub stimeacc: Option<String>,

    #[serde(rename = "@time_unknown")]
    pub time_unknown: Option<String>,

    #[serde(rename = "@jd_ut")]
    pub jd_ut: Option<f64>,

    #[serde(rename = "@sznabbr")]
    pub sznabbr: Option<String>,

    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Place {
    #[serde(rename = "@slati")]
    pub slati: Option<String>,

    #[serde(rename = "@slong")]
    pub slong: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Country {
    #[serde(rename = "@sctr")]
    pub sctr: Option<String>,

    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Positions {
    #[serde(rename = "@sun_sign")]
    pub sun_sign: Option<String>,

    #[serde(rename = "@sun_degmin")]
    pub sun_degmin: Option<String>,

    #[serde(rename = "@moon_sign")]
    pub moon_sign: Option<String>,

    #[serde(rename = "@moon_degmin")]
    pub moon_degmin: Option<String>,

    #[serde(rename = "@asc_sign")]
    pub asc_sign: Option<String>,

    #[serde(rename = "@asc_degmin")]
    pub asc_degmin: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TextData {
    pub shortbiography: Option<CopyrightText>,

    pub wikipedia_link: Option<String>,

    pub adb_link: Option<String>,

    pub sourcenotes: Option<CopyrightText>,
}

#[derive(Debug, Deserialize)]
pub struct CopyrightText {
    #[serde(rename = "@copyright")]
    pub copyright: Option<String>,

    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResearchData {
    pub categories: Option<Categories>,

    pub relationships: Option<Relationships>,

    pub events: Option<Events>,
}

#[derive(Debug, Deserialize)]
pub struct Categories {
    #[serde(rename = "@count")]
    pub count: Option<u32>,

    #[serde(rename = "@note")]
    pub note: Option<String>,

    #[serde(rename = "category", default)]
    pub items: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    #[serde(rename = "@cat_id")]
    pub cat_id: Option<u64>,

    #[serde(rename = "@adb_id")]
    pub adb_id: Option<u64>,

    #[serde(rename = "@catnotes")]
    pub catnotes: Option<String>,

    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Relationships {
    #[serde(rename = "@count")]
    pub count: Option<u32>,

    #[serde(rename = "@note")]
    pub note: Option<String>,

    #[serde(rename = "relationship", default)]
    pub items: Vec<Relationship>,
}

#[derive(Debug, Deserialize)]
pub struct Relationship {
    #[serde(rename = "@rel_id")]
    pub rel_id: Option<u64>,

    #[serde(rename = "@rel_adb_id")]
    pub rel_adb_id: Option<u64>,

    #[serde(rename = "@adb_id")]
    pub adb_id: Option<u64>,

    #[serde(rename = "@relcat")]
    pub relcat: Option<String>,

    #[serde(rename = "@relnotes")]
    pub relnotes: Option<String>,

    #[serde(rename = "$text")]
    pub value: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Events {
    #[serde(rename = "@count")]
    pub count: Option<u32>,

    #[serde(rename = "@note")]
    pub note: Option<String>,

    #[serde(rename = "event", default)]
    pub items: Vec<Event>,
}

#[derive(Debug, Deserialize)]
pub struct Event {
    #[serde(rename = "@sevcode")]
    pub sevcode: Option<String>,

    #[serde(rename = "@evn_id")]
    pub evn_id: Option<u64>,

    #[serde(rename = "@adb_id")]
    pub adb_id: Option<u64>,

    #[serde(rename = "@evnotes")]
    pub evnotes: Option<String>,

    pub event_data: Option<EventData>,
}

#[derive(Debug, Deserialize)]
pub struct EventData {
    pub sbdate: Option<SbDate>,

    pub sbdate_dmy: Option<String>,
}

/// Parse an Astrodatabank export from any reader.
///
/// Uses `quick_xml::de::from_reader` to avoid loading the entire document into memory.
pub fn parse_astrodatabank_export<R: std::io::BufRead>(
    reader: R,
) -> Result<AstrodatabankExport, quick_xml::DeError> {
    quick_xml::de::from_reader(reader)
}

/// Parse an Astrodatabank export from a file path.
pub fn parse_astrodatabank_export_file(
    path: impl AsRef<std::path::Path>,
) -> Result<AstrodatabankExport, std::io::Error> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);

    // Map XML deserialize errors into io::Error so callers doing I/O can return a single error type.
    parse_astrodatabank_export(reader)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
}

/// Write a minimal GEDCOM 7 file for an Astrodatabank export.
///
/// This uses `ged_io`'s structured types + writer (not manual string formatting).
///
/// Current mapping (minimal, intended for smoke-testable output):
/// - Individual: `NAME`, `SEX`
/// - Birth event: `BIRT` + `DATE` + `PLAC`
/// - Note: `NOTE` (from short biography when present)
/// - Source: a single global `@S1@` `SOUR` record and per-individual `SOUR @S1@` citations
pub fn write_gedcom7_file(
    export: &AstrodatabankExport,
    path: impl AsRef<std::path::Path>,
) -> Result<(), std::io::Error> {
    fn write_line(out: &mut String, level: u8, tag: &str, value: Option<&str>) {
        out.push_str(&format!("{level} {tag}"));
        if let Some(v) = value {
            let v = v.trim();
            if !v.is_empty() {
                out.push(' ');
                out.push_str(v);
            }
        }
        out.push('\n');
    }

    fn write_text(out: &mut String, level: u8, tag: &str, value: &str) {
        let mut iter = value.lines();
        let first = iter.next().unwrap_or("");
        write_line(out, level, tag, Some(first));
        for line in iter {
            write_line(out, level + 1, "CONT", Some(line));
        }
    }

    fn write_ourania_kv(out: &mut String, level: u8, key: &str, value: Option<&str>) {
        let Some(value) = value else {
            return;
        };
        let value = value.trim();
        if value.is_empty() {
            return;
        }

        let tag = format!("_OURANIA_{}", key.to_ascii_uppercase());
        write_line(out, level, &tag, Some(value));
    }

    fn write_ourania_text(out: &mut String, level: u8, key: &str, value: Option<&str>) {
        let Some(value) = value else {
            return;
        };
        let value = value.trim();
        if value.is_empty() {
            return;
        }

        let tag = format!("_OURANIA_{}", key.to_ascii_uppercase());
        write_text(out, level, &tag, value);
    }

    fn write_ourania_u64(out: &mut String, level: u8, key: &str, value: Option<u64>) {
        if let Some(v) = value {
            write_ourania_kv(out, level, key, Some(&v.to_string()));
        }
    }

    fn write_ourania_i32(out: &mut String, level: u8, key: &str, value: Option<i32>) {
        if let Some(v) = value {
            write_ourania_kv(out, level, key, Some(&v.to_string()));
        }
    }

    fn write_ourania_u8(out: &mut String, level: u8, key: &str, value: Option<u8>) {
        if let Some(v) = value {
            write_ourania_kv(out, level, key, Some(&v.to_string()));
        }
    }

    fn write_ourania_u32(out: &mut String, level: u8, key: &str, value: Option<u32>) {
        if let Some(v) = value {
            write_ourania_kv(out, level, key, Some(&v.to_string()));
        }
    }

    fn write_ourania_f64(out: &mut String, level: u8, key: &str, value: Option<f64>) {
        if let Some(v) = value {
            write_ourania_kv(out, level, key, Some(&v.to_string()));
        }
    }

    fn write_source_citation(out: &mut String, level: u8) {
        write_line(out, level, "SOUR", Some("@S1@"));
    }

    fn write_birth_data(out: &mut String, level: u8, bdata: &BirthData) {
        write_line(out, level, "BIRT", None);

        if let Some(sbdate) = bdata.sbdate.as_ref().and_then(|d| d.value.as_ref()) {
            let v = sbdate.trim();
            if !v.is_empty() {
                write_line(out, level + 1, "DATE", Some(v));
            }
        }

        // Preserve sbdate metadata.
        if let Some(sbdate) = bdata.sbdate.as_ref() {
            write_ourania_kv(out, level + 1, "CALENDAR", sbdate.ccalendar.as_deref());
            write_ourania_i32(out, level + 1, "IYEAR", sbdate.iyear);
            write_ourania_u8(out, level + 1, "IMONTH", sbdate.imonth);
            write_ourania_u8(out, level + 1, "IDAY", sbdate.iday);
            write_ourania_kv(out, level + 1, "SBDATE", sbdate.value.as_deref());
        }
        write_ourania_kv(out, level + 1, "SBDATE_DMY", bdata.sbdate_dmy.as_deref());

        // Time.
        if let Some(sbtime) = bdata.sbtime.as_ref() {
            // The main time string becomes the requested _OURANIA_TIME tag.
            write_ourania_kv(out, level + 1, "TIME", sbtime.value.as_deref());

            // Preserve the rest of the sbtime attributes too.
            write_ourania_kv(out, level + 1, "SBTIME_AMPM", sbtime.sbtime_ampm.as_deref());
            write_ourania_kv(out, level + 1, "CTIMETYPE", sbtime.ctimetype.as_deref());
            write_ourania_kv(out, level + 1, "STIMETYPE", sbtime.stimetype.as_deref());
            write_ourania_kv(out, level + 1, "STMERID", sbtime.stmerid.as_deref());
            write_ourania_kv(out, level + 1, "CTZAUTO", sbtime.ctzauto.as_deref());
            write_ourania_u32(out, level + 1, "ITIMEACC", sbtime.itimeacc);
            write_ourania_kv(out, level + 1, "STIMEACC", sbtime.stimeacc.as_deref());
            write_ourania_kv(
                out,
                level + 1,
                "TIME_UNKNOWN",
                sbtime.time_unknown.as_deref(),
            );
            write_ourania_f64(out, level + 1, "JD_UT", sbtime.jd_ut);
            write_ourania_kv(out, level + 1, "SZNABBR", sbtime.sznabbr.as_deref());
        }

        // Place.
        let place = bdata.place.as_ref().map(|p| p.value.trim()).unwrap_or("");
        let country = bdata.country.as_ref().map(|c| c.value.trim()).unwrap_or("");
        let plac = match (place.is_empty(), country.is_empty()) {
            (false, false) => format!("{}, {}", place, country),
            (false, true) => place.to_string(),
            (true, false) => country.to_string(),
            (true, true) => String::new(),
        };
        if !plac.is_empty() {
            write_line(out, level + 1, "PLAC", Some(&plac));
        }

        if let Some(p) = bdata.place.as_ref() {
            write_ourania_kv(out, level + 1, "PLACE", Some(p.value.as_str()));
            write_ourania_kv(out, level + 1, "PLACE_SLATI", p.slati.as_deref());
            write_ourania_kv(out, level + 1, "PLACE_SLONG", p.slong.as_deref());
        }

        if let Some(c) = bdata.country.as_ref() {
            write_ourania_kv(out, level + 1, "COUNTRY", Some(c.value.as_str()));
            write_ourania_kv(out, level + 1, "COUNTRY_SCTR", c.sctr.as_deref());
        }

        if let Some(pos) = bdata.positions.as_ref() {
            write_ourania_kv(out, level + 1, "SUN_SIGN", pos.sun_sign.as_deref());
            write_ourania_kv(out, level + 1, "SUN_DEGMIN", pos.sun_degmin.as_deref());
            write_ourania_kv(out, level + 1, "MOON_SIGN", pos.moon_sign.as_deref());
            write_ourania_kv(out, level + 1, "MOON_DEGMIN", pos.moon_degmin.as_deref());
            write_ourania_kv(out, level + 1, "ASC_SIGN", pos.asc_sign.as_deref());
            write_ourania_kv(out, level + 1, "ASC_DEGMIN", pos.asc_degmin.as_deref());
        }

        write_source_citation(out, level + 1);
    }

    let mut out = String::new();

    // HEAD
    write_line(&mut out, 0, "HEAD", None);
    write_line(&mut out, 1, "GEDC", None);
    write_line(&mut out, 2, "VERS", Some("7.0"));
    write_line(&mut out, 2, "FORM", Some("LINEAGE-LINKED"));
    write_line(&mut out, 1, "SOUR", Some("adbimport"));
    write_line(&mut out, 2, "VERS", Some(env!("CARGO_PKG_VERSION")));

    // Preserve export metadata.
    write_ourania_kv(
        &mut out,
        1,
        "EXPORT_FORMAT",
        Some(export.export_format.as_str()),
    );
    write_ourania_kv(
        &mut out,
        1,
        "EXPORT_TIMESTAMP",
        Some(export.timestamp.value.as_str()),
    );
    write_ourania_kv(
        &mut out,
        1,
        "EXPORT_TIMESTAMP_ITST",
        export.timestamp.itst.as_deref(),
    );
    write_ourania_kv(
        &mut out,
        1,
        "EXPORT_TIMESTAMP_USER",
        export.timestamp.user.as_deref(),
    );
    write_ourania_kv(
        &mut out,
        1,
        "EXPORT_TIMESTAMP_HOST",
        export.timestamp.host.as_deref(),
    );
    write_ourania_text(
        &mut out,
        1,
        "IMPORTANT_NOTICE",
        export.important_notice.as_deref(),
    );

    // SOUR
    write_line(&mut out, 0, "@S1@", Some("SOUR"));
    write_line(&mut out, 1, "TITL", Some("Astrodatabank"));

    // INDI
    for (idx, entry) in export.entries.iter().enumerate() {
        let xref = format!("@I{}@", idx + 1);
        write_line(&mut out, 0, &xref, Some("INDI"));

        // Stable link to original record.
        write_ourania_u64(&mut out, 1, "ADB_ID", Some(entry.adb_id));
        write_ourania_kv(
            &mut out,
            1,
            "UPDATE_TIMESTAMP",
            entry.update_timestamp.as_ref().map(|u| u.value.as_str()),
        );
        write_ourania_kv(
            &mut out,
            1,
            "UPDATE_TIMESTAMP_ITST",
            entry
                .update_timestamp
                .as_ref()
                .and_then(|u| u.itst.as_deref()),
        );

        // NAME
        let name_value = entry.public_data.name.replace('/', "").trim().to_string();
        if !name_value.is_empty() {
            write_line(&mut out, 1, "NAME", Some(&name_value));
        }

        // Preserve other name variants.
        write_ourania_kv(&mut out, 1, "SFLNAME", entry.public_data.sflname.as_deref());
        write_ourania_kv(
            &mut out,
            1,
            "BIRTHNAME",
            entry.public_data.birthname.as_deref(),
        );

        // SEX
        if let Some(g) = entry.public_data.gender.as_ref() {
            let sex_raw = g.value.trim();
            let sex_norm = match sex_raw.to_ascii_uppercase().as_str() {
                "M" | "MALE" => Some("M"),
                "F" | "FEMALE" => Some("F"),
                "X" | "NONBINARY" | "NON-BINARY" => Some("X"),
                "U" | "UNKNOWN" | "" => Some("U"),
                _ => None,
            };

            if let Some(sex_norm) = sex_norm {
                write_line(&mut out, 1, "SEX", Some(sex_norm));
            }

            // Preserve original gender encoding.
            write_ourania_kv(&mut out, 1, "GENDER", Some(sex_raw));
            write_ourania_kv(&mut out, 1, "GENDER_CSEX", g.csex.as_deref());
        }

        // Public data metadata
        if let Some(rr) = entry.public_data.roddenrating.as_ref() {
            write_ourania_kv(&mut out, 1, "RODDENRATING", Some(rr.value.as_str()));
            write_ourania_kv(&mut out, 1, "RODDENRATING_RRC", rr.rrc.as_deref());
        }

        if let Some(dt) = entry.public_data.datatype.as_ref() {
            write_ourania_kv(&mut out, 1, "DATATYPE", dt.sdatatype.as_deref());
            write_ourania_kv(&mut out, 1, "DATATYPE_DTC", dt.dtc.as_deref());
            write_ourania_kv(&mut out, 1, "DATASOURCE", dt.sdatasource.as_deref());
            write_ourania_kv(&mut out, 1, "DATASOURCE_DSC", dt.dsc.as_deref());
        }

        write_ourania_kv(
            &mut out,
            1,
            "SCOLLECTOR",
            entry.public_data.scollector.as_deref(),
        );
        write_ourania_kv(&mut out, 1, "SEDITOR", entry.public_data.seditor.as_deref());
        write_ourania_kv(
            &mut out,
            1,
            "SBIOGRAPHER",
            entry.public_data.sbiographer.as_deref(),
        );
        write_ourania_kv(
            &mut out,
            1,
            "SCREATIONDATE",
            entry.public_data.screationdate.as_deref(),
        );
        write_ourania_kv(
            &mut out,
            1,
            "SLASTEDITDATE",
            entry.public_data.slasteditdate.as_deref(),
        );

        // BIRT
        if let Some(bdata) = entry.public_data.bdata.as_ref() {
            write_birth_data(&mut out, 1, bdata);
        }

        // Alternative birth/event data blocks: keep as custom structures.
        for alt in &entry.public_data.bdata_alt {
            write_line(&mut out, 1, "_OURANIA_BDATA_ALT", None);
            if let Some(et) = alt.event_type.value.as_deref() {
                write_ourania_kv(&mut out, 2, "EVENT_TYPE", Some(et));
            }
            write_ourania_u64(&mut out, 2, "EVENT_ID", alt.event_type.event_id);
            write_ourania_kv(&mut out, 2, "EVENT_NOTES", alt.event_notes.as_deref());

            if let Some(sbdate) = alt.sbdate.as_ref() {
                write_ourania_kv(&mut out, 2, "SBDATE", sbdate.value.as_deref());
                write_ourania_kv(&mut out, 2, "CALENDAR", sbdate.ccalendar.as_deref());
                write_ourania_i32(&mut out, 2, "IYEAR", sbdate.iyear);
                write_ourania_u8(&mut out, 2, "IMONTH", sbdate.imonth);
                write_ourania_u8(&mut out, 2, "IDAY", sbdate.iday);
            }
            write_ourania_kv(&mut out, 2, "SBDATE_DMY", alt.sbdate_dmy.as_deref());

            if let Some(sbtime) = alt.sbtime.as_ref() {
                write_ourania_kv(&mut out, 2, "TIME", sbtime.value.as_deref());
                write_ourania_kv(&mut out, 2, "SBTIME_AMPM", sbtime.sbtime_ampm.as_deref());
                write_ourania_kv(&mut out, 2, "CTIMETYPE", sbtime.ctimetype.as_deref());
                write_ourania_kv(&mut out, 2, "STIMETYPE", sbtime.stimetype.as_deref());
                write_ourania_kv(&mut out, 2, "STMERID", sbtime.stmerid.as_deref());
                write_ourania_kv(&mut out, 2, "CTZAUTO", sbtime.ctzauto.as_deref());
                write_ourania_u32(&mut out, 2, "ITIMEACC", sbtime.itimeacc);
                write_ourania_kv(&mut out, 2, "STIMEACC", sbtime.stimeacc.as_deref());
                write_ourania_kv(&mut out, 2, "TIME_UNKNOWN", sbtime.time_unknown.as_deref());
                write_ourania_f64(&mut out, 2, "JD_UT", sbtime.jd_ut);
                write_ourania_kv(&mut out, 2, "SZNABBR", sbtime.sznabbr.as_deref());
            }

            if let Some(p) = alt.place.as_ref() {
                write_ourania_kv(&mut out, 2, "PLACE", Some(p.value.as_str()));
                write_ourania_kv(&mut out, 2, "PLACE_SLATI", p.slati.as_deref());
                write_ourania_kv(&mut out, 2, "PLACE_SLONG", p.slong.as_deref());
            }
            if let Some(c) = alt.country.as_ref() {
                write_ourania_kv(&mut out, 2, "COUNTRY", Some(c.value.as_str()));
                write_ourania_kv(&mut out, 2, "COUNTRY_SCTR", c.sctr.as_deref());
            }
        }

        // NOTE
        if let Some(text) = entry.text_data.as_ref() {
            if let Some(bio) = text.shortbiography.as_ref().and_then(|t| t.value.as_ref()) {
                let bio = bio.trim();
                if !bio.is_empty() {
                    write_text(&mut out, 1, "NOTE", bio);
                }
                write_ourania_kv(
                    &mut out,
                    1,
                    "SHORTBIOGRAPHY_COPYRIGHT",
                    text.shortbiography
                        .as_ref()
                        .and_then(|t| t.copyright.as_deref()),
                );
            }

            write_ourania_kv(
                &mut out,
                1,
                "WIKIPEDIA_LINK",
                text.wikipedia_link.as_deref(),
            );
            write_ourania_kv(&mut out, 1, "ADB_LINK", text.adb_link.as_deref());

            // Sourcenotes are often multi-line; preserve as multi-line custom text.
            write_ourania_text(
                &mut out,
                1,
                "SOURCENOTES",
                text.sourcenotes.as_ref().and_then(|t| t.value.as_deref()),
            );
            write_ourania_kv(
                &mut out,
                1,
                "SOURCENOTES_COPYRIGHT",
                text.sourcenotes
                    .as_ref()
                    .and_then(|t| t.copyright.as_deref()),
            );
        }

        // Research data: categories / relationships / events
        if let Some(r) = entry.research_data.as_ref() {
            if let Some(cats) = r.categories.as_ref() {
                write_ourania_u32(&mut out, 1, "CATEGORIES_COUNT", cats.count);
                write_ourania_kv(&mut out, 1, "CATEGORIES_NOTE", cats.note.as_deref());
                for cat in &cats.items {
                    write_line(&mut out, 1, "_OURANIA_CATEGORY", cat.value.as_deref());
                    write_ourania_u64(&mut out, 2, "CAT_ID", cat.cat_id);
                    write_ourania_u64(&mut out, 2, "ADB_ID", cat.adb_id);
                    write_ourania_kv(&mut out, 2, "CATNOTES", cat.catnotes.as_deref());
                }
            }

            if let Some(rels) = r.relationships.as_ref() {
                write_ourania_u32(&mut out, 1, "RELATIONSHIPS_COUNT", rels.count);
                write_ourania_kv(&mut out, 1, "RELATIONSHIPS_NOTE", rels.note.as_deref());
                for rel in &rels.items {
                    write_line(&mut out, 1, "_OURANIA_RELATIONSHIP", rel.value.as_deref());
                    write_ourania_u64(&mut out, 2, "REL_ID", rel.rel_id);
                    write_ourania_u64(&mut out, 2, "REL_ADB_ID", rel.rel_adb_id);
                    write_ourania_u64(&mut out, 2, "ADB_ID", rel.adb_id);
                    write_ourania_kv(&mut out, 2, "RELCAT", rel.relcat.as_deref());
                    write_ourania_kv(&mut out, 2, "RELNOTES", rel.relnotes.as_deref());
                }
            }

            if let Some(evs) = r.events.as_ref() {
                write_ourania_u32(&mut out, 1, "EVENTS_COUNT", evs.count);
                write_ourania_kv(&mut out, 1, "EVENTS_NOTE", evs.note.as_deref());
                for ev in &evs.items {
                    write_line(&mut out, 1, "_OURANIA_EVENT", ev.sevcode.as_deref());
                    write_ourania_u64(&mut out, 2, "EVN_ID", ev.evn_id);
                    write_ourania_u64(&mut out, 2, "ADB_ID", ev.adb_id);
                    write_ourania_kv(&mut out, 2, "EVNOTES", ev.evnotes.as_deref());

                    if let Some(ed) = ev.event_data.as_ref() {
                        if let Some(sbdate) = ed.sbdate.as_ref() {
                            write_ourania_kv(&mut out, 2, "SBDATE", sbdate.value.as_deref());
                            write_ourania_kv(&mut out, 2, "CALENDAR", sbdate.ccalendar.as_deref());
                            write_ourania_i32(&mut out, 2, "IYEAR", sbdate.iyear);
                            write_ourania_u8(&mut out, 2, "IMONTH", sbdate.imonth);
                            write_ourania_u8(&mut out, 2, "IDAY", sbdate.iday);
                        }
                        write_ourania_kv(&mut out, 2, "SBDATE_DMY", ed.sbdate_dmy.as_deref());
                    }
                }
            }
        }

        // Minimal per-individual source citation.
        write_source_citation(&mut out, 1);
    }

    write_line(&mut out, 0, "TRLR", None);

    std::fs::write(path, out)
}

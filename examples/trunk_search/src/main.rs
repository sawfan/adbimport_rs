#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // This example is intended to be built and served with Trunk for the wasm32 target.
}

// When building for wasm32 as a *binary* crate, Rust still requires a crate-level
// `main` function to exist.
#[cfg(target_arch = "wasm32")]
fn main() {}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    wasm_app::start()
}

#[cfg(target_arch = "wasm32")]
mod wasm_app {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    use web_sys::{
        Document, HtmlElement, HtmlInputElement, Request, RequestInit, RequestMode, Response,
    };

    use adbimport::genealogy::{ArchivedEventKind, EventId, GenealogyStore};

    fn esc(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    fn format_positions(sign: Option<&str>, degmin: Option<&str>) -> String {
        match (sign, degmin) {
            (None, None) => String::new(),
            (Some(sign), None) => sign.to_string(),
            (None, Some(degmin)) => degmin.to_string(),
            (Some(sign), Some(degmin)) => format!("{sign} {degmin}"),
        }
    }

    fn document() -> Document {
        web_sys::window().expect("no window").document().expect("no document")
    }

    fn set_text(id: &str, text: &str) {
        let doc = document();
        let el = doc
            .get_element_by_id(id)
            .unwrap_or_else(|| panic!("missing element #{id}"));
        el.set_text_content(Some(text));
    }

    fn get_input_value(id: &str) -> String {
        let doc = document();
        let el = doc
            .get_element_by_id(id)
            .unwrap_or_else(|| panic!("missing element #{id}"));
        el.dyn_into::<HtmlInputElement>()
            .expect("not an input")
            .value()
    }

    async fn fetch_bytes(url: &str) -> Result<Vec<u8>, JsValue> {
        let opts = RequestInit::new();
        opts.set_method("GET");
        opts.set_mode(RequestMode::SameOrigin);

        let request = Request::new_with_str_and_init(url, &opts)?;

        let window = web_sys::window().expect("no window");
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: Response = resp_value.dyn_into()?;

        let buf = JsFuture::from(resp.array_buffer()?).await?;
        let array = js_sys::Uint8Array::new(&buf);

        let mut bytes = vec![0u8; array.length() as usize];
        array.copy_to(&mut bytes);
        Ok(bytes)
    }

    fn find_birth_event<'a>(
        store: &'a GenealogyStore,
        person: &rkyv::Archived<adbimport::genealogy::Person>,
    ) -> Result<Option<&'a rkyv::Archived<adbimport::genealogy::Event>>, JsValue> {
        for event_id in person.events.iter() {
            let event = store
                .event(EventId(event_id.0.into()))
                .map_err(|e| JsValue::from_str(&e.to_string()))?;
            let Some(event) = event else {
                continue;
            };

            if matches!(event.kind, ArchivedEventKind::Birth) {
                return Ok(Some(event));
            }
        }

        Ok(None)
    }

    fn format_results(store: &GenealogyStore, query: &str) -> Result<String, JsValue> {
        let hits = store
            .search_people_by_name(query, 25)
            .map_err(|e| JsValue::from_str(&e.to_string()))?;

        let mut out = String::new();
        out.push_str(&format!("<div>hits={}</div>", hits.len()));

        out.push_str("<table>");
        out.push_str(
            "<thead><tr><th>ID</th><th>Name</th><th>Birth date</th><th>Birth time</th><th>Sun</th><th>Moon</th><th>Asc</th></tr></thead>",
        );
        out.push_str("<tbody>");

        for p in hits {
            let id = p.id.0;
            let name = p
                .names
                .first()
                .map(|n| n.display.as_str())
                .unwrap_or("<no name>");

            let (birth_date, birth_time, sun, moon, asc) = match find_birth_event(store, p)? {
                None => (String::new(), String::new(), String::new(), String::new(), String::new()),
                Some(ev) => {
                    let date = ev
                        .date
                        .as_ref()
                        .map(|d| d.original.as_str())
                        .unwrap_or("")
                        .to_string();

                    let time = match (ev.time.as_deref(), ev.time_zone.as_deref()) {
                        (None, None) => String::new(),
                        (Some(t), None) => t.to_string(),
                        (None, Some(tz)) => tz.to_string(),
                        (Some(t), Some(tz)) => format!("{t} {tz}"),
                    };

                    let (sun, moon, asc) = match ev.positions.as_ref() {
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

            out.push_str("<tr>");
            out.push_str(&format!("<td>{}</td>", id));
            out.push_str(&format!("<td class=\"name\">{}</td>", esc(name)));
            out.push_str(&format!("<td>{}</td>", esc(&birth_date)));
            out.push_str(&format!("<td>{}</td>", esc(&birth_time)));
            out.push_str(&format!("<td>{}</td>", esc(&sun)));
            out.push_str(&format!("<td>{}</td>", esc(&moon)));
            out.push_str(&format!("<td>{}</td>", esc(&asc)));
            out.push_str("</tr>");
        }

        out.push_str("</tbody></table>");
        Ok(out)
    }

    pub fn start() -> Result<(), JsValue> {
        // Spawn an async task so we can fetch assets.
        wasm_bindgen_futures::spawn_local(async move {
            set_text("status", "Loading public/sample.rkyv...");

            let bytes = match fetch_bytes("public/sample.rkyv").await {
                Ok(bytes) => bytes,
                Err(err) => {
                    set_text("status", &format!("Failed to fetch sample.rkyv: {err:?}"));
                    return;
                }
            };

            let store = match GenealogyStore::from_bytes(bytes) {
                Ok(store) => store,
                Err(err) => {
                    set_text("status", &format!("Failed to load archive: {err}"));
                    return;
                }
            };

            // Render initial results.
            let query = get_input_value("query");
            match format_results(&store, &query) {
                Ok(text) => {
                    set_text("status", "Ready");
                    let doc = document();
                    let el = doc
                        .get_element_by_id("results")
                        .unwrap_or_else(|| panic!("missing element #results"));
                    el.dyn_into::<HtmlElement>()
                        .expect("#results not an element")
                        .set_inner_html(&text);
                }
                Err(err) => {
                    set_text("status", &format!("Search failed: {err:?}"));
                }
            }

            // Hook up button.
            let doc = document();
            let button = doc
                .get_element_by_id("search")
                .unwrap_or_else(|| panic!("missing #search"))
                .dyn_into::<HtmlElement>()
                .expect("#search not an element");

            let store = std::rc::Rc::new(store);
            let handler_store = store.clone();

            let closure = Closure::<dyn FnMut()>::new(move || {
                let query = get_input_value("query");
                match format_results(&handler_store, &query) {
                    Ok(text) => {
                        set_text("status", "Ready");
                        let doc = document();
                        let el = doc
                            .get_element_by_id("results")
                            .unwrap_or_else(|| panic!("missing element #results"));
                        el.dyn_into::<HtmlElement>()
                            .expect("#results not an element")
                            .set_inner_html(&text);
                    }
                    Err(err) => {
                        set_text("status", &format!("Search failed: {err:?}"));
                    }
                }
            });

            button.set_onclick(Some(closure.as_ref().unchecked_ref()));
            closure.forget();

            // Keep store alive for the lifetime of the page.
            std::mem::forget(store);
        });

        Ok(())
    }
}


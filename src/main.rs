#![allow(non_snake_case)]
use std::collections::BTreeMap;

use dioxus::prelude::*;

use paste::paste;

use web_sys;

#[cfg(test)]
mod util;

macro_rules! days {
    ($($day:expr),*) => {
        paste! {
            $(mod [<day_ $day>];)*

            const DAYS: &'static [Day] = &[
                $(
                    Day::new(
                        $day,
                        include_str!(concat!("day_", stringify!($day), ".rs")),
                        [<day_ $day>]::solve_1,
                        [<day_ $day>]::solve_2,
                    ),
                )*
            ];
        }
    };
}

days!(01, 02, 03, 04, 05, 06, 07, 08, 09);

struct Day {
    pub ord: u32,
    pub code: &'static str,
    pub solve1: fn(&str) -> String,
    pub solve2: fn(&str) -> String,
}

impl Day {
    const fn new(
        ord: u32,
        code: &'static str,
        solve1: fn(&str) -> String,
        solve2: fn(&str) -> String,
    ) -> Self {
        Self { ord, code, solve1, solve2 }
    }
}

struct Puzzle {
    pub code: &'static str,
    pub solve: fn(&str) -> String,
}

impl Puzzle {
    const fn new(
        code: &'static str,
        solve: fn(&str) -> String,
    ) -> Self {
        Self { code, solve }
    }
}

fn main() {
    dioxus_logger::init(log::LevelFilter::Info).expect("failed to init logger");
    dioxus_web::launch(App);
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col flex-1 px-2 xl:px-12",
            div {
                h1 {
                    class: "text-xl text-center",
                    "Advent of Code 2023 Solver"
                }
            }
            Solver {}
        }
    }
}

fn Solver(cx: Scope) -> Element {
    let src = use_state(cx, || DAYS.last().unwrap().code);
    let answer = use_state(cx, || None);
    let duration = use_state(cx, || None);

    let window = web_sys::window().expect("Window object");
    let perf = window.performance().expect("Performance");
    let location = window.location();
    let hash = location.hash();
    let cur_puzzle_id = if let Ok(Some(cur_puzzle)) = hash.as_ref().map(|h| h.strip_prefix("#")) {
        cur_puzzle.to_string()
    } else {
        "01-1".to_string()
    };
    let puzzles = DAYS.iter()
        .flat_map(|d| {
            [
                (format!("{:0>2}-1", d.ord), Puzzle::new(d.code, d.solve1)),
                (format!("{:0>2}-2", d.ord), Puzzle::new(d.code, d.solve2))
            ]
        })
        .collect::<BTreeMap<_, _>>();

    render!{
        form {
            onsubmit: move |event| {
                let input = &event.data.values["input"][0];
                let (res, dur) = if let Some(puzzle) = &puzzles.get(event.data.values["task"][0].as_str()) {
                    let start = perf.now();
                    let res = (puzzle.solve)(input);
                    let end = perf.now();
                    (res, Some(end - start))
                } else {
                    ("Invalid task".to_string(), None)
                };
                answer.set(Some(res));
                duration.set(dur);
            },
            Puzzles {
                cur_puzzle_id: cur_puzzle_id,
                src: src,
            }
            div {
                class: "columns-1 xl:columns-2 gap-2",
                div {
                    class: "w-full",
                    div {
                        class: "py-2",
                        legend {
                            "Input data"
                        }
                        textarea {
                            name: "input",
                            placeholder: "Paste your input data",
                            class: "block p-2 w-full resize border bg-gray-50",
                            rows: "20",
                        }
                    }
                    div {
                        class: "grid grid-cols-4",
                        div {
                            button {
                                class: "inline-flex justify-center px-4 py-1 text-white bg-blue-600 border border-blue-700 rounded-md",
                                "Solve it"
                            }
                        }
                        div {
                            class: "col-span-3 py-1",
                            p {
                                span { class: "pl-2", "Result: " }
                                span { answer.as_ref().map(|a| rsx!{ "{a}" }) }
                            }
                            p {
                                span { class: "pl-2", "Duration: " }
                                span { duration.as_ref().map(|d| rsx!{ "{d}ms" }) }
                            }
                        }
                    }
                }
                div {
                    id: "code",
                    class: "border p-2 w-full overflow-x-auto",
                    Source {
                        code: src.to_string()
                    }
                }
            }
        }
    }
}

#[inline_props]
fn Puzzles<'a>(
    cx: Scope,
    cur_puzzle_id: String,
    src: &'a UseState<&'static str>,
) -> Element {
    render!{
        div {
            class: "py-2",
            legend {
                class: "p-2",
                "Choose task"
            }
            div {
                class: "grid auto-cols-min gap-2 grid-cols-[repeat(auto-fill,_minmax(50px,_1fr))]",
                DAYS.iter().enumerate().map(|(day_ord, day)| {
                    let day_ord = day_ord as u32 + 1;
                    rsx!{
                        div {
                            PuzzleButton {
                                day_ord: day_ord,
                                puzzle_ord: 1,
                                code: day.code,
                                cur_puzzle_id: &cur_puzzle_id,
                                src: src,
                            }
                            PuzzleButton {
                                day_ord: day_ord,
                                puzzle_ord: 2,
                                code: day.code,
                                cur_puzzle_id: &cur_puzzle_id,
                                src: src,
                            }
                        }
                    }
                })
            }
        }
    }
}

#[inline_props]
fn PuzzleButton<'a>(
    cx: Scope,
    day_ord: u32,
    puzzle_ord: u32,
    code: &'static str,
    cur_puzzle_id: &'a str,
    src: &'a UseState<&'static str>,
) -> Element {
    let window = web_sys::window().expect("Window object");
    let location = window.location();
    let puzzle_id = format!("{day_ord:0>2}-{puzzle_ord}");
    let new_hash = format!("#{puzzle_id}");

    render!{
        div {
            class: "py-0.5 text-center",
            input {
                id: "puzzle-{puzzle_id}",
                r#type: "radio",
                name: "puzzle",
                value: "{puzzle_id}",
                checked: &puzzle_id == cur_puzzle_id,
                class: "hidden peer",
            }
            label {
                r#for: "puzzle-{puzzle_id}",
                class: "block whitespace-nowrap p-2 border rounded-lg cursor-pointer hover:text-gray-600 hover:bg-gray-100 peer-checked:border-blue-600 peer-checked:text-blue-600",
                onclick: move |event| {
                    src.set(code);
                    location.set_hash(&new_hash).ok();
                    event.stop_propagation();
                },
                "{puzzle_id}"
            }
        }
    }
}

#[inline_props]
fn Source(cx: Scope, code: String) -> Element {
    // let code = cx.props.code.clone();
    // log::info!("Got code: {}", &code);

    let create_eval = use_eval(cx);
    let hl_code = use_state(cx, String::new);

    let future = use_future!(cx, |code| {
        to_owned![create_eval];
        let hl_code = hl_code.clone();
        async move {
            // log::info!("Evaluating JS code...");
            let eval = create_eval(
                r#"
                let code = await dioxus.recv();
                let hlCode = hljs.highlight(code, {"language": "rust"}).value;
                let hlCodeWithLines = hljs.lineNumbersValue(hlCode, {});
                dioxus.send(hlCodeWithLines);
                "#,
            )
            .unwrap();
            eval.send(serde_json::Value::String(code.to_string())).unwrap();

            if let serde_json::Value::String(res) = eval.recv().await.unwrap() {
                hl_code.set(res);
            }
        }
    });
    future.value();
    render!{
        pre {
            code {
                class: "language-rust",
                dangerous_inner_html: "{hl_code}"
            }
        }
    }
}

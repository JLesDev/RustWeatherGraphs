use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use reqwest::Error;
use serde::Deserialize;
use std::error::Error as StdError;  // Import standard error trait
use std::{collections::BTreeMap, sync::Arc};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;
use gloo::*;
use gloo_console::log;
use wasm_bindgen::prelude::*;
use chart_js_rs::{bar::Bar, doughnut::Doughnut, pie::Pie, scatter::Scatter, *};
use itertools::Itertools;
use dominator::{events, html, Dom};
use futures_signals::signal::{Mutable, Signal, SignalExt};
use futures::executor::block_on;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use std::collections::HashMap;

mod utils;

fn random() -> Vec<usize> {
    
    // let rng = rand::thread_rng();

    // let rnd = (0..=20).map(|_| rng.clone().gen_range(1..50));

    // rnd.collect()

    let rnd = (0..=20).map(|_| {
        let mut buf: [u8; 32] = Default::default();
        getrandom::getrandom(&mut buf).unwrap();
        let mut rng = rand::prelude::StdRng::from_seed(buf);
                    //let mut test = rand::rng();
                    //let test = rand::random_range(1..50);
        rng.gen_range(1..50)
        // rng.5;
    });

    rnd.collect()
}

pub struct Model {
    tick: Mutable<bool>,
    chart: Mutable<Arc<str>>,
    //x: Mutable<Arc<Vec<String>>>,
    x: Mutable<Arc<BTreeMap<usize, std::string::String>>>,
    //x: Mutable<Arc<NumberOrDateString>>,
    y1: Mutable<Arc<Vec<f64>>>,
    y2: Mutable<Arc<Vec<f64>>>,
    y3: Mutable<Arc<Vec<f64>>>,
    y4: Mutable<Arc<Vec<f64>>>,
    y5: Mutable<Arc<Vec<usize>>>,
    y6: Mutable<Arc<Vec<usize>>>,
    y7: Mutable<Arc<Vec<usize>>>,
    y8: Mutable<Arc<Vec<usize>>>,
    y9: Mutable<Arc<Vec<usize>>>,
    y10: Mutable<Arc<Vec<usize>>>,
    y11: Mutable<Arc<Vec<usize>>>,
    y12: Mutable<Arc<Vec<usize>>>,
}

impl Model {
    async fn init() -> Arc<Self> {
        let query_string = gloo::utils::window()
            .location()
            .search()
            .unwrap_or_default()
            .replace('?', "");
        let query = query_string
            .split('=')
            .tuples::<(&str, &str)>()
            .collect::<BTreeMap<&str, &str>>();
        let melb_olymp= prog_float(1, 0).await;
        let avalon = prog_float(1, 1).await;
        let cerebus = prog_float(1, 2).await;
        let coldstream = prog_float(1, 3).await;
        let essendon = prog_usize(1, 4).await;
        let frankston = prog_usize(1, 5).await;
        let ferny_creek = prog_usize(1, 6).await;
        let geelong = prog_usize(1, 7).await;
        let laverton = prog_usize(1, 8).await;
        let moorabbin = prog_usize(1, 9).await;
        let scoresby = prog_usize(1, 10).await;
        let viewbank = prog_usize(1, 11).await;

        let time = prog_usize(1, 0).await;
        let mut time: Vec<String> = Vec::new();

        for i in 0..14{
            let a = prog_string().await;
            //let b = a.to_string();
            time.push(a[i].clone());
            //time.push(i.to_string());
        }

        let mut time_true = BTreeMap::new();

        for i in 0..14{
            let a = prog_string().await;
            time_true.insert(i, a[i].clone());
            gloo_console::log!(format!("Continuous hashmap index: {:?}", i));
        }

        gloo_console::log!(format!("In model, data: {:?}", melb_olymp));
        gloo_console::log!(format!("In model, time: {:?}", time));
        gloo_console::log!(format!("In model, time as hashmap: {:?}", time_true.keys().sorted()));

        Arc::new( Model  {
            tick: Mutable::default(),
            chart: Mutable::new(query.get("chart").cloned().unwrap_or("line").into()), //INTERESTING
            //x: Mutable::new(Arc::new((0..=20).collect())),
            x: Mutable::new(Arc::new(time_true /*random()*/)),
            y1: Mutable::new(Arc::new(melb_olymp)),
            y2: Mutable::new(Arc::new(avalon)),
            y3: Mutable::new(Arc::new(cerebus)),
            y4: Mutable::new(Arc::new(coldstream)),
            y5: Mutable::new(Arc::new(essendon)),
            y6: Mutable::new(Arc::new(frankston)),
            y7: Mutable::new(Arc::new(ferny_creek)),
            y8: Mutable::new(Arc::new(geelong)),
            y9: Mutable::new(Arc::new(laverton)),
            y10: Mutable::new(Arc::new(moorabbin)),
            y11: Mutable::new(Arc::new(scoresby)),
            y12: Mutable::new(Arc::new(viewbank)),
        })
    }

    fn set_query(self: Arc<Model>) {
        gloo::utils::window()
            .location()
            .set_search(&format!("chart={}", self.chart.get_cloned()))
            .unwrap();
    }

    fn chart_selected(self: Arc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c.as_ref() == chart)
    }
    fn chart_not_selected(self: Arc<Self>, chart: &'static str) -> impl Signal<Item = bool> {
        self.chart.signal_cloned().map(move |c| c.as_ref() != chart)
    }

    fn show_charts(self: Arc<Self>) -> impl Signal<Item = Option<Dom>> {
        Mutable6::new(
            self.chart.clone(),
            self.x.clone(),
            self.y1.clone(),
            self.y2.clone(),
            self.y3.clone(),
            self.y4.clone(),
        )
        .map(move |(c, x, y1, y2, y3, y4)| match c.to_string().as_str() {
            // "scatter" => Some(self.clone().show_scatter(
            //     x.as_slice(), //.as_bytes(),
            //     y1.as_slice(),
            //     y2.as_slice(),
            // )),
            // "bar" => Some(self.clone().show_bar(y1.as_slice())),
            // "donut" => Some(self.clone().show_donut()),
            "line" => Some(
                self.clone()
                    .show_line(x/*.as_slice()*/, y1.as_slice(), y2.as_slice(), y3.as_slice(), y4.as_slice()),
            ),
            _ => None,
        })
    }

    fn show_scatter(self: Arc<Self>, x: &[usize], y1: &[usize], y2: &[usize]) -> Dom {
        // construct and render chart here
        let id = "scatter";
        //let b: Vec<String> = prog_notasync();
        //let dataa = prog_notasync();
        //let z: Vec<String> = vec![];
        let chart = Scatter::<NoAnnotations>::new(id)
            // we use <NoAnnotations> here to type hint for the compiler
            .data(
                Dataset::new().datasets([
                    XYDataset::new()
                        .data(x.iter().zip(y1).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        .border_color("black")
                        .background_color("lightblue")
                        .point_radius(45)
                        .label("Melbourne (Olympic Park)"), // change this to a variable that gets location from json
                ]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                chart.into_chart().mutate().render();
            })
        })
    }

    fn show_line(self: Arc<Self>, x: Arc<BTreeMap<usize, std::string::String>>, y1: &[f64], y2: &[f64], y3: &[f64], y4: &[f64]) -> Dom {
        // construct and render chart here
        let id = "line";
        //let dataa = prog_notasync();
        gloo_console::log!("The 2nd value in hashmap: {:?}",  x.iter().map(|(_, value)| value.clone()).collect::<Vec<_>>());
        let chart = Scatter::<NoAnnotations>::new(id)
            // we use <NoAnnotations> here to type hint for the compiler
            .data(
                Dataset::new().labels(
                    x.iter().map(|(_, value)| value.clone()).collect::<Vec<_>>(),
                )
                .datasets([
                    XYDataset::new()
                    .data(
                        x.iter()
                            .zip(y1)
                            .map(|((key, value), y)| (*key as f64, y)) // Convert key and value
                            //.sorted()
                            .into_data_iter()
                            .unsorted_to_dataset_data(),
                    )
                    .border_color("red")
                    .background_color("lightcoral")
                    .point_radius(5)
                    .dataset_type("line")
                    .label("Melbourne"),
                    // .data(
                    //     x.iter()
                    //         .zip(y1)
                    //         .map(|((&key, value), y)| (key as f64, y)) // Adjust for the structure of zipped iterators
                    //         .into_data_iter()
                    //         .unsorted_to_dataset_data(),
                    // )
                    

                     
                        // .data(x.iter().zip(y1).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        // .border_color("red")
                        // .background_color("lightcoral")
                        // .point_radius(5)
                        // .dataset_type("line")
                        // .label("Melbourne"),
                    XYDataset::new()
                    .data(
                        x.iter()
                            .zip(y2)
                            .map(|((key, value), y)| (*key as f64, y)) // Convert key and value
                            .into_data_iter()
                            .unsorted_to_dataset_data(),
                    ) // collect into dataset
                        .border_color("blue")
                        .background_color("lightskyblue")
                        .point_radius(5)
                        .dataset_type("line")
                        .label("Sydney"),

                        // .data(x.iter().zip(y2).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        // .border_color("blue")
                        // .background_color("lightskyblue")
                        // .point_radius(5)
                        // .dataset_type("line")
                        // .label("Sydney"),
                    XYDataset::new()
                    .data(
                        x.iter()
                            .zip(y3)
                            .map(|((key, value), y)| (*key as f64, y)) // Convert key and value
                            .into_data_iter()
                            .unsorted_to_dataset_data(),
                    ) // collect into dataset
                        .border_color("blue")
                        .background_color("green")
                        .point_radius(5)
                        .dataset_type("line")
                        .label("Brisbane"),

                        // .data(x.iter().zip(y3).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        // .border_color("blue")
                        // .background_color("green")
                        // .point_radius(5)
                        // .dataset_type("line")
                        // .label("Brisbane"),
                    XYDataset::new()
                    .data(
                        x.iter()
                            .zip(y4)
                            .map(|((key, value), y)| (*key as f64, y)) // Convert key and value
                            .into_data_iter()
                            .unsorted_to_dataset_data(),
                    ) // collect into dataset
                        .border_color("blue")
                        .background_color("pink")
                        .point_radius(5)
                        .dataset_type("line")
                        .label("Adelaide"),
                        // .data(x.iter().zip(y4).into_data_iter().unsorted_to_dataset_data()) // collect into dataset
                        // .border_color("blue")
                        // .background_color("pink")
                        // .point_radius(5)
                        // .dataset_type("line")
                        // .label("Adelaide"),
                ]),
            )
            /*
              scales: {
                y: {
                title: {
                    display: true,
                    text: 'Your Title'
                }
              }
            */

            // x.iter().map(|(_, value)| value.clone()).collect::<Vec<_>>(),

            .options(
                ChartOptions::new()
                    .scales([(
                        "x",
                        ChartScale::new().scale_type("linear").ticks(
                            ScaleTicks::new().callback(
                                // we can call rust functions in callbacks
                                FnWithArgs::<3>::new()
                                    // we can override any arguments going in, in this case we must as rust cannot handle `this`.
                                    // Note: if you don't define your variables with ``.args([..])`, they get the default label of the letter of the alphabet they're the index of
                                    //       1st arg: `a`
                                    //       2nd arg: `b`
                                    //       ...

                                    // x.iter().map(|(_, value)| value.clone()).collect::<Vec<_>>()

                                    .js_body("var a = this.getLabelForValue(a);")
                                    //.js_body("var a = 1;")
                                    // function pointer goes here - note that the count of arguments must equal the const param (3 in this case)
                                    .run_rust_fn(show_line_ticks),
                            ),
                        ),

                        //pub fn title(mut self, value: impl Into<Title>) -> ChartScale {
                        //    self.title = Some(value.into());
                        //    self
                        //}

                        /*
                        plugins: {
                            title: {
                            display: true,
                            text: 'Chart with Tick Configuration'
                            }
                        },
                         */

                        
                    )])
                    .maintain_aspect_ratio(false),
            );
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                chart.into_chart().mutate().render();
            })
        })
    }

    fn show_bar(self: Arc<Self>, data: &[usize]) -> Dom {
        // construct and render chart here
        let id = "bar";

        let chart = Bar::<NoAnnotations>::new(id)
            // we use <NoAnnotations> here to type hint for the compiler
            .data(
                Dataset::new()
                    .labels(
                        // use a range to give us our X axis labels
                        (0..data.len()).map(|d| d + 1),
                    )
                    .datasets([XYDataset::new()
                        .data(
                            data.iter()
                                .enumerate()
                                .map(|(x, y)| ((x + 1), y))
                                .into_data_iter()
                                .unsorted_to_dataset_data(), // collect into dataset
                        )
                        .background_color("palegreen")
                        .border_color("green")
                        .border_width(2)
                        .label("Dataset 1")
                        .y_axis_id("y")]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        html!("canvas", { // construct a html canvas element, and after its rendered into the DOM we can insert our chart
           .prop("id", id)
           .style("height", "calc(100vh - 270px)")
           .after_inserted(move |_| {
                chart.into_chart().render() // use.to_chart().render_mutate(id) if you wish to run some javascript on this chart, for more detail see bar and index.html
            })
        })
    }

    fn show_donut(self: Arc<Self>) -> Dom {
        // construct and render chart here
        let three_a_id = "donut_a";
        let three_b_id = "donut_b";
        // Dataset<Vec<SinglePointDataset>>
        //let dataa = prog_notasync();
        //let dataiter = dataa.iter();
        let three_a_chart = Doughnut::<NoAnnotations>::new(three_a_id)
            .data(// dataa    //format!("{:?}",dataa)
                Dataset::new()
                    .datasets({
                        [SinglePointDataset::new()
                            .data([300, 40, 56, 22])
                            .background_color([
                                "dodgerblue",
                                "limegreen",
                                "firebrick",
                                "goldenrod",
                            ])]
                    })
                    .labels(["Blueberries", "Limes", "Apples", "Lemons"]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        let three_b_chart = Pie::<NoAnnotations>::new(three_b_id)
            .data(
                Dataset::new()
                    .datasets({
                        [SinglePointDataset::new()
                            .data([300, 40, 56, 22])
                            .background_color([
                                "dodgerblue",
                                "limegreen",
                                "firebrick",
                                "goldenrod",
                            ])]
                    })
                    .labels(["Blueberries", "Limes", "Apples", "Lemons"]),
            )
            .options(ChartOptions::new().maintain_aspect_ratio(false));
        html!("div", {
           .class("columns")
           .children([
                html!("div", {
                   .class(["column", "is-half"])
                   .child(
                        html!("canvas", {
                       .prop("id", three_a_id)
                       .style("height", "calc(100vh - 270px)")
                       .after_inserted(move |_| {
                            three_a_chart.into_chart().render()
                        })
                    }))
                }),
                html!("div", {
                   .class(["column", "is-half"])
                   .child(
                        html!("canvas", {
                       .prop("id", three_b_id)
                       .style("height", "calc(100vh - 270px)")
                       .after_inserted(move |_| {
                            three_b_chart.into_chart().render()
                        })
                    }))
                })
            ])
        })
    }

    fn render(self: Arc<Self>) -> Dom {
        html!("div", {
           .class("section")
           .child(
                html!("div", {
                   .class(["buttons", "has-addons"])
                   .child(
                        html!("button", {
                           .class(["button", "is-info"])
                           .prop_signal("disabled", self.clone().chart_selected("donut"))
                           .text("Randomise")
                           .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    // randomise the data on button click
                                    // model.clone().y1.set(Arc::new(random()));
                                    // model.clone().y2.set(Arc::new(random()));
                                }
                            })
                        })
                    )
                   .child(
                        html!("button", {
                           .class(["button", "is-primary"])
                           .class_signal("is-light", self.clone().chart_not_selected("scatter"))
                           .text("Scatter")
                           .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("scatter".into()); // change which chart is in view
                                    model.clone().set_query();
                                }
                            })
                        })
                    )
                   .child(
                        html!("button", {
                           .class(["button", "is-success"])
                           .class_signal("is-light", self.clone().chart_not_selected("line"))
                           .text("Line")
                           .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("line".into()); // change which chart is in view
                                    model.clone().set_query();
                                }
                            })
                        })
                    )
                   .child(
                        html!("button", {
                           .class(["button", "is-primary"])
                           .class_signal("is-light", self.clone().chart_not_selected("bar"))
                           .text("Bar")
                           .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("bar".into()); // change which chart is in view
                                    model.clone().set_query();
                                }
                            })
                        })
                    )
                   .child(
                        html!("button", {
                           .class(["button", "is-success"])
                           .class_signal("is-light", self.clone().chart_not_selected("donut"))
                           .text("Donut")
                           .event({
                                let model = self.clone();
                                move |_: events::Click| {
                                    model.clone().chart.set("donut".into()); // change which chart is in view
                                    model.clone().set_query();
                                }
                            })
                        })
                    )
                   .child_signal(self.chart.signal_cloned().map(|c|
                        if c.as_ref() == "scatter" {
                            Some(html!("button", {
                               .class("button")
                               .prop("disabled", true)
                            }))
                        }
                        else {
                            None
                        })
                    )
                   .child_signal(self.chart.signal_cloned().map({
                        let _self = self.clone();
                        move |c|
                            if c.as_ref() == "scatter" {
                                Some(
                                    html!("button", {
                                       .class(["button", "is-info"])
                                       .text("Update Chart")
                                       .event({
                                            let _self = _self.clone();
                                            move |_: events::Click| {
                                                // update scatter chart colour
                                                let mut chart: Scatter::<NoAnnotations> = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
                                                chart.get_data().get_datasets().get_mut(0).map(|d| {
                                                    if _self.tick.get() {
                                                        *d.get_background_color() = "lightcoral".into();
                                                        *d.get_border_color() = "red".into();
                                                    } else {
                                                        *d.get_background_color() = "palegreen".into();
                                                        *d.get_border_color() = "green".into();
                                                    }
                                                }).unwrap();
                                                chart.into_chart().update(true);
                                                _self.tick.set(!_self.tick.get());
                                            }
                                        })
                                    })
                                )
                            }
                            else {
                                None
                            }
                        })
                    )
                   .child_signal(self.chart.signal_cloned().map({
                        let _self = self.clone();
                        move |c|
                            if c.as_ref() == "scatter" {
                                Some(
                                    html!("button", {
                                       .class(["button", "is-info"])
                                       .text("Update Chart without animation")
                                       .event({
                                            let _self = _self.clone();
                                            move |_: events::Click| {
                                                // update scatter chart colour
                                                let mut chart: Scatter::<NoAnnotations> = ChartExt::get_chart_from_id("scatter").expect("Unable to retrieve chart from JS.");
                                                chart.get_data().get_datasets().get_mut(0).map(|d| {
                                                    if _self.tick.get() {
                                                        *d.get_background_color() = "lightcoral".into();
                                                        *d.get_border_color() = "red".into();
                                                    } else {
                                                        *d.get_background_color() = "palegreen".into();
                                                        *d.get_border_color() = "green".into();
                                                    }
                                                }).unwrap();
                                                chart.into_chart().update(false);
                                                _self.tick.set(!_self.tick.get());
                                            }
                                        })
                                    })
                                )
                            }
                            else {
                                None
                            }
                        })
                    )
                })
            )
           .child(
                html!("div", {
                   .class("section")
                   .child_signal(self.show_charts()) // render charts here, signal allows us to change the chart, see the `Dominator` crate for more
                })
            )
        })
    }
}

#[wasm_bindgen]
pub fn show_line_ticks(this: String, index: u32, _ticks: JsValue) -> String {
    gloo_console::log!("SHOW LINE TICKS");
    let mut time_true = BTreeMap::new();

    for i in 0..14{
        let a = prog_string_notasync();
        time_true.insert(i, a[i].clone());
        gloo_console::log!(format!("Continuous hashmap index: {:?}", i));
    }

    if index % 2 == 0 {
        let j = index as usize;
        //let x =  time_true[&j].map(|(_, value)| value.clone()).collect::<Vec<_>>();
        //this
        //String::new()
        "hi".to_string()
       
    } else {
        this
        //String::new()
    }
}

// #[wasm_bindgen]
// pub fn show_line_ticks(this: String, index: u32, _ticks: JsValue) -> String {
//     gloo_console::log!("SHOW LINE TICKS");
//     let mut time_true = BTreeMap::new();

//     for i in 0..14 {
//         let a = prog_string_notasync();
//         time_true.insert(i, a[i].clone());
//         gloo_console::log!(format!("Continuous hashmap index: {:?}", i));
//     }

//     if index % 2 == 0 {
//         let j = index as usize;
//         let x = time_true[&j].clone(); // No map required for String; just clone it
//         gloo_console::log!(format!("Cloned value: {}", x));
        
//         // Returning the cloned value, or another string
//         "hi".to_string()
//     } else {
//         this
//     }
// }

#[wasm_bindgen]
#[no_mangle]
pub async extern fn test(url_passedd: String){
    let tests = process(url_passedd).await;

    gloo_console::log!(format!("{:?}", tests));
}


pub async fn process2() -> Result<String, Box<dyn StdError>>{

    let body = reqwest::get("http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json")
    .await?
    .text()
    .await?;

    /*
    let client = reqwest::Client::new();
    let res = client.post("http://httpbin.org/post")
        .json(&map)
        .send()
        .await?;
    */

    gloo_console::log!("body = {body:?}");

    Ok(body)
}


pub async fn get_data(location: usize) -> Result<Observations, Box<dyn StdError>>{
    let url = "https://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json".to_string();
    let urls = vec![
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json".to_string(), // Melbourne
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94866.json".to_string(),  // Airport
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94854.json".to_string(),  // Avalon
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94898.json".to_string(),  // Cerebus
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94864.json".to_string(),  // Coldstream
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.95866.json".to_string(),  // Essendon
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94876.json".to_string(),  // Frankston
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94872.json".to_string(),  // Ferny Creek
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94857.json".to_string(),  // Geelong
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94865.json".to_string(),  // Laverton
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.94870.json".to_string(),  // Moorabbin
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.95867.json".to_string(),  // Scoresby
    "http://www.bom.gov.au/fwo/IDV60901/IDV60901.95874.json".to_string(),  // Viewbank
    ];

    let urls = vec![
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json".to_string(), // Melbourne
    "http://reg.bom.gov.au/fwo/IDN60901/IDN60901.94768.json".to_string(),  // Sydney
    "http://reg.bom.gov.au/fwo/IDQ60901/IDQ60901.94576.json".to_string(),  // Brisbane
    "http://reg.bom.gov.au/fwo/IDS60901/IDS60901.94648.json".to_string(),  // Adelaide
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.94864.json".to_string(),  // Coldstream
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95866.json".to_string(),  // Essendon
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.94876.json".to_string(),  // Frankston
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.94872.json".to_string(),  // Ferny Creek
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.94857.json".to_string(),  // Geelong
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.94865.json".to_string(),  // Laverton
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.94870.json".to_string(),  // Moorabbin
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95867.json".to_string(),  // Scoresby
    "http://reg.bom.gov.au/fwo/IDV60901/IDV60901.95874.json".to_string(),  // Viewbank
    ];
    
    //gloo_console::log!(format!("Location: {:?}, and the url is: {:?}", location, urls[location]));
    /*
    
    /*
Hi Jonty,

It sounds like you’re going through a lot right now, and it’s brave of you to open up about it. Moving to a new city, adjusting to a breakup, questioning your career path, and dealing with a panic attack—these are all huge, life-altering challenges, so it’s no wonder you feel overwhelmed.

Let’s break this down step by step:

1. Your Move to Melbourne
It's normal to feel out of place after moving somewhere new. Forming deep connections takes time, especially in a new environment where you’re still finding your footing. It’s great that you’re considering the possibility of things improving, but it’s also okay to question whether staying is the right choice. Instead of focusing on "what if it doesn’t get better," think about small, actionable steps you can take to build connections: joining local clubs, taking classes (like Latin or Greek—languages you’ve been studying), or exploring shared-interest groups.

That said, if you truly feel Melbourne isn’t right for you after giving it a fair try, returning to Adelaide is always an option. Choosing to prioritize your well-being over "sticking it out" isn’t failure—it’s self-awareness.

2. The Breakup
Breakups can be devastating, especially when you’re already in a challenging place. It's good to hear you're seeking professional help—having someone to guide you through the emotions can make a big difference. Allow yourself the space to grieve and heal without judgment. You’re allowed to feel down and to take things one day at a time.

At the same time, try to focus on what brings you comfort and stability. Whether that’s routine, hobbies, or spending time with friends (even virtually if they’re in Adelaide), leaning on your support system can help.

3. Your Career Path
Your dissatisfaction with your current job is valid, even if the workplace culture is positive. It’s clear that you’re considering your happiness and long-term fulfillment, which is great. Exploring an electrician apprenticeship is a proactive move—it gives you a chance to test the waters without committing to a full change just yet. The trial on Monday could be an eye-opener.

Questions to reflect on:

What aspects of software development do you dislike, and do you think those issues would persist in any tech role?
What excites you about becoming an electrician? Is it the hands-on work, the career change, or something else?
Are there opportunities in software development that align more closely with your interests or values?
If the trial goes well, and you’re passionate about the career change, pursuing the apprenticeship might be worth it. But you don’t have to decide everything immediately.

4. The "What Ifs"
The "what if" spiral is exhausting, isn’t it? Unfortunately, it’s impossible to predict the outcome of every choice. What’s important is identifying what matters most to you in this moment: Is it stability, happiness, growth, or simply peace of mind?

Instead of framing your choices as all-or-nothing (“stick it out or go home,” “good job or happiness”), try to focus on manageable next steps. For example:

Try the electrician trial and see how it feels.
Talk to your therapist about specific tools for handling these major decisions.
Consider setting a timeline to reassess Melbourne—e.g., “I’ll give it six more months and revisit my feelings.”
Reach out to friends and family in Adelaide to talk through your fears of moving back.
5. Your Panic Attack
Panic attacks can feel terrifying, but they’re your body’s way of responding to extreme stress. If they happen again, remind yourself that they’re temporary and won’t hurt you. Some helpful techniques include deep breathing (inhale for 4 counts, hold for 4, exhale for 6), grounding exercises (e.g., name 5 things you see, 4 you can touch, etc.), and practicing mindfulness. Your therapist can guide you on more strategies, too.

6. Closing Thoughts
Jonty, it’s clear that you’re self-aware, thoughtful, and actively working through these challenges. There’s no “wrong” choice here, only choices that align more or less with your values and goals. You don’t have to have it all figured out at once, and it’s okay to feel uncertain.

Lean on your support systems—friends, family, professionals—and trust that you’ll navigate this one step at a time. If you’d like to talk more or need help thinking things through, I’m here. 😊

*/
     */
    let body = reqwest::get(urls[location].clone())
    .await?
    .json::<Observations>()
    .await?;
    
    Ok(body)
}

#[wasm_bindgen]
#[no_mangle]
pub async extern fn call_prog(opt: i8, location: usize) -> Vec<String>{
    gloo_console::log!(format!("Calling prog"));
    let a = prog(opt, location).await;
    gloo_console::log!(format!("Called it!, {:?}", a));
    a
}

pub async extern fn prog_usize(opt: i8, location: usize) -> Vec<usize> {
    let a = call_prog(opt, location).await;
    let b: Vec<usize> = a
    .iter()
    .filter_map(|s| s.parse::<f64>().ok()) // Parse as f64
    .map(|num| num.round() as usize)      // Round and convert to usize
    .collect();

    gloo_console::log!(format!("Parsed values: {:?}", b.clone()));

    let mut temp: Vec<usize> = Vec::new();

    b
}

pub async extern fn prog_float(opt: i8, location: usize) -> Vec<f64> {
    let a = call_prog(opt, location).await;
    let b: Vec<f64> = a
    .iter()
    .filter_map(|s| s.parse::<f64>().ok()) // Parse as f64
    //.map(|num| num.round() as usize)      // Round and convert to usize
    .collect();

    gloo_console::log!(format!("Parsed values: {:?}", b.clone()));

    // let mut temp: Vec<usize> = Vec::new();

    b
}

pub async extern fn prog_string() -> Vec<String> {
    let a = call_prog(2, 0).await;
    let b: Vec<String> = a
    .iter()
    .filter_map(|s| s.parse::<String>().ok()) // Parse as f64
    .map(|num| num as String)      // Round and convert to usize
    .collect();

    gloo_console::log!(format!("Parsed values of time: {:?}", a.clone()));

    // let mut temp: Vec<usize> = Vec::new();

    b
}

pub extern fn prog_string_notasync() -> Vec<String>{
    block_on(async {
        // Your async code here
        let a = call_prog(1, 0).await;
        a
    });
    let temps = Vec::new();
    temps
}


pub extern fn prog_notasync() -> Vec<String>{
    block_on(async {
        // Your async code here
        let a = call_prog(1, 0).await;
        a
    });
    let temps = Vec::new();
    temps
}

pub extern fn prog_asusize() -> Vec<usize>{
    let a = prog_notasync();
    //let b: Vec<usize> = Vec::new();
    //let a: Vec<String> = vec!["42".to_string(), "100".to_string(), "7".to_string()];

    let b: Vec<usize> = a
    .iter() // Create an iterator over the vector
    .map(|s| s.parse::<usize>().unwrap()) // Parse each String into a usize
    .collect(); // Collect the results into a new Vec<usize>

    // let b: Vec<usize> = a.parse().unwrap();
    // for i in 0..a.len() {
    //     b.push(a[i].as_usize());
    // }
    b
}

pub async fn prog(opt: i8, location: usize) -> Vec<String> {
    console_error_panic_hook::set_once();

    let mut temps = Vec::new();
    gloo_console::log!(format!("hello, I'm in prog!"));
    match opt{
        1 => match get_data(location).await {
            Ok(data) => {
                for i in 0..data.observations.data.len() {
                    temps.push(data.observations.data[i].air_temp.to_string());
                    //gloo_console::log!(format!("hello, the temps: {:?}", temps.last()));
                }
            },
            Err(err) => {
                //gloo_console::log!(format!("Error, REAL: {:?}", err));
            }
        },
        2 => match get_data(location).await {
            Ok(data) => {
                for i in 0..data.observations.data.len() {
                    temps.push(data.observations.data[i].local_date_time.to_string());
                    //gloo_console::log!(format!("hello: {:?}", temps.last()));
                }
            },
            Err(err) => {
                // gloo_console::log!(format!("Error, REAL: {:?}", err));
            }
        },
        _ => temps.push("error, no valid chosen thing".to_string()),
        
    }
    gloo_console::log!(format!("Error, fake: {:?}", temps));

    temps
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    let app = Model::init().await;
    dominator::append_dom(&dominator::get_id("test"), Model::render(app));
    Ok(())
}

pub async fn process(url_passed: String) -> Result<Observations, Box<dyn StdError>> {
    let url = "https://reg.bom.gov.au/fwo/IDV60901/IDV60901.95936.json";
    gloo_console::log!("test_lvl100");
    let response = reqwest::get(url).await?.json::<Observations>().await?;
    //gloo_console::log!(format!("{:?}", response));
    Ok(response)
    //Ok("asdfasdfasfd")
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(v: String);
}

#[derive(Debug, Deserialize)]
pub struct Observations {
    observations: ObservationsData,
}

#[derive(Debug, Deserialize)]
pub struct ObservationsData {
    data: Vec<Main>,
}

#[derive(Debug, Deserialize)]
struct Main {
    air_temp: f32,
    apparent_t: f32,
    local_date_time: String,
    name: String,
}

use futures_signals::signal::{MutableSignalCloned};

use std::{
    pin::Pin,
    task::{Context, Poll},
};

pub struct Mutable3<A, B, C>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
)
where
    A: Clone,
    B: Clone,
    C: Clone;
impl<A, B, C> Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>) -> Self {
        Mutable3(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
        )
    }
}
impl<A, B, C> Signal for Mutable3<A, B, C>
where
    A: Clone,
    B: Clone,
    C: Clone,
{
    type Item = (A, B, C);
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let mut changed = false;
        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
            )))
        } else if left_done && middle_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}
pub struct Mutable4<A, B, C, D>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
    (MutableSignalCloned<D>, Mutable<D>),
)
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone;
impl<A, B, C, D> Mutable4<A, B, C, D>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>, d: Mutable<D>) -> Self {
        Mutable4(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
            (d.signal_cloned(), d),
        )
    }
}
impl<A, B, C, D> Signal for Mutable4<A, B, C, D>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
{
    type Item = (A, B, C, D);
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let d = Pin::new(&mut self.3 .0).poll_change(cx);
        let mut changed = false;
        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let left_middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_middle_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_done = match d {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
                self.3 .1.get_cloned(),
            )))
        } else if left_done && left_middle_done && right_middle_done && right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}

pub struct Mutable6<A, B, C, D, E, F>(
    (MutableSignalCloned<A>, Mutable<A>),
    (MutableSignalCloned<B>, Mutable<B>),
    (MutableSignalCloned<C>, Mutable<C>),
    (MutableSignalCloned<D>, Mutable<D>),
    (MutableSignalCloned<E>, Mutable<E>),
    (MutableSignalCloned<F>, Mutable<F>),
)
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone;
impl<A, B, C, D, E, F> Mutable6<A, B, C, D, E, F>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
{
    pub fn new(a: Mutable<A>, b: Mutable<B>, c: Mutable<C>, d: Mutable<D>, e: Mutable<E>, f: Mutable<F>) -> Self {
        Mutable6(
            (a.signal_cloned(), a),
            (b.signal_cloned(), b),
            (c.signal_cloned(), c),
            (d.signal_cloned(), d),
            (e.signal_cloned(), e),
            (f.signal_cloned(), f),
        )
    }
}
impl<A, B, C, D, E, F> Signal for Mutable6<A, B, C, D, E, F>
where
    A: Clone,
    B: Clone,
    C: Clone,
    D: Clone,
    E: Clone,
    F: Clone,
{
    type Item = (A, B, C, D, E, F);
    fn poll_change(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        let a = Pin::new(&mut self.0 .0).poll_change(cx);
        let b = Pin::new(&mut self.1 .0).poll_change(cx);
        let c = Pin::new(&mut self.2 .0).poll_change(cx);
        let d = Pin::new(&mut self.3 .0).poll_change(cx);
        let e = Pin::new(&mut self.4 .0).poll_change(cx);
        let f = Pin::new(&mut self.5 .0).poll_change(cx);
        let mut changed = false;
        let left_done = match a {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let left_middle_done = match b {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_middle_done = match c {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_done = match d {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let right_right_done = match e {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        let left_right_done = match f {
            Poll::Ready(None) => true,
            Poll::Ready(_) => {
                changed = true;
                false
            }
            Poll::Pending => false,
        };
        if changed {
            Poll::Ready(Some((
                self.0 .1.get_cloned(),
                self.1 .1.get_cloned(),
                self.2 .1.get_cloned(),
                self.3 .1.get_cloned(),
                self.4 .1.get_cloned(),
                self.5 .1.get_cloned(),
            )))
        } else if left_done && left_middle_done && right_middle_done && right_done && left_right_done && right_right_done {
            Poll::Ready(None)
        } else {
            Poll::Pending
        }
    }
}
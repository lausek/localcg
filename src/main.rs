//#![feature(custom_attribute)]
#![feature(box_patterns)]

extern crate gdk;
extern crate gtk;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate localc;

mod app;
mod widget;

use relm::Widget;

fn main()
{
    if let Err(e) = app::App::run(()) {
        println!("{:?}", e);
    }
}

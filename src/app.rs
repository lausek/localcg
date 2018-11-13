use std::rc::Rc;
use std::cell::RefCell;
use std::borrow::BorrowMut;
use std::ops::DerefMut;

use gtk::prelude::*;
use gtk::{Inhibit, Window, WindowType};

use relm::{Relm, Update, Widget};

use treecalc::{
    parser::parse,
    program::{context::Context, execute_with_ctx},
};

use crate::widget::{cmdline::Cmdline, graph::Graph, history::History};

#[derive(Msg)]
pub enum Msg
{
    Change,
    Redraw,
    Quit,
}

pub struct Model
{
    graph: Graph,
    cmdline: Cmdline,
    history: History,
    context: Rc<RefCell<Context>>,
}

impl Model
{
    fn new() -> Self
    {
        let mut model = Self {
            graph: Graph::new(),
            cmdline: Cmdline::new(),
            history: History::new(),
            context: Rc::new(RefCell::new(Context::default())),
        };
        model.graph.set_ctx(Rc::clone(&model.context));
        model
    }
}

pub struct App
{
    model: Model,
    window: Window,
}

impl App {

    fn update_context(&mut self, cmd: String) {
        use treecalc::program::node::Node::*;
        match parse(cmd) {
            Ok(program) => {
                let ret = {
                    execute_with_ctx(&program, (*self.model.context).borrow_mut().deref_mut())
                };
                self.model.history.push(format!("{:?}", ret));

                // TODO: check if declaration happened
                match program {
                    Mov(box Func(name, _), _) => self.model.graph.add_graph(name),
                    _ => {},
                }

                self.update(Msg::Redraw);
            }
            _ => {}
        }
    }

}

impl Update for App
{
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model
    {
        Model::new()
    }

    fn update(&mut self, event: Msg)
    {
        match event {
            Msg::Change => {
                // FIXME: maybe `take` this?
                let cmd = self.model.cmdline.buffer().get_text();
                self.model.history.push(cmd.clone());
                self.update_context(cmd);
                self.model.cmdline.buffer().set_text("");
            }
            Msg::Redraw => self.model.graph.draw(),
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for App
{
    type Root = Window;

    fn root(&self) -> Self::Root
    {
        self.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self
    {
        let window = Window::new(WindowType::Toplevel);

        let root_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
        let text_pane = gtk::Paned::new(gtk::Orientation::Vertical);

        text_pane.pack1(model.history.widget(), true, true);
        text_pane.pack2(model.cmdline.widget(), false, false);

        root_pane.pack1(&text_pane, false, false);
        root_pane.pack2(model.graph.draw_area(), true, true);

        window.add(&root_pane);
        window.show_all();

        connect!(
            relm,
            model.cmdline.widget(),
            connect_activate(_),
            Msg::Change
        );
        connect!(
            relm,
            window,
            connect_delete_event(_, _),
            return (Some(Msg::Quit), Inhibit(false))
        );

        Self { model, window }
    }
}

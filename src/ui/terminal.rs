// use std::io;
// use crate::ui::render::{render_main_view, render_task_view};
// use crate::model::task::{TaskManager, Task, TaskEntry};
// use std::process::Command;
// use std::path::Path;
// use std::thread;
// use std::time::Duration;
//
// ///////////////////////////////////////////////////////////
// #[derive(Clone, Debug)]
// pub enum SelectedItem {
//     Task(Task),
//     TaskEntry(TaskEntry),
// }
//
// #[derive(Clone, Debug)]
// pub enum UserAction {
//     Select(SelectedItem),
//     Back,
//     Quit,
//     None,
// }
//
// impl UserAction {
//     pub fn all() -> Vec<UserAction> {
//         vec![
//             UserAction::Back,
//             UserAction::Quit,
//             UserAction::None,
//         ]
//     } 
//     pub fn from_index(index: usize) -> Self {
//         UserAction::all()[index].clone()
//     }
// }
//
// /// The idea is to have a "screen stack"
// // pub enum View {
// //     MainView {idx: usize, max_idx: &mut usize},
// //     TaskView {task_id: usize},
// //     EditorView {task_entry_id: usize},
// // }
//
// enum AppState {
//     MainMenu,
//     ViewTask,
//     Editor,
//     Done,
// }
//
// pub struct TerminalSelection {
//     pub idx: usize,  // index of current selection
//     pub len: usize,  // number of selections
//     pub item: Option<SelectedItem>
// }
//
// impl TerminalSelection {
//     pub fn new() -> Self { TerminalSelection {idx: 0, len: 0, item: None} }
//     
//     pub fn incr(&mut self) {
//         self.idx = (self.idx + self.len - 1) % self.len;
//     }
//
//     pub fn decr(&mut self) {
//         self.idx = (self.idx + self.len + 1) % self.len;
//     }
// }
// pub struct TerminalState { 
//     pub db: TaskManager,
//     pub select: TerminalSelection
// }
//
// impl TerminalState {
//     
//     pub fn new(db: TaskManager) -> Self {
//         TerminalState {
//             select: TerminalSelection::new(),
//             db,
//         }
//     }
// }
//
// ///////////////////////////////////////////////////////////
//
// // pub struct App {
// //     pub db: TaskManager,
// //     pub view_stack: Vec<Screen>
// // }
// //
// // impl App {
// //     
// //     pub fn new() -> Self {
// //
// //     }
// //
// // }
// ///////////////////////////////////////////////////////////
//
// pub fn start(db: TaskManager) -> Result<(), io::Error> {
//     
//     let mut app_state = AppState::MainMenu;
//     // let mut term_state = TerminalState::new(db);
//     // let mut screens = Vec::new();
//
//     // screens.push(Screen::MainView {idx: 0});
//     
//     // loop { 
//     //     app_state = match app_state { 
//     //         AppState::MainMenu => run_view_main(&mut term_state)?, 
//     //         AppState::ViewTask => run_view_task(&mut term_state)?, 
//     //         AppState::Editor => run_editor(&mut term_state)?, 
//     //         AppState::Done => break,
//     //         _ => break,
//     //     }
//     // }
//     Ok(())
// }
//
// ///////////////////////////////////////////////////////////
//
// fn run_view_main(state: &mut TerminalState) -> Result<AppState, io::Error> {
//     
//     let action = render_main_view(state);
//     match action {
//         UserAction::Select(t) => {
//             state.select.item = Some(t); 
//             Ok(AppState::ViewTask)
//         },
//         UserAction::Quit => Ok(AppState::Done),
//         UserAction::Back => Ok(AppState::Done),
//         _ => Ok(AppState::MainMenu)
//     } 
// }
//
// fn run_view_task(state: &mut TerminalState) -> Result<AppState, io::Error> {
//     
//     match render_task_view(state) {
//         UserAction::Select(t) => {
//             println!("Selected!: {:?}", t);
//             state.select.item = Some(t);
//             Ok(AppState::Editor)
//         },
//         UserAction::Quit => Ok(AppState::Done),
//         UserAction::Back => Ok(AppState::MainMenu),
//         _ => Ok(AppState::ViewTask) // stay
//     }
// }
//
// fn run_editor(state: &mut TerminalState) -> Result<AppState, io::Error> {
//     
//     println!("Run Editor!: {:?}", state.select.item.as_ref().unwrap());
//     let status = Command::new("nvim")
//         .arg("/home/justin/school/229/common.s")
//         .status()
//         .expect("Failed to open editor");
//     
//     std::process::exit(1); 
//     Ok(AppState::ViewTask)
//     
// }
//

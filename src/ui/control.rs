/// control.rs

use std::fmt;
use std::{collections::HashMap, io};
use crossterm::event::{self, Event, KeyCode, KeyEvent};
use crate::ui::state::{TaskViewState, MainViewState, EntryViewState};
use crate::ui::view::View;
use crate::model::task::Task;
use crate::model::task_entry::TaskEntry;
use crate::model::task_manager::TaskManager;
use crate::ui::view::Transition;

///////////////////////////////////////////////////////////

type KeyHandler = HashMap<KeyCode, Transition>;

#[derive(Clone, Debug)]
pub enum UserAction {
    Select,
    Back,
    Quit,
}

impl UserAction {
    pub fn all() -> Vec<UserAction> {
        vec![
            UserAction::Select,
            UserAction::Back,
            UserAction::Quit,
        ]
    } 

    pub fn from_index(index: usize) -> Self {
        UserAction::all()[index].clone()
    }    
}

impl fmt::Display for UserAction {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            UserAction::Select => "Select (s)",  
            UserAction::Back => "Back (b)",  
            UserAction::Quit => "Quit (q)"  
        };
        write!(fmt, "{}", text)
    }
} 

//////////////////////////////////////////////////////////

pub trait Controlable { 
    /// keyboard handler
    fn control(&mut self) -> Transition;    
}

/// TODO: Factor out common default key handling

impl Controlable for MainViewState {
    
    fn control(&mut self) -> Transition {
        
        match event::read().unwrap() {  

            Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
                => Transition::Quit, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
                => {
                    let item = self.items[self.selector.idx].clone();
                    Transition::Push(View::TaskView(TaskViewState::new(item)))
                } 
            
            Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
                => { 
                    self.selector.decr();
                    Transition::Stay
                }, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
                => { 
                    self.selector.incr();
                    Transition::Stay
                },
           
            Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
                =>  Transition::Stay,
            _ 
                => Transition::Stay,
        } 
    }
}

impl Controlable for TaskViewState {
    
    fn control(&mut self) -> Transition {
        
        match event::read().unwrap() {  

            Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
                => Transition::Quit, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
                => {
                    let item = self.items[self.selector.idx].clone();
                    Transition::Push(View::EntryView(EntryViewState::new(item)))
                } 
            
            Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
                => { 
                    self.selector.decr();
                    Transition::Stay
                }, 
            
            Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
                => { 
                    self.selector.incr();
                    Transition::Stay
                },
           
            Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
                =>  Transition::Pop,
            _ 
                => Transition::Stay,
        } 
    }
}


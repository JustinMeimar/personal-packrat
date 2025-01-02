use std::io;
use std::io::Stdout;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent},
    execute,
    terminal,
};
use tui::{ 
    text::{Span, Spans, Text},
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Terminal,
};
use crate::{model::task::TaskManager, ui::view::{Transition, View}};
use crate::ui::state::{TaskViewState, MainViewState, EntryViewState};
use crate::ui::control::Controlable;
use crate::ui::render_lib::{term_default_layout, term_user_action_list};
use crate::model::task::{Task, TaskEntry};
use super::state::SelectionState;

///////////////////////////////////////////////////////////

pub trait Renderable { 
    
    /// 
    fn render(&mut self) -> io::Result<Transition>;
}

///////////////////////////////////////////////////////////

impl Renderable for EntryViewState {
    fn render(&mut self) -> io::Result<Transition> {
        Ok(Transition::Stay)
    }
}

impl Renderable for MainViewState {

    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 

        loop {
            let widgets = vec![
                term_user_action_list(),
                term_user_task_list(&self.items, &self.selector),
            ];
            
            draw_widgets(&mut terminal, widgets);

            transition = self.control();
            match transition {
                Transition::Stay => continue,
                _ => break
            }
        }
        render_view_teardown(&mut terminal); 
        
        return Ok(transition);
    }
}

impl Renderable for TaskViewState {

    fn render(&mut self) -> io::Result<Transition> {
        
        let mut terminal = render_view_startup()?;
        let mut transition = Transition::Stay; 
        
        loop {
            
            let widgets = vec![
                term_user_action_list(),
                term_user_task_entries_list(&self.items, self.selector.idx)
            ];

            draw_widgets(&mut terminal, widgets);

            transition = self.control();
            match transition {
                Transition::Stay => continue,
                _ => break
            }
        }
        
        render_view_teardown(&mut terminal); 
        return Ok(transition);
    }
}

///////////////////////////////////////////////////////////

fn render_view_startup() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    
    // Flush stdout
    let mut stdout = std::io::stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    // Initialize the terminal
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    crossterm::terminal::enable_raw_mode()?;

    Ok(terminal)
}

fn render_view_teardown(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    execute!(terminal.backend_mut(), crossterm::terminal::LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

#[derive(Clone, Debug)]
pub enum UserAction {
    Select,
    Back,
    Quit,
    None,
}

impl UserAction {
    pub fn all() -> Vec<UserAction> {
        vec![
            UserAction::Back,
            UserAction::Quit,
            UserAction::None,
        ]
    } 
    pub fn from_index(index: usize) -> Self {
        UserAction::all()[index].clone()
    }
}

fn draw_widgets(terminal: &mut Terminal<CrosstermBackend<Stdout>>, widgets: Vec<List<'static>>) {
    
    terminal
        .draw(|f| {
            let chunks = term_default_layout().split(f.size());
            widgets.iter().enumerate().for_each(|(i, w)| {
                f.render_widget(w.clone(), chunks[i]);
            });
        })
        .unwrap();
}

fn term_user_task_list(tasks: &Vec<Task>,selection: &SelectionState) -> List<'static> {
    
    let task_list: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(i, task)| style_list_item(&task.name, selection.idx, i))
        .collect();


    List::new(task_list)
        .block(Block::default().title("Tasks").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}

fn term_user_task_entries_list(tasks: &Vec<TaskEntry>, idx: usize) -> List<'static> {
    
    let task_list: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(i, entry)| style_list_item(&entry.id.to_string(), idx, i))
        .collect();


    List::new(task_list)
        .block(Block::default().title("Tasks").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}

fn style_list_item(
    item_text: &str, // Accept a string slice
    selection_idx: usize,
    map_idx: usize,
) -> ListItem<'static> {
    let style = if selection_idx == map_idx {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default()
    };
    ListItem::new(Spans::from(Span::styled(item_text.to_string(), style)))
}

fn control_handler(tasks: &Vec<Task>, selector: &mut SelectionState) -> Transition {
    
    match event::read().unwrap() {  
        
        Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
            => Transition::Quit, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
            => {

                let task = TaskManager::instance()
                    .lock()
                    .unwrap()
                    .get_tasks()[selector.idx]
                    .clone();

                Transition::Push(View::TaskView(TaskViewState::new(task)))
            } 
        
        Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
            => { 
                selector.decr();
                Transition::Stay
            }, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
            => { 
                selector.incr();
                Transition::Stay
            },
       
        Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
            =>  Transition::Stay,
        _ 
            => Transition::Stay,
    }
}

fn control_handler_entries(tasks: &Vec<TaskEntry>, selector: &mut SelectionState) -> Transition {
    
    match event::read().unwrap() {  
        
        Event::Key(KeyEvent { code: KeyCode::Char('q') | KeyCode::Char('e'), .. }) 
            => Transition::Quit, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('s') | KeyCode::Enter, .. })
            => {

                Transition::Stay
            } 
        
        Event::Key(KeyEvent { code: KeyCode::Char('j') | KeyCode::Down, .. })
            => { 
                selector.decr();
                Transition::Stay
            }, 
        
        Event::Key(KeyEvent { code: KeyCode::Char('k') | KeyCode::Up, .. })
            => { 
                selector.incr();
                Transition::Stay
            },
       
        Event::Key(KeyEvent { code: KeyCode::Char('b'), .. })
            =>  Transition::Pop,
        _ 
            => Transition::Stay,
    }
}

// fn draw_main_view(
//     terminal: &mut Terminal<CrosstermBackend<Stdout>>,
//     // state: &mut TerminalState,
// ) {
//     let widgets = vec![
//         term_user_action_list(),
//         // term_user_task_list(state),
//     ];
//     terminal
//         .draw(|f| {
//             let chunks = term_default_layout().split(f.size());
//             widgets.iter().enumerate().for_each(|(i, w)| {
//                 f.render_widget(w.clone(), chunks[i]);
//             });
//         })
//         .unwrap();
// }
//


use argon2::{
    Argon2, Params, Version,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use education_platform_auth::{User, UserError};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};
use std::io;

/// Application state for the terminal UI.
#[derive(Debug)]
struct App {
    screen: Screen,
    menu_state: ListState,
    form: RegistrationForm,
    message: Option<Message>,
    should_quit: bool,
}

/// Represents the current screen in the application.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Screen {
    Menu,
    RegisterUser,
}

/// Form data for user registration.
#[derive(Debug, Default)]
struct RegistrationForm {
    first_name: String,
    middle_name: String,
    last_name: String,
    second_last_name: String,
    document: String,
    email: String,
    password: String,
    active_field: FormField,
}

/// Fields in the registration form.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum FormField {
    #[default]
    FirstName,
    MiddleName,
    LastName,
    SecondLastName,
    Document,
    Email,
    Password,
}

/// Message to display to the user.
#[derive(Debug, Clone)]
struct Message {
    text: String,
    is_error: bool,
}

/// Menu options available in the main menu.
const MENU_OPTIONS: &[&str] = &["Register User", "Exit"];

impl App {
    fn new() -> Self {
        let mut menu_state = ListState::default();
        menu_state.select(Some(0));

        Self {
            screen: Screen::Menu,
            menu_state,
            form: RegistrationForm::default(),
            message: None,
            should_quit: false,
        }
    }

    fn run(mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        while !self.should_quit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        match self.screen {
            Screen::Menu => self.draw_menu(frame, area),
            Screen::RegisterUser => self.draw_registration_form(frame, area),
        }

        if let Some(ref msg) = self.message {
            self.draw_message_popup(frame, area, msg.clone());
        }
    }

    fn draw_menu(&mut self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Education Platform ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan));

        let items: Vec<ListItem> = MENU_OPTIONS
            .iter()
            .map(|&option| ListItem::new(option))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::DarkGray),
            )
            .highlight_symbol(">> ");

        frame.render_stateful_widget(list, area, &mut self.menu_state);

        let help = Paragraph::new("↑↓: Navigate | Enter: Select | q: Quit")
            .style(Style::default().fg(Color::DarkGray));
        let help_area = Rect::new(area.x + 1, area.bottom() - 1, area.width - 2, 1);
        frame.render_widget(help, help_area);
    }

    fn draw_registration_form(&self, frame: &mut Frame, area: Rect) {
        let block = Block::default()
            .title(" Register User ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green));

        frame.render_widget(block, area);

        let inner_area = Rect::new(area.x + 2, area.y + 2, area.width - 4, area.height - 4);

        let chunks = Layout::vertical([
            Constraint::Length(3), // First Name
            Constraint::Length(3), // Middle Name
            Constraint::Length(3), // Last Name
            Constraint::Length(3), // Second Last Name
            Constraint::Length(3), // Document
            Constraint::Length(3), // Email
            Constraint::Length(3), // Password
            Constraint::Min(1),    // Help text
        ])
        .split(inner_area);

        self.draw_input_field(
            frame,
            chunks[0],
            "First Name *",
            &self.form.first_name,
            FormField::FirstName,
        );
        self.draw_input_field(
            frame,
            chunks[1],
            "Middle Name",
            &self.form.middle_name,
            FormField::MiddleName,
        );
        self.draw_input_field(
            frame,
            chunks[2],
            "Last Name *",
            &self.form.last_name,
            FormField::LastName,
        );
        self.draw_input_field(
            frame,
            chunks[3],
            "Second Last Name",
            &self.form.second_last_name,
            FormField::SecondLastName,
        );
        self.draw_input_field(
            frame,
            chunks[4],
            "Document (DNI) *",
            &self.form.document,
            FormField::Document,
        );
        self.draw_input_field(frame, chunks[5], "Email *", &self.form.email, FormField::Email);
        self.draw_password_field(
            frame,
            chunks[6],
            "Password",
            &self.form.password,
            FormField::Password,
        );

        let help =
            Paragraph::new("Tab: Next Field | Shift+Tab: Previous | Enter: Submit | Esc: Back")
                .style(Style::default().fg(Color::DarkGray));
        frame.render_widget(help, chunks[7]);
    }

    fn draw_input_field(
        &self,
        frame: &mut Frame,
        area: Rect,
        label: &str,
        value: &str,
        field: FormField,
    ) {
        let is_active = self.form.active_field == field;
        let border_color = if is_active {
            Color::Yellow
        } else {
            Color::Gray
        };

        let block = Block::default()
            .title(format!(" {} ", label))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let display_value = if is_active {
            format!("{}▌", value)
        } else {
            value.to_string()
        };

        let paragraph = Paragraph::new(display_value).block(block);
        frame.render_widget(paragraph, area);
    }

    fn draw_password_field(
        &self,
        frame: &mut Frame,
        area: Rect,
        label: &str,
        value: &str,
        field: FormField,
    ) {
        let is_active = self.form.active_field == field;
        let border_color = if is_active {
            Color::Yellow
        } else {
            Color::Gray
        };

        let block = Block::default()
            .title(format!(" {} ", label))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let masked: String = "*".repeat(value.len());
        let display_value = if is_active {
            format!("{}▌", masked)
        } else {
            masked
        };

        let paragraph = Paragraph::new(display_value).block(block);
        frame.render_widget(paragraph, area);
    }

    fn draw_message_popup(&self, frame: &mut Frame, area: Rect, message: Message) {
        let popup_width = 50.min(area.width - 4);
        let popup_height = 5;

        let popup_area = Rect::new(
            (area.width - popup_width) / 2,
            (area.height - popup_height) / 2,
            popup_width,
            popup_height,
        );

        frame.render_widget(Clear, popup_area);

        let (title, border_color) = if message.is_error {
            (" Error ", Color::Red)
        } else {
            (" Success ", Color::Green)
        };

        let block = Block::default()
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        let text = vec![
            Line::from(Span::raw(&message.text)),
            Line::from(Span::styled(
                "Press any key to continue",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(text).block(block).centered();
        frame.render_widget(paragraph, popup_area);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                return Ok(());
            }

            if self.message.is_some() {
                self.message = None;
                return Ok(());
            }

            match self.screen {
                Screen::Menu => self.handle_menu_input(key.code),
                Screen::RegisterUser => self.handle_form_input(key.code),
            }
        }
        Ok(())
    }

    fn handle_menu_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => {
                let i = self.menu_state.selected().unwrap_or(0);
                let new_index = if i == 0 {
                    MENU_OPTIONS.len() - 1
                } else {
                    i - 1
                };
                self.menu_state.select(Some(new_index));
            }
            KeyCode::Down => {
                let i = self.menu_state.selected().unwrap_or(0);
                let new_index = (i + 1) % MENU_OPTIONS.len();
                self.menu_state.select(Some(new_index));
            }
            KeyCode::Enter => match self.menu_state.selected() {
                Some(0) => self.screen = Screen::RegisterUser,
                _ => self.should_quit = true,
            },
            KeyCode::Char('q') => self.should_quit = true,
            _ => {}
        }
    }

    fn handle_form_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Esc => {
                self.form = RegistrationForm::default();
                self.screen = Screen::Menu;
            }
            KeyCode::Tab => self.form.next_field(),
            KeyCode::BackTab => self.form.previous_field(),
            KeyCode::Enter => self.submit_registration(),
            KeyCode::Backspace => self.form.delete_char(),
            KeyCode::Char(c) => self.form.insert_char(c),
            _ => {}
        }
    }

    fn submit_registration(&mut self) {
        let middle_name = if self.form.middle_name.trim().is_empty() {
            None
        } else {
            Some(self.form.middle_name.clone())
        };

        let second_last_name = if self.form.second_last_name.trim().is_empty() {
            None
        } else {
            Some(self.form.second_last_name.clone())
        };

        let hashed_password = if self.form.password.trim().is_empty() {
            None
        } else {
            match hash_password_argon2id(&self.form.password) {
                Ok(hash) => Some(hash),
                Err(e) => {
                    self.message = Some(Message {
                        text: format!("Password hashing failed: {}", e),
                        is_error: true,
                    });
                    return;
                }
            }
        };

        match User::new(
            self.form.first_name.clone(),
            middle_name,
            self.form.last_name.clone(),
            second_last_name,
            self.form.document.clone(),
            self.form.email.clone(),
            hashed_password,
        ) {
            Ok(user) => {
                self.message = Some(Message {
                    text: format!("User '{}' registered!", user.name().full_name()),
                    is_error: false,
                });
                self.form = RegistrationForm::default();
                self.screen = Screen::Menu;
            }
            Err(e) => {
                self.message = Some(Message {
                    text: format_user_error(&e),
                    is_error: true,
                });
            }
        }
    }
}

/// Hashes a password using Argon2id algorithm.
///
/// Uses recommended parameters: memory=65536 KB, iterations=3, parallelism=4.
fn hash_password_argon2id(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let params = Params::new(65536, 3, 4, None)?;
    let argon2 = Argon2::new(argon2::Algorithm::Argon2id, Version::V0x13, params);
    let hash = argon2.hash_password(password.as_bytes(), &salt)?;
    Ok(hash.to_string())
}

impl RegistrationForm {
    fn next_field(&mut self) {
        self.active_field = match self.active_field {
            FormField::FirstName => FormField::MiddleName,
            FormField::MiddleName => FormField::LastName,
            FormField::LastName => FormField::SecondLastName,
            FormField::SecondLastName => FormField::Document,
            FormField::Document => FormField::Email,
            FormField::Email => FormField::Password,
            FormField::Password => FormField::FirstName,
        };
    }

    fn previous_field(&mut self) {
        self.active_field = match self.active_field {
            FormField::FirstName => FormField::Password,
            FormField::MiddleName => FormField::FirstName,
            FormField::LastName => FormField::MiddleName,
            FormField::SecondLastName => FormField::LastName,
            FormField::Document => FormField::SecondLastName,
            FormField::Email => FormField::Document,
            FormField::Password => FormField::Email,
        };
    }

    fn insert_char(&mut self, c: char) {
        let field = self.get_active_field_mut();
        field.push(c);
    }

    fn delete_char(&mut self) {
        let field = self.get_active_field_mut();
        field.pop();
    }

    fn get_active_field_mut(&mut self) -> &mut String {
        match self.active_field {
            FormField::FirstName => &mut self.first_name,
            FormField::MiddleName => &mut self.middle_name,
            FormField::LastName => &mut self.last_name,
            FormField::SecondLastName => &mut self.second_last_name,
            FormField::Document => &mut self.document,
            FormField::Email => &mut self.email,
            FormField::Password => &mut self.password,
        }
    }
}

fn format_user_error(error: &UserError) -> String {
    match error {
        UserError::IdError(e) => format!("ID error: {}", e),
        UserError::PersonNameError(e) => format!("Name error: {}", e),
        UserError::DniError(e) => format!("Document error: {}", e),
        UserError::EmailError(e) => format!("Email error: {}", e),
        UserError::HashedPasswordError(e) => format!("Password error: {}", e),
        _ => format!("Unknown error: {}", error),
    }
}

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}

use std::{collections::HashMap, io::Result};

/// Contains all the persistent "data" that will be passed to any function that needs
/// to know the current state of the application
pub struct App {
    /// The json key currently being edited
    pub key_input: String,
    /// The json value currently being edited
    pub value_input: String,
    /// The representation of our key and value pairs
    pub pairs: HashMap<String, String>,
    /// The current screen that the user is looking at, and will later determine what is being rendered
    pub current_screen: CurrentScreen,
    /// The optional state containing which of the key or value pair the user is editing.
    /// It is an option, because when the user is not directly editing a key-value pair, this will be None.
    pub currently_editing: Option<CurrentlyEditing>,
}

/// The app has three screens:
pub enum CurrentScreen {
    /// The main summary screen showing all past key-value pairs entered
    Main,
    /// The screen shown when the user wishes to create a new key-value pair
    Editing,
    /// Displays a prompt asking if the user wants to output the key-value pairs they have entered
    Exiting,
}

/// Keeps track of which field the user is currently editing
pub enum CurrentlyEditing {
    Key,
    Value,
}

impl App {
    /// Initialize the application state with the defaults
    pub fn new() -> App {
        App {
            key_input: String::new(),
            value_input: String::new(),
            pairs: HashMap::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: None,
        }
    }

    /// This function will be called when the user saves a key-value pair in the editor.
    /// It adds the two stored variables to the key-value pairs `HashMap``, and resets the status of all of the editing variables.
    pub fn save_key_value(&mut self) {
        self.pairs
            .insert(self.key_input.clone(), self.value_input.clone());
        self.key_input = String::new();
        self.value_input = String::new();
        self.currently_editing = None;
    }

    /// Responsible for switching the focus between key and value if we are currently editing
    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            }
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }

    /// Dump the HashMap
    pub(crate) fn print_output(&self) -> Result<()> {
        println!("{:#?}", self.pairs);
        Ok(())
    }
}

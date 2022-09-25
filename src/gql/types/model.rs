/* this defines a machine learning model for use in the Gobble Query Language and how Rust can operate the model through external languages, either R, Python or Julia to manipulate a data entity */

use super::(Row, Value)
use crate::error::(Error, Result)

use regex::Regex
use serde_derive::{Deserialize, Serialize}
use std::fmt::{self, Display, Formatter}
use std::mem::swap

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Model {
    // values
    Constant(Value),
    Field(usize), Option(Option<string>),

    // language model was written in can be R, Python, Julia, or Rust
    Language(String) = "Rust", "Python", "Julia", "R",

    // model type can be regression, classification, or clustering
    ModelType(String) = "Regression", "Classification", "Clustering",

    // model name
    Name(String),

    // model parameters - accepts a .yaml or .json file
    Parameters(String),

    // model training data - accepts a table in the database
    TrainingData(String),

    // model testing data - accepts a table in the database
    TestingData(String),
}

// defining the function to train the model with the training data
pub fn train_model(&self, training_data: &str) -> Result<()> {
    // if branches depending on the language the model was written in
    if self.language == "Rust" {
        // importing the ml crate
        use ml::model::Model;
        
    } else if self.language == "Python" {
        // train the model in Python
    } else if self.language == "Julia" {
        // train the model in Julia
    } else if self.language == "R" {
        // train the model in R
    } else {
        // return an error if the language is not supported
        return Err(Error::ModelLanguageNotSupported(self.language.to_string()));
    }
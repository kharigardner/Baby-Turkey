// this script is the base of the gobble query language, a query language for the babyturkey database

use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::LinkedList;
use std::collections::BinaryHeap;
use std::collections::hash_map::Entry;

// defining the parsers for the gobble query language
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{alpha1, alphanumeric1, char, digit1, multispace0, multispace1},
    combinator::{all_consuming, map, map_res, opt, recognize},
    error::{context, convert_error, ErrorKind, ParseError},
    multi::{many0, many1, separated_list0, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

// defining the gobble query language
#[derive(Debug, PartialEq)]
pub enum GobbleQuery {
    Select(SelectQuery),
    Insert(InsertQuery),
    Update(UpdateQuery),
    Delete(DeleteQuery),
    Create(CreateQuery),
    Drop(DropQuery),
    Alter(AlterQuery),
    Show(ShowQuery),
    Use(UseQuery),
    Explain(ExplainQuery),
    Describe(DescribeQuery),
    Help(HelpQuery),
    Quit(QuitQuery),
    Exit(ExitQuery),
}

// defining the select query
#[derive(Debug, PartialEq)]
pub struct SelectQuery {
    pub distinct: bool,
    pub columns: Vec<Column>,
    pub from: Vec<Table>,
    pub where_clause: Option<WhereClause>,
    pub group_by: Option<GroupByClause>,
    pub order_by: Option<OrderByClause>,
    pub limit: Option<LimitClause>,
}

// defining the insert query
#[derive(Debug, PartialEq)]
pub struct InsertQuery {
    pub table: Table,
    pub columns: Option<Vec<Column>>,
    pub values: Vec<Values>,
}

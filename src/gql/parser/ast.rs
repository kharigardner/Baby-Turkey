use super::super::types::DataType;
use crate::error::result;

use std::collections::HashMap;
use std::mem::swap;

// GQL Statements
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
#[allow(clippy::large_enum_variant)]

// enum containing all possible statements, ddl, dml, dcl, and tcl statements
pub enum Statement {
    //  SQL-Like Statements
    // Transaction Statements - go, commit, timetravel, rollback, release, explain
    Go {
        read_only: bool,
        db_version: Option<String>,
    },
    Commit,
    TimeTravel {
        version: Option<String>,
        savepoint: Option<String>,
    },
    Rollback {
        version: Option<String>,
        savepoint: Option<String>,
    },
    Release {
        savepoint: String,
    },
    Explain {
        statement: Box<Statement>,
    },
    // Data Definition Statements - create, alter, drop, truncate, rename
    Create {
        object_type: ObjectType,
        if_not_exists: bool,
        name: String,
        columns: Vec<ColumnDef>,
        constraints: Vec<Constraint>,
        options: Vec<TableOption>,
        with: Option<With>,
    },
    Alter {
        object_type: ObjectType,
        name: String,
        operation: AlterOperation,
    },
    Drop {
        object_type: ObjectType,
        if_exists: bool,
        name: String,
        cascade: bool,
    },
    Truncate {
        table_name: String,
    },
    Rename {
        object_type: ObjectType,
        name: String,
        new_name: String,
    },
    // Data Manipulation Statements - insert, update, delete, select
    Insert {
        table_name: String,
        columns: Vec<String>,
        values: Vec<Vec<Expr>>,
        select: Option<Box<Statement>>,
        on_conflict: Option<OnConflict>,
    },
    Update {
        table_name: String,
        assignments: Vec<Assignment>,
        selection: Option<Box<Expr>>,
    },
    Delete {
        table_name: String,
        selection: Option<Box<Expr>>,
    },
    Select {
        distinct: bool,
        projection: Vec<SelectItem>,
        from: Vec<FromItem>,
        selection: Option<Box<Expr>>,
        group_by: Vec<Expr>,
        having: Option<Box<Expr>>,
        order_by: Vec<OrderByExpr>,
        limit: Option<Box<Expr>>,
        offset: Option<Box<Expr>>,
    },
    // Data Control Statements - grant, revoke
    Grant {
        privilege: Privilege,
        object_type: ObjectType,
        on_name: String,
        to_name: String,
    },
    Revoke {
        privilege: Privilege,
        object_type: ObjectType,
        on_name: String,
        from_name: String,
    },
    // Session Control Statements - set, show, start, end
    Set {
        variable: String,
        value: Expr,
    },
    Show {
        variable: String,
    },
    Start {
        transaction: bool,
    },
    End,
    // Graph Statements
    Traverse {
        start: Vec<Expr>,
        edge: Vec<Expr>,
        end: Vec<Expr>,
        where_clause: Option<Box<Expr>>,
        order_by: Vec<OrderByExpr>,
        limit: Option<Box<Expr>>,
        offset: Option<Box<Expr>>,
    },
    CreateEdge {
        from: Vec<Expr>,
        to: Vec<Expr>,
        edge: Vec<Expr>,
        properties: Vec<Assignment>,
    },
    DeleteEdge {
        from: Vec<Expr>,
        to: Vec<Expr>,
        edge: Vec<Expr>,
    },
    // JSON Statements
    ParseStruct {
        json: Expr,
        schema: Expr,
    },
    ParseArray {
        json: Expr,
        schema: Expr,
    }
    // Machine Learning Statements
    CreateModel {
        name: String,
        model_type: ModelType,
        options: Vec<TableOption>,
    },
    TrainModel {
        name: String,
        data: Expr,
        options: Vec<TableOption>,
    },
    Predict {
        model: Expr,
        data: Expr,
        options: Vec<TableOption>,
    },
    Evaluate {
        model: Expr,
        data: Expr,
        options: Vec<TableOption>,
    },
    // Other Statements
    Cast {
        expr: Expr,
        data_type: DataType,
    },
    Call {
        name: String,
        args: Vec<Expr>,
    },
    // Visualization Statements
    CreateViz {
        name: String,
        viz_type: VizType,
        options: Vec<TableOption>,
    },
    ShowViz {
        name: String,
        options: Vec<TableOption>,
    },
    AlterViz {
        name: String,
        options: Vec<TableOption>,
    },
    DropViz {
        name: String,
    },
    // Statement to attach vizualization to a table
    AttachViz {
        table_name: String,
        column_name: Option<String>,
        viz_name: String,
    },
    // Statement to detach vizualization from a table
    DetachViz {
        table_name: String,
        viz_name: String,
    }
}
#TODO: add items defined below into the statement enums above

/// items used in statements

// FROM item
#[derive(Debug, Clone, PartialEq)]
pub enum FromItem {
    Table {
        name: String,
        alias: Option<String>,
    },
    Join {
        left: Box<FromItem>,
        right: Box<FromItem>,
        join_operator: JoinOperator,
        join_constraint: Option<JoinConstraint>,
    },
    Subquery {
        query: Box<Statement>,
        alias: String,
    },
}

// JOIN operator
#[derive(Debug, Clone, PartialEq)]
pub enum JoinOperator {
    Inner,
    LeftOuter,
    RightOuter,
    FullOuter,
}

// JOIN constraint
#[derive(Debug, Clone, PartialEq)]
pub enum JoinConstraint {
    On(Expr),
    Using(Vec<String>),
}

// Column definition
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub data_type: DataType,
    pub nullable: bool,
    pub default: Option<Expr>,
    pub options: Vec<ColumnOption>,
    pub constraint_type: Option<ConstraintType>,
    pub constraint_enforced: bool,
}

// ConstraintType
#[derive(Debug, Clone, PartialEq)]
pub enum ConstraintType {
    PrimaryKey,
    Unique,
    ForeignKey,
    Check,
}

// Sort Item
#[derive(Debug, Clone, PartialEq)]
pub struct OrderByExpr {
    pub expr: Expr,
    pub asc: bool,
}

// Expressions
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOperator,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOperator,
        expr: Box<Expr>,
    },
    Function {
        name: String,
        args: Vec<Expr>,
    },
    Cast {
        expr: Box<Expr>,
        data_type: DataType,
    },
    Case {
        operand: Option<Box<Expr>>,
        conditions: Vec<Expr>,
        results: Vec<Expr>,
        else_result: Option<Box<Expr>>,
    },
    Exists {
        subquery: Box<Statement>,
    },
    Select {
        subquery: Box<Statement>,
    },
    Column {
        name: String,
    },
    Literal {
        value: Value,
    },
    Wildcard,
}

// Binary Operator
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulus,
    Equals,
    NotEquals,
    LessThan,
    LessThanOrEquals,
    GreaterThan,
    GreaterThanOrEquals,
    And,
    Or,
    Like,
    NotLike,
    In,
    NotIn,
    Is,
    IsNot,
}

// Unary Operator
#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    Not,
    Minus,
}

// Assignment
#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub id: String,
    pub value: Expr,
}

// Literals
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(String),
    String(String),
    Boolean(bool),
    Null,
}


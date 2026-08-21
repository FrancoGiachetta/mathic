/**
 * @file tree-sitter for Mathic's grammar
 * @author Franco Giachetta <francogiachetta27@gmail.com>
 * @license Apache
 */

/// <reference types="tree-sitter-cli/dsl" />
// @ts-check

const PREC = {
  ASSIGNMENT: -1,
  LOGICAL_OR: 0,
  LOGICAL_AND: 1,
  EQUALITY: 2,
  INEQUALITY: 3,
  ADDITION: 4,
  MULTIPLICATION: 5,
  UNARY: 6,
  CALL: 7,
};

const numericTypes = [
  "u8",
  "i8",
  "u16",
  "i16",
  "u32",
  "i32",
  "u64",
  "i64",
  "u128",
  "i128",
  "isz",
  "usz",
  "f32",
  "f64",
];

const nativeTypes = numericTypes.concat(["bool", "str", "char", "expr"]);

export default grammar({
  name: "mathic",

  conflicts: ($) => [
    [$.assignment, $.primary],
    [$.expression, $.expression_no_assign],
  ],

  extras: ($) => [/\s|\\\r?\n/, $.comment],

  inline: ($) => [
    $._field_identifier,
    $.bracket_type,
    $.path,
    $._type_identifier,
  ],

  supertypes: ($) => [
    $.top_decl,
    $.declaration,
    $.stmt,
    $.expression,
    $.expression_no_assign,
    $.non_binary_expression,
  ],

  word: ($) => $.IDENT,

  rules: {
    /// ================================================================
    ///  Program
    /// ================================================================

    program: ($) => repeat($.top_decl),

    top_decl: ($) => choice($.func_decl, $.struct_decl, $.imports_decls),

    imports_decls: ($) => seq("imp", $.import_path, ";"),

    func_decl: ($) =>
      seq(
        "df",
        field("name", $.IDENT),
        "(",
        field("params", optional($.param_list)),
        ")",
        field("return_type", optional($._type)),
        field("body", $.block),
      ),

    struct_decl: ($) =>
      seq(
        "struct",
        field("name", $.IDENT),
        "{",
        field("fields", optional($.struct_fields)),
        "}",
      ),

    /// ================================================================
    ///  Declarations
    /// ================================================================

    declaration: ($) =>
      choice($.func_decl, $.struct_decl, $.var_decl, $.sym_decl),

    var_decl: ($) =>
      seq(
        "let",
        field("name", $.IDENT),
        ":",
        field("type", $._type),
        "=",
        field("value", $.expression),
        ";",
      ),

    sym_decl: ($) =>
      seq(
        "sym",
        field("name", $.IDENT),
        ":",
        field("type", $.native_type),
        ";",
      ),

    /// ================================================================
    ///  Statements
    /// ================================================================

    stmt: ($) =>
      choice(
        $.declaration,
        $.for_stmt,
        $.while_stmt,
        $.if_stmt,
        $.return_stmt,
        $.expr_stmt,
        $.block,
      ),

    for_stmt: ($) =>
      seq(
        "for",
        $.IDENT,
        "in",
        $.expression_no_assign,
        choice(
          field("loop_body", $.block),
          seq("..", $.expression_no_assign, field("loop_body", $.block)),
        ),
      ),

    while_stmt: ($) =>
      seq(
        "while",
        field("condition", $.expression_no_assign),
        field("then_body", $.block),
      ),

    if_stmt: ($) =>
      seq(
        "if",
        field("condition", $.expression_no_assign),
        field("then_body", $.block),
        optional(seq("else", field("else_body", $.block))),
      ),

    return_stmt: ($) => seq("return", field("value", $.expression), ";"),

    expr_stmt: ($) => seq($.expression, ";"),

    block: ($) => seq("{", repeat($.stmt), "}"),

    /// ================================================================
    ///  Expressions
    /// ================================================================

    expression: ($) =>
      choice(
        $.assignment_expression,
        $.binary_expression,
        $.non_binary_expression,
      ),

    binary_expression: ($) => {
      const prec_table = [
        ["or", PREC.LOGICAL_OR],
        ["and", PREC.LOGICAL_AND],
        ["==", PREC.EQUALITY],
        ["!=", PREC.EQUALITY],
        ["<", PREC.INEQUALITY],
        [">", PREC.INEQUALITY],
        ["<=", PREC.INEQUALITY],
        [">=", PREC.INEQUALITY],
        ["+", PREC.ADDITION],
        ["-", PREC.ADDITION],
        ["*", PREC.MULTIPLICATION],
        ["/", PREC.MULTIPLICATION],
      ];

      return choice(
        ...prec_table.map(([op, op_prec]) => {
          return prec.left(
            op_prec,
            seq(
              field("lhs", $.expression_no_assign),
              field("operation", op),
              field("rhs", $.expression_no_assign),
            ),
          );
        }),
      );
    },

    non_binary_expression: ($) => choice($.unary_expression, $.call_expression),

    assignment_expression: ($) =>
      prec.right(PREC.ASSIGNMENT, choice($.assignment, $.initializer)),

    assignment: ($) =>
      seq(
        field("lhs", seq($.IDENT, repeat($.field_access))),
        "=",
        field("rhs", $.expression),
      ),

    initializer: ($) => seq($.expression_no_assign, optional($.struct_init)),

    expression_no_assign: ($) =>
      choice($.binary_expression, $.non_binary_expression),

    unary_expression: ($) =>
      prec.right(
        PREC.UNARY,
        seq(
          field("operator", choice("!", "-")),
          field("rhs", $.non_binary_expression),
        ),
      ),

    call_expression: ($) =>
      seq(
        field("callee", $.primary),
        repeat(
          choice(
            seq("(", field("arguments", optional($.args_list)), ")"),
            seq($.field_access),
            seq("[", $.bracket_args, "]"),
          ),
        ),
      ),

    bracket_args: ($) => $.substitution_args,

    struct_init: ($) =>
      seq(
        "{",
        seq($.IDENT, ":", $.expression),
        repeat(seq(",", $.IDENT, ":", $.expression)),
        "}",
      ),

    substitution_args: ($) =>
      seq(
        $.IDENT,
        "=",
        $.expression,
        repeat(seq(",", $.IDENT, "=", $.expression)),
      ),

    primary: ($) =>
      choice(
        "true",
        "false",
        $.path,
        $.NUM,
        $.STRING,
        seq("(", $.expression, ")"),
      ),

    /// ================================================================
    ///  Utilities
    /// ================================================================

    param_list: ($) =>
      seq($.IDENT, ":", $._type, repeat(seq(",", $.IDENT, ":", $._type))),

    struct_fields: ($) =>
      seq(
        optional("pub"),
        $.IDENT,
        ":",
        $._type,
        repeat(seq(",", optional("pub"), $.IDENT, ":", $._type)),
      ),

    args_list: ($) => seq($.expression, repeat(seq(",", $.expression))),

    import_path: ($) =>
      seq(
        $.IDENT,
        optional(
          seq(
            "::",
            choice(
              $.import_path,
              "*",
              seq(
                "{",
                seq($.import_path, repeat(seq(",", $.import_path))),
                "}",
              ),
            ),
          ),
        ),
      ),

    path: ($) => seq($.IDENT, repeat(seq("::", $.IDENT))),

    field_access: ($) => seq(".", field("field", $._field_identifier)),

    /// ================================================================
    ///   Types
    /// ================================================================

    _type: ($) => choice($.bracket_type, $.native_type, $._type_identifier),

    native_type: ($) => choice(...nativeTypes),

    bracket_type: ($) => seq($.path, seq("<", $.IDENT, ">")),

    _type_identifier: ($) => alias($.path, $.type_identifier),

    /// ================================================================
    ///   Terminal token classes
    /// ================================================================

    IDENT: ($) => /[\p{XID_Start}_]\p{XID_Continue}*/,
    NUM: ($) => /0|[1-9]\d*(\.\d+)?/,
    STRING: ($) => /"[^"]*"/,

    comment: ($) =>
      token(
        choice(seq("//", /.*/), seq("/*", /[^*]*\*+([^/*][^*]*\*+)*/, "/")),
      ),

    _field_identifier: ($) => alias($.IDENT, $.field_identifier),
  },
});

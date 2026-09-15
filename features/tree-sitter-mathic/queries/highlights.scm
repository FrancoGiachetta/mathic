; Identifier

(IDENT) @variable
(field_identifier) @variable.parameter
(path_identifier) @variable
(type_identifier) @type
(native_type) @type.builtin

; Assume that uppercase identifiers are types
((IDENT) @type
  (#match? @type "^[A-Z]"))
((path_identifier) @type
  (#match? @type "^[A-Z]"))

; Assume that path tails are functions
(path
  (path_identifier)*
  (path
    tail: (path_identifier) @function))

; Functions

(func_decl
  name: (IDENT) @function)
(func_decl
  params: (param_list
    (IDENT) @variable.parameter))

(call_expression
  (primary
    (path
      tail: (path_identifier) @function))
  "(")

(substitution_args
  sym: (IDENT) @function)

; Struct

(struct_init
  (IDENT) @variable.parameter)
(struct_fields
  (IDENT) @variable.parameter)

; Keywords

[
  "imp"
  "df"
  "struct"
  "let"
  "sym"
  "for"
  "in"
  "while"
  "if"
  "else"
  "return"
  "or"
  "and"
] @keyword

; Literals

(STRING) @string
(NUM) @constant.numeric

[
  "true"
  "false"
] @constant.builtin

; Operators

[
  "+"
  "-"
  "*"
  "/"
  "="
  "=="
  "!"
  "!="
  ">"
  "<"
  ">="
  "<="
] @operator

(comment) @comment

[
  ","
  ";"
  ":"
  "::"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

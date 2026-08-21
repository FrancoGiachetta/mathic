; Identifier

(IDENT) @variable
(field_identifier) @property
(type_identifier) @type
(native_type) @type.builtin
(numeric_type) @type.builtin

; Functions

(func_decl
  name: (IDENT) @function)

(call_expression
  callee: (primary) @function)
(call_expression
  callee: [(primary) (field_access
    field: (field_identifier) @function.method)])

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
(NUM) @number

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
  ">="
  "<="
]

(comment) @comment

[
  ","
  "."
  ";"
  ":"
] @punctuation.delimiter

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
] @punctuation.bracket

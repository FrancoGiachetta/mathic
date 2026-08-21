; Identifier

(IDENT) @variable
(field_identifier) @property
(type_identifier) @type
(native_type) @type.builtin

; Functions

(func_decl
  name: (IDENT) @function
  params: (param_list
    (IDENT) @variable.parameter))
(func_decl
  name: (IDENT) @function)

(call_expression
  callee: (primary) @function)
(call_expression
  (field_access
    field: (field_identifier) @function.method))

(substitution_args
  sym: (IDENT) @function)

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

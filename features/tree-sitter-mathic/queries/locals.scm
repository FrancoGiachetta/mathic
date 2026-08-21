; Scopes

(func_decl) @local.scope
(for_stmt) @local.scope
(block) @local.scope

; Definitions

(func_decl
  name: (IDENT) @local.definition.function)

(struct_decl
  name: (IDENT) @local.definition.type)

(param_list
  (IDENT) @local.definition.parameter)

(var_decl
  name: (IDENT) @local.definition.var)

(sym_decl
  name: (IDENT) @local.definition.var)

(for_stmt
  . (IDENT) @local.definition.var)

; References

(assignment
  lhs: (IDENT) @local.reference)

(primary
  (IDENT) @local.reference)

%%
[0-9]+ "INTEGER"
\+ "ADD"
\* "MUL"
\( "LPAR"
\) "RPAR"
; ";"
= "ASSIGN"
let "LET" 
println "PRINT_LN" 
[a-zA-Z0-9_]+ "IDENTIFIER"
[\t\n ]+ ;
//[^\n]*?$ ;

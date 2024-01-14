%start StatementList
%%

StatementList -> Result<Vec<Node>, ()>:
    StatementList Statement { append($1.map_err(|_| ())?, $2.map_err(|_| ())?)  }
    | { Ok(vec![]) }
    ;

Statement -> Result<Node, ()>:
   ';' { Ok(Node::Empty{}) }
    | Expression ';' { $1 }
    | Builtins { $1 }
    ;
    
Expression -> Result<Node, ()>:
    AdditiveExpression { $1 }
    | PrimaryExpression 'ASSIGN' Expression {
        match $1.map_err(|_| ())? {
            Node::Id { value } => {
                Ok(Node::Assign { id: value, rhs: Box::new($3?) })
            },
            _ => Err(())
        }
    }
    | 'LET' PrimaryExpression 'ASSIGN' Expression {
        match $2.map_err(|_| ())? {
            Node::Id { value } => {
                Ok(Node::Declare { id: value, rhs: Some(Box::new($4?)) })
            },
            _ => Err(())
        }
    } 
    ;

AdditiveExpression -> Result<Node, ()>:
    MultiplicativeExpression { $1 }
    | AdditiveExpression 'ADD' MultiplicativeExpression { 
        Ok(Node::Add{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

MultiplicativeExpression -> Result<Node, ()>: 
    PrimaryExpression { $1 }
    | MultiplicativeExpression 'MUL' PrimaryExpression { 
      Ok(Node::Mul{ lhs: Box::new($1?), rhs: Box::new($3?) })
    }
    ;

PrimaryExpression -> Result<Node, ()>:
    'IDENTIFIER' { Ok(Node::Id { value: $lexer.span_str(($1.map_err(|_| ())?).span()).to_string() }) }
    |  'LPAR' Expression 'RPAR' { $2 }
    | 'INTEGER' { parse_int($lexer.span_str(($1.map_err(|_| ())?).span())) }
    ;

Builtins -> Result<Node, ()>:
    'PRINT_LN' 'LPAR' Expression 'RPAR' { Ok(Node::PrintLn{ rhs: Box::new($3?) }) };

%%
use crate::ast::Node;

fn append(mut lhs: Vec<Node>, rhs: Node ) -> Result<Vec<Node>, ()>{
    lhs.push(rhs);
    Ok(lhs)
}

fn parse_int(s: &str) -> Result<Node, ()> {
    match s.parse::<u64>() {
        Ok(n_val) => Ok(Node::Number{ value: n_val }),
        Err(_) => {
            eprintln!("{} cannot be represented as a u64", s);
            Err(())
        }
    }
}

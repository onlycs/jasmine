use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Variable {
    pub mutable: bool,
    pub ident: String,
    pub ty: Type,
    pub expr: Expression,
}

impl Parse for Variable {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut mutable = false;
        let mut ident = None;
        let mut ty = None;
        let mut expr = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::mut_kwd => mutable = true,
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::ty => ty = Some(Type::parse(rule)?),
                Rule::expr => expr = Some(Expression::parse(rule)?),
                _ => {}
            }
        }

        Some(Variable {
            mutable,
            ident: ident?,
            ty: ty?,
            expr: expr?,
        })
    }
}

impl Variable {
    pub fn rewrite(&self) -> String {
        format!(
            "{} {} {} = {}",
            if !self.mutable { "final" } else { "" },
            self.ty.rewrite(),
            rewrite_ident(&self.ident),
            self.expr.rewrite()
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub begin: i64,
    pub end: i64,
    pub inclusive: bool,
}

impl Parse for Range {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut begin = None;
        let mut end = None;
        let mut inclusive = false;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::int => {
                    if begin.is_none() {
                        begin = Some(rule.as_str().trim().parse::<i64>().ok()?);
                    } else {
                        end = Some(rule.as_str().trim().parse::<i64>().ok()?);
                    }
                }
                Rule::range_incl => inclusive = true,
                _ => {}
            }
        }

        Some(Self {
            begin: begin?,
            end: end?,
            inclusive,
        })
    }
}

impl Range {
    pub fn rewrite(&self) -> String {
        format!(
            "new Range({}, {}, {})",
            self.begin,
            self.end,
            if self.inclusive { "true" } else { "false" }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DefinitionType {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Vec<CharDecl>),
    Char(CharDecl),
    Array(Vec<Expression>),
    Struct(CreateStructure),
    Closure(Closure),
    Range(Range),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Definition {
    pub kind: DefinitionType,
}

impl Parse for Definition {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let Some(rule) = pair.into_inner().nth(0) else { return None };

        let mut kind = None;

        match rule.as_rule() {
            Rule::r#struct => kind = Some(DefinitionType::Struct(CreateStructure::parse(rule)?)),
            Rule::float => {
                let mut rule_str = rule.as_str();

                if rule_str.ends_with("f") {
                    rule_str = &rule_str[..rule_str.len() - 1];
                }

                kind = Some(DefinitionType::Float(rule_str.parse::<f64>().ok()?))
            }
            Rule::int => {
                let mut rule_str = rule.as_str().trim();

                if rule_str.ends_with("i") {
                    rule_str = &rule_str[..rule_str.len() - 1];
                }

                kind = Some(DefinitionType::Int(rule_str.parse::<i64>().ok()?))
            }
            Rule::bool => kind = Some(DefinitionType::Bool(rule.as_str().parse::<bool>().ok()?)),
            Rule::string => kind = Some(DefinitionType::String(CharDecl::parse_many(rule)?)),
            Rule::char => {
                let Some(char_decl) = rule.into_inner().next() else {return None};

                kind = Some(DefinitionType::Char(CharDecl::parse(char_decl)?))
            }
            Rule::array => {
                let mut exprs = vec![];

                for expr in rule.into_inner() {
                    match expr.as_rule() {
                        Rule::expr => {
                            exprs.push(Expression::parse(expr)?);
                        }
                        _ => {}
                    }
                }

                kind = Some(DefinitionType::Array(exprs))
            }
            Rule::closure => kind = Some(DefinitionType::Closure(Closure::parse(rule)?)),
            Rule::range => kind = Some(DefinitionType::Range(Range::parse(rule)?)),
            _ => {}
        }

        Some(Definition { kind: kind? })
    }
}

impl Definition {
    pub fn rewrite(&self) -> String {
        match &self.kind {
            DefinitionType::Bool(b) => b.to_string(),
            DefinitionType::Char(c) => c.rewrite(),
            DefinitionType::Float(f) => format!("((double) {})", f.to_string()),
            DefinitionType::String(s) => format!("\"{}\"", CharDecl::rewrite_many(s.clone(), "")),
            DefinitionType::Int(i) => i.to_string(),
            DefinitionType::Struct(def) => def.rewrite(),
            DefinitionType::Array(arr) => {
                format!("Vec.from({})", Expression::rewrite_many(arr.clone(), ", "))
            }
            DefinitionType::Closure(closure) => closure.rewrite(),
            DefinitionType::Range(range) => range.rewrite(),
        }
    }
}

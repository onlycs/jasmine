use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct StructDef {
    pub ident: String,
    pub fields: Vec<Arg>,
}

impl Parse for StructDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<StructDef> {
        let mut fields = vec![];
        let mut ident = None;

        for struct_part in pair.into_inner() {
            match struct_part.as_rule() {
                Rule::ident => {
                    ident = Some(struct_part.as_str().to_string());
                }
                Rule::define_arguments => {
                    let args = Arg::parse_many(struct_part)?;

                    fields = args;
                }
                _ => {}
            }
        }

        Some(StructDef {
            ident: ident?,
            fields,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplDef {
    pub ident: String,
    pub methods: Vec<ImplFnDef>,
}

impl Parse for ImplDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut methods = vec![];

        for impl_part in pair.into_inner() {
            match impl_part.as_rule() {
                Rule::ident => {
                    ident = Some(impl_part.as_str().to_string());
                }
                Rule::impl_fn_def => {
                    methods.push(ImplFnDef::parse(impl_part)?);
                }
                _ => {}
            }
        }

        Some(ImplDef {
            ident: ident?,
            methods,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImplFnType {
    Consume,
    Ref,
    MutRef,
    Static,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDef {
    pub ident: String,
    pub args: Vec<Arg>,
    pub body: Vec<InBlock>,
    pub returns: Option<Type>,
}

impl Parse for FnDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::define_arguments => {
                    args = Arg::parse_many(rule)?;
                }
                Rule::block => {
                    body = InBlock::parse_many(rule)?;
                }
                Rule::ty => {
                    returns = Some(Type::parse(rule)?);
                }
                _ => {}
            }
        }

        Some(FnDef {
            ident: ident?,
            args,
            body,
            returns,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplFnDef {
    pub ident: String,
    pub args: Vec<Arg>,
    pub body: Vec<InBlock>,
    pub returns: Option<Type>,
    pub self_type: ImplFnType,
}

impl Parse for ImplFnDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;
        let mut self_type = ImplFnType::Static;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::impl_define_arguments => {
                    for arg_rule in rule.into_inner() {
                        match arg_rule.as_rule() {
                            Rule::define_arguments => {
                                args = Arg::parse_many(arg_rule)?;
                            }
                            Rule::borrow_kwd => {
                                self_type = ImplFnType::Ref;
                            }
                            Rule::mut_kwd => {
                                self_type = ImplFnType::MutRef;
                            }
                            Rule::self_kwd if self_type == ImplFnType::Static => {
                                self_type = ImplFnType::Consume;
                            }
                            _ => {}
                        }
                    }
                }
                Rule::block => {
                    body = InBlock::parse_many(rule)?;
                }
                Rule::ty => {
                    returns = Some(Type::parse(rule)?);
                }
                _ => {}
            }
        }

        Some(ImplFnDef {
            ident: ident?,
            args,
            body,
            returns,
            self_type,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClosureDef {
    pub args: Vec<Arg>,
    pub body: Vec<InBlock>,
    pub returns: Option<Type>,
}

impl Parse for ClosureDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::define_arguments => {
                    args = Arg::parse_many(rule)?;
                }
                Rule::block => {
                    body = InBlock::parse_many(rule)?;
                }
                Rule::ty => {
                    returns = Some(Type::parse(rule)?);
                }
                _ => {}
            }
        }

        Some(ClosureDef {
            args,
            body,
            returns,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VarDef {
    pub mutable: bool,
    pub ident: String,
    pub ty: Type,
    pub expr: Expr,
}

impl Parse for VarDef {
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
                Rule::expr => expr = Some(Expr::parse(rule)?),
                _ => {}
            }
        }

        Some(VarDef {
            mutable,
            ident: ident?,
            ty: ty?,
            expr: expr?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhileDef {
    pub cond: Expr,
    pub body: Vec<InBlock>,
}

impl Parse for WhileDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut cond = None;
        let mut body = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => cond = Some(Expr::parse(rule)?),
                Rule::block => body = InBlock::parse_many(rule)?,
                _ => {}
            }
        }

        Some(WhileDef { cond: cond?, body })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForDef {
    pub arg: Arg,
    pub iter: Expr,
    pub body: Vec<InBlock>,
}

impl Parse for ForDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut arg = None;
        let mut iter = None;
        let mut body = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::define_argument => arg = Some(Arg::parse(rule)?),
                Rule::expr => iter = Some(Expr::parse(rule)?),
                Rule::block => body = InBlock::parse_many(rule)?,
                _ => {}
            }
        }

        Some(ForDef {
            arg: arg?,
            iter: iter?,
            body,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ElseIfDef {
    pub cond: Expr,
    pub body: Vec<InBlock>,
}

impl Parse for ElseIfDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut cond = None;
        let mut body = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => cond = Some(Expr::parse(rule)?),
                Rule::block => body = InBlock::parse_many(rule)?,
                _ => {}
            }
        }

        Some(ElseIfDef { cond: cond?, body })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfDef {
    pub cond: Expr,
    pub body: Vec<InBlock>,
    pub else_ifs: Vec<ElseIfDef>,
    pub else_body: Option<Vec<InBlock>>,
}

impl Parse for IfDef {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut cond = None;
        let mut body = vec![];
        let mut else_ifs = vec![];
        let mut else_body = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => cond = Some(Expr::parse(rule)?),
                Rule::block => body = InBlock::parse_many(rule)?,
                Rule::else_if_def => else_ifs.push(ElseIfDef::parse(rule)?),
                Rule::else_def => {
                    for rule in rule.into_inner() {
                        match rule.as_rule() {
                            Rule::block => else_body = Some(InBlock::parse_many(rule)?),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Some(IfDef {
            cond: cond?,
            body,
            else_ifs,
            else_body,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CharDecl {
    RawChar(char),
    EscapeChar(Escape),
}

impl Parse for CharDecl {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::raw_char => Some(CharDecl::RawChar(pair.as_str().chars().next()?)),
            Rule::escape => Some(CharDecl::EscapeChar(Escape::parse(pair)?)),
            _ => None,
        }
    }
}

impl ParseMany for CharDecl {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut chars = vec![];

        for char_decl in pair.into_inner() {
            chars.push(Self::parse(char_decl)?);
        }

        Some(chars)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Definition {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(Vec<CharDecl>),
    Char(CharDecl),
    Array(Vec<Definition>),
    StructDef(StructDef),
    Closure(ClosureDef),
}

impl Parse for Definition {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        if let Some(rule) = pair.into_inner().nth(0) {
            match rule.as_rule() {
                Rule::r#struct => Some(Definition::StructDef(StructDef::parse(rule)?)),
                Rule::float => {
                    let mut rule_str = rule.as_str();

                    if rule_str.ends_with("f") {
                        rule_str = &rule_str[..rule_str.len() - 1];
                    }

                    Some(Definition::Float(rule_str.parse::<f64>().ok()?))
                }
                Rule::int => {
                    let mut rule_str = rule.as_str();

                    if rule_str.ends_with("i") {
                        rule_str = &rule_str[..rule_str.len() - 1];
                    }

                    Some(Definition::Int(rule_str.parse::<i64>().ok()?))
                }
                Rule::bool => Some(Definition::Bool(rule.as_str().parse::<bool>().ok()?)),
                Rule::string => Some(Definition::String(CharDecl::parse_many(rule)?)),
                Rule::char => {
                    if let Some(char_decl) = rule.into_inner().next() {
                        Some(Definition::Char(CharDecl::parse(char_decl)?))
                    } else {
                        None
                    }
                }
                Rule::array => {
                    let mut defs = vec![];

                    for def in rule.into_inner() {
                        match def.as_rule() {
                            Rule::definition => {
                                defs.push(Definition::parse(def)?);
                            }
                            _ => {}
                        }
                    }

                    Some(Definition::Array(defs))
                }
                Rule::closure => Some(Definition::Closure(ClosureDef::parse(rule)?)),
                _ => None,
            }
        } else {
            None
        }
    }
}

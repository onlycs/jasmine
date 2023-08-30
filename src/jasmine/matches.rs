use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    pub ty_ident: String,
    pub variant_ident: String,
    pub data_ident: Option<String>,
    pub data_ident_ty: Option<Type>,
    pub body: Vec<BlockPart>,
}

impl Parse for MatchArm {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ty_ident = None;
        let mut variant_ident = None;
        let mut after_dblcln = false;

        let mut data_ident = None;
        let mut body = vec![];
        let mut ty = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident if !after_dblcln => ty_ident = Some(rule.as_str().to_string()),
                Rule::ident if after_dblcln && variant_ident.is_none() => {
                    variant_ident = Some(rule.as_str().to_string())
                }
                Rule::ident if after_dblcln && variant_ident.is_some() => {
                    data_ident = Some(rule.as_str().to_string())
                }
                Rule::block => body = BlockPart::parse_many(rule)?,
                Rule::dblcln => after_dblcln = true,
                Rule::ty => ty = Some(Type::parse(rule)?),
                _ => {}
            }
        }

        Some(MatchArm {
            ty_ident: ty_ident?,
            variant_ident: variant_ident?,
            data_ident,
            data_ident_ty: ty,
            body,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchStatement {
    pub expr: Expression,
    pub arms: Vec<MatchArm>,
    pub final_arm: Option<Vec<BlockPart>>,
}

impl Parse for MatchStatement {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut expr = None;
        let mut arms = vec![];
        let mut final_arm = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => expr = Some(Expression::parse(rule)?),
                Rule::match_arm => arms.push(MatchArm::parse(rule)?),
                Rule::block => final_arm = Some(BlockPart::parse_many(rule)?),
                _ => {}
            }
        }

        Some(MatchStatement {
            expr: expr?,
            arms,
            final_arm,
        })
    }
}

impl MatchStatement {
    pub fn rewrite(&self) -> String {
        let arms_iter = self.arms.iter();
        let mut rewritten = "".to_string();

        for arm in arms_iter {
            let data_sect = 'block: {
                let Some(ident) = &arm.data_ident else { break 'block String::from("") };
                let Some(ty) = &arm.data_ident_ty else { break 'block String::from("") };

                format!(
                    "{} {} = {}._getData_{}();\n",
                    ty.rewrite(),
                    rewrite_ident(ident),
                    self.expr.rewrite(),
                    rewrite_ident(&arm.variant_ident)
                )
            };

            rewritten.push_str(&format!(
                "else if ({}.is({}.{})) {{
					{}{}
				}} ",
                self.expr.rewrite(),
                rewrite_ident(&arm.ty_ident),
                format!("_{}", rewrite_ident(&arm.variant_ident)),
                data_sect,
                BlockPart::rewrite_many(arm.body.clone(), "\n")
            ));
        }

        rewritten.push_str(&format!(
            "else {{
				{}
			}} ",
            self.final_arm
                .as_ref()
                .map(|a| BlockPart::rewrite_many(a.to_vec(), "\n"))
                .unwrap_or(
                    "throw new RuntimeException(\"Not all match arms were covered in this statement\");".to_string()
                )
        ));

        rewritten.split("else").skip(1).join("else")
    }
}

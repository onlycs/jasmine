use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariant {
    pub ident: String,
    pub data: Option<Type>,
}

impl Parse for EnumVariant {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut data = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::ty => data = Some(Type::parse(rule)?),
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            data,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enumeration {
    pub ident: String,
    pub variants: Vec<EnumVariant>,

    pub generics: Option<GenericArguments>,
    pub where_clause: Option<Vec<WhereUnit>>,
}

impl Parse for Enumeration {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut variants = vec![];

        let mut generics = None;
        let mut where_clause = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::enum_variant => variants.push(EnumVariant::parse(rule)?),

                Rule::generic_args => {
                    generics = Some(GenericArguments::parse(rule)?);
                }
                Rule::where_clause => {
                    where_clause = Some(WhereUnit::parse_many(rule)?);
                }
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            variants,
            generics,
            where_clause,
        })
    }
}

impl Enumeration {
    pub fn rewrite_no_closing(&self) -> String {
        let generics = match &self.generics {
            Some(g) => g.rewrite(self.where_clause.as_ref()),
            None => "".to_string(),
        };

        let fulltype = format!("{}{}", rewrite_ident(&self.ident), generics);

        let mut rewritten = format!(
            "
			public static class {} {{
			",
            fulltype
        );

        for (idx, variant) in self.variants.iter().enumerate() {
            rewritten.push_str(&format!(
                "public static final int _{} = {};\n",
                variant.ident,
                idx + 1
            ));
        }

        rewritten.push_str("int currentVariant;\n");

        let mut variant_data = vec![];

        for variant in self.variants.iter() {
            if let Some(data) = &variant.data {
                rewritten.push_str(&format!(
                    "{} {}Data;\n",
                    data.rewrite(),
                    rewrite_ident(&variant.ident)
                ));
                variant_data.push((rewrite_ident(&variant.ident), data.clone()));
            }
        }

        rewritten.push_str(&format!(
            "
			private {}(int _currentVariant{}) {{
				this.currentVariant = _currentVariant;
				{}
			}}
			",
            self.ident,
            format!(
                ", {}",
                variant_data
                    .iter()
                    .map(|(ident, ty)| format!("{} _{}Data", ty.rewrite(), ident))
                    .join(", ")
            ),
            variant_data
                .iter()
                .map(|(ident, _)| format!("this.{}Data = _{}Data;", ident, ident))
                .join("\n"),
        ));

        for variant in self.variants.iter() {
            rewritten.push_str(&format!(
                "
				public static {generics} {fulltype} {}({}) {{
					return new {fulltype}({}, {});
				}}
				",
                variant.ident,
                match &variant.data {
                    Some(data) => format!("{} data", data.rewrite()),
                    None => "".to_string(),
                },
                format!("_{}", variant.ident),
                variant_data
                    .iter()
                    .map(|(ident, _)| {
                        if ident == &variant.ident {
                            "data".to_string()
                        } else {
                            "null".to_string()
                        }
                    })
                    .join(", ")
            ))
        }

        for (variant, data_ty) in variant_data {
            rewritten.push_str(&format!(
                "
				public {} _getData_{}() {{
					return {}Data;
				}}
				",
                data_ty.rewrite(),
                variant,
                variant
            ));
        }

        rewritten.push_str(
            "
			public boolean is(int variant) {
				return currentVariant == variant;
			}
			",
        );

        rewritten
    }
}

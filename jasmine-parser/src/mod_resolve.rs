use crate::prelude::*;

pub fn resolve(path: &mut PathBuf, ast: &mut File) -> Result<(), ParserError> {
    path.pop(); // remove the file, get the dir of the file

    for item in ast.items.iter_mut().filter_map(|item| match item {
        Item::Mod(item) if item.content.is_none() => Some(item),
        _ => None,
    }) {
        let ident = item.ident.to_string();
        trace!("Got mod item: {ident}. resolving to {ident}.jasmine or {ident}/mod.jasmine",);

        path.push(format!("{ident}.jasmine"));

        if !path.as_path().exists() {
            path.pop();
            path.push(&ident);
            path.push("mod.jasmine");
        }

        if !path.as_path().exists() {
            bail!(ParserError::UnresolvedModule(ident));
        }

        trace!("Found module file: {}", path.display());
        let parsed = crate::parse(path)?;

        if parsed.shebang.is_some() {
            bail!(ParserError::UnexpectedShebang);
        }

        if !parsed.attrs.is_empty() {
            bail!(ParserError::UnexpectedInnerAttribute);
        }

        item.content = Some((Default::default(), parsed.items));
    }

    Ok(())
}

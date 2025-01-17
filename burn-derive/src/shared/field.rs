use super::attribute::AttributeAnalyzer;
use proc_macro2::Ident;
use syn::{Field, Type, TypePath};

#[derive(Clone)]
pub struct FieldTypeAnalyzer {
    pub field: Field,
}

impl FieldTypeAnalyzer {
    pub fn new(field: Field) -> Self {
        FieldTypeAnalyzer { field }
    }

    pub fn ident(&self) -> Ident {
        self.field.ident.clone().unwrap()
    }

    pub fn is_of_type(&self, paths: &[&str]) -> bool {
        match &self.field.ty {
            syn::Type::Path(path) => {
                let name = Self::path_name(path);
                paths.contains(&name.as_str())
            }
            _ => false,
        }
    }

    #[allow(dead_code)]
    pub fn first_generic_field(&self) -> TypePath {
        let err = || panic!("Field {} as no generic", self.field.ident.clone().unwrap());
        match &self.field.ty {
            syn::Type::Path(path) => Self::path_generic_argument(path),
            _ => err(),
        }
    }
    pub fn path_generic_argument(path: &TypePath) -> TypePath {
        let segment = path.path.segments.last().unwrap();
        let err = || panic!("Path segment {} has no generic", segment.ident.clone(),);
        match &segment.arguments {
            syn::PathArguments::None => err(),
            syn::PathArguments::AngleBracketed(param) => {
                let first_param = param.args.first().unwrap();

                if let syn::GenericArgument::Type(Type::Path(path)) = first_param {
                    path.clone()
                } else {
                    err()
                }
            }
            syn::PathArguments::Parenthesized(_) => err(),
        }
    }

    fn path_name(path: &TypePath) -> String {
        let length = path.path.segments.len();
        let mut name = String::new();
        for (i, segment) in path.path.segments.iter().enumerate() {
            if i == length - 1 {
                name += segment.ident.to_string().as_str();
            } else {
                let tmp = segment.ident.to_string() + "::";
                name += tmp.as_str();
            }
        }
        name
    }

    pub fn attributes(&self) -> impl Iterator<Item = AttributeAnalyzer> {
        self.field
            .attrs
            .clone()
            .into_iter()
            .map(AttributeAnalyzer::new)
    }

    pub fn is_param(&self) -> bool {
        self.is_of_type(&["Param", "burn::Param"])
    }
}

pub(crate) fn parse_fields(ast: &syn::DeriveInput) -> Vec<Field> {
    let mut fields = Vec::new();

    match &ast.data {
        syn::Data::Struct(struct_data) => {
            for field in struct_data.fields.iter() {
                fields.push(field.clone());
            }
        }
        syn::Data::Enum(_) => panic!("Only struct can be derived"),
        syn::Data::Union(_) => panic!("Only struct cna be derived"),
    };
    fields
}

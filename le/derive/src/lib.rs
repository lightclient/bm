#![recursion_limit="128"]

extern crate proc_macro;

use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse2, Generics, DeriveInput};
use syn::spanned::Spanned;
use deriving::{struct_fields, has_attribute};

use proc_macro::TokenStream;

#[proc_macro_derive(IntoTree, attributes(bm))]
pub fn into_tree_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut impl_generics = parse2::<Generics>(quote! { #impl_generics }).expect("Parse generic failed");
    impl_generics.params.push(parse2(quote! { DB }).expect("Parse generic failed"));

    let where_fields = struct_fields(&input.data)
	.expect("Not supported derive type")
        .iter()
        .map(|f| {
	    let ty = &f.ty;

            if has_attribute("bm", &f.attrs, "compact") {
                quote_spanned! {
		    f.span() => for<'a> bm_le::CompactRef<'a, #ty>: bm_le::IntoTree<DB>
	        }
            } else {
	        quote_spanned! {
		    f.span() => #ty: bm_le::IntoTree<DB>
	        }
            }
	});

    let fields = struct_fields(&input.data)
        .expect("Not supported derive type")
        .iter()
        .map(|f| {
            let name = &f.ident;

            if has_attribute("bm", &f.attrs, "compact") {
                quote_spanned! { f.span() => {
                    vector.push(bm_le::IntoTree::into_tree(&bm_le::CompactRef(&self.#name), db)?);
                } }
            } else {
                quote_spanned! { f.span() => {
                    vector.push(bm_le::IntoTree::into_tree(&self.#name, db)?);
                } }
            }
        });

    let expanded = quote! {
        impl #impl_generics bm_le::IntoTree<DB> for #name #ty_generics where
            #where_clause
            #(#where_fields),*,
            DB: bm_le::Backend<Intermediate=bm_le::Intermediate, End=bm_le::End>
        {
            fn into_tree(&self, db: &mut DB) -> Result<bm_le::ValueOf<DB>, bm_le::Error<DB::Error>> {
                let mut vector = Vec::new();
                #(#fields)*
                bm_le::utils::vector_tree(&vector, db, None)
            }
        }
    };

    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(FromTree, attributes(bm))]
pub fn from_tree_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let mut impl_generics = parse2::<Generics>(quote! { #impl_generics }).expect("Parse generic failed");
    impl_generics.params.push(parse2(quote! { DB }).expect("Parse generic failed"));

    let where_fields = struct_fields(&input.data)
	.expect("Not supported derive type")
        .iter()
        .map(|f| {
	    let ty = &f.ty;

            if has_attribute("bm", &f.attrs, "compact") {
                quote_spanned! {
		    f.span() => bm_le::Compact<#ty>: bm_le::FromTree<DB>
	        }
            } else {
	        quote_spanned! {
		    f.span() => #ty: bm_le::FromTree<DB>
	        }
            }
	});

    let fields = struct_fields(&input.data)
        .expect("Not supported derive type")
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let name = &f.ident;
            let ty = &f.ty;

            if has_attribute("bm", &f.attrs, "compact") {
                quote_spanned! {
                    f.span() =>
                        #name: <bm_le::Compact<#ty> as bm_le::FromTree<_>>::from_tree(
                            &vector.get(db, #i)?,
                            db,
                        )?.0,
                }
            } else {
                quote_spanned! {
                    f.span() =>
                        #name: bm_le::FromTree::from_tree(
                            &vector.get(db, #i)?,
                            db,
                        )?,
                }
            }
        });

    let fields_count = struct_fields(&input.data)
        .expect("Not supported derive type")
        .iter()
        .count();

    let expanded =
        quote! {
            impl #impl_generics bm_le::FromTree<DB> for #name #ty_generics where
                #where_clause
                #(#where_fields),*,
                DB: bm_le::Backend<Intermediate=bm_le::Intermediate, End=bm_le::End>
            {
                fn from_tree(
                    root: &bm_le::ValueOf<DB>,
                    db: &DB,
                ) -> Result<Self, bm_le::Error<DB::Error>> {
                    use bm_le::Leak;

                    let vector = bm_le::DanglingVector::<DB>::from_leaked(
                        (root.clone(), #fields_count, None)
                    );

                    Ok(Self {
                        #(#fields)*
                    })
                }
            }
        };

    proc_macro::TokenStream::from(expanded)
}

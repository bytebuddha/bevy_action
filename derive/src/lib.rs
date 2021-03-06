use syn::*;
use quote::quote;
use find_crate::find_crate;
use proc_macro::TokenStream;
use proc_macro2::Span;

#[derive(Debug)]
struct CrateLocations {
    serde: String,
    bevy: Option<String>,
    bevy_reflect: Option<String>,
    bevy_utils: Option<String>
}

impl CrateLocations {

    fn find() -> CrateLocations {
        let locations = CrateLocations {
            serde: find_crate(|x| x.contains("serde"))
                .map(|x|x.name)
                .expect("Failed to find serde crate"),
            bevy: find_crate(|x| x.contains("bevy"))
                .map(|x|x.name)
                .ok(),
            bevy_reflect: find_crate(|x| x.contains("bevy_reflect"))
                .map(|x|x.name)
                .ok(),
            bevy_utils: find_crate(|x| x.contains("bevy_utils"))
                .map(|x|x.name)
                .ok()
        };
        if locations.bevy.is_none() {
            if locations.bevy_reflect.is_none() && locations.bevy_utils.is_none() {
                panic!("Requires either bevy or bevy_reflect and bevy_utils crate.");
            }
        }
        locations
    }

    fn hashmap(&self) -> proc_macro2::TokenStream {
        match &self.bevy {
            Some(bevy) => {
                let ident = syn::Ident::new(&bevy, Span::call_site());
                quote! { #ident::utils::HashMap }
            },
            None => {
                let reflect = self.bevy_utils.as_ref().unwrap();
                let ident = syn::Ident::new(&reflect, Span::call_site());
                quote! { #ident::HashMap }
            }
        }
    }

    fn typeuuid(&self) -> proc_macro2::TokenStream {
        match &self.bevy {
            Some(bevy)=> {
                 let ident = syn::Ident::new(&bevy, Span::call_site());
                quote! { #ident::reflect::TypeUuid }
            },
            None => {
                let reflect = self.bevy_reflect.as_ref().unwrap();
                let reflect = syn::Ident::new(&reflect, Span::call_site());
                quote! { #reflect::TypeUuid }
            }
        }
    }


}

#[proc_macro_derive(ConfigActions, attributes(Pressed, JustPressed, Axis))]
pub fn derive_macro(_input: TokenStream) -> TokenStream {
    let output = quote! {};
    output.into()
}

#[proc_macro_attribute]
pub fn config_actions(attr: TokenStream, item: TokenStream) -> TokenStream {
     let locations = CrateLocations::find();
    println!("{:?}", locations);
     let item_enum = syn::parse_macro_input!(item as syn::ItemEnum);
     let serde_crate = syn::Ident::new(&locations.serde, Span::call_site());
     let hmap = locations.hashmap();
     let typeuuid = locations.typeuuid();

     let ident = &item_enum.ident;

     let default_impl = get_default_implementation(&item_enum, hmap.clone());

     match attribute_inputs(parse_macro_input!(attr as AttributeArgs)) {
         Err(err) => return err,
         Ok((file, uuid)) => {
             let output = quote! {
                 #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, ConfigActions)]
                 #[derive(#serde_crate::Serialize, #serde_crate::Deserialize, #typeuuid)]
                 #[uuid = #uuid]
                 #item_enum

                 impl bevy_actions::ConfigActions for #ident {
                     const PATH: &'static str = #file;

                     fn default_bindings() -> #hmap<bevy_actions::Event, Self> {
                         #default_impl
                     }
                 }
             };
             println!("{}", output);
             output.into()
         }
     }

}

fn attribute_inputs(args: Vec<NestedMeta>) -> std::result::Result<(Lit, Lit), TokenStream> {
    let mut output = (
        syn::parse::<Lit>(quote! {""}.into()).unwrap(),
        syn::parse::<Lit>(quote! {""}.into()).unwrap(),
    );
    if args.len() != 2 {
        return Err(quote! { compile_error!("Proc macro: `config_actions` expected 2 arguments") }.into());
    }
    for arg in args.iter() {
        match arg {
            syn::NestedMeta::Meta(meta) => match meta {
                syn::Meta::NameValue(value) => {
                    if "file" == value.path.get_ident().unwrap().to_string() {
                        output.0 = value.lit.clone();
                    }
                    if "uuid" == value.path.get_ident().unwrap().to_string() {
                        output.1 = value.lit.clone();
                    }
                },
                _ => return Err(quote!{compile_error!("Uxpected Named::Meta value")}.into())
            },
            _ => return Err(quote!{compile_error!("Expected NestedMeta Value")}.into())
        }
    }
    Ok(output)
}

fn get_default_implementation(input: &syn::ItemEnum, hmap: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    use std::str::FromStr;
    let mut variants = vec![];
    let enum_ident = &input.ident;
    for item in input.variants.iter() {
        let enum_item = &item.ident;
        for attr in item.attrs.iter() {
            let mut attr_str = attr.tokens.to_string();
            attr_str = attr_str[1..attr_str.len() - 1].to_string();
            match syn::parse::<syn::MetaList>(TokenStream::from_str(&attr_str).unwrap()) {
                Err(err) => {
                    println!("{:?}", err);
                    return quote!{compile_error!("Failed to parse config_actions attribute arguments")}.into()
                },
                Ok(attrs) => {
                    let mut event_attr: Vec<proc_macro2::TokenStream> = Default::default();
                    match attr.path.get_ident().unwrap().to_string().as_ref() {
                        "Pressed" => {
                            match attrs.path.get_ident().unwrap().to_string().as_ref() {
                                "Keyboard" => {
                                    let code = attrs.nested.first().unwrap();
                                    event_attr.push(quote! {
                                        Pressed(bevy_actions::Button::Keyboard(KeyCode::#code))
                                    });
                                },
                                "Mouse" => {
                                    let code = attrs.nested.first().unwrap();
                                    event_attr.push(quote! {
                                        Pressed(bevy_actions::Button::Mouse(MouseButton::#code))
                                    });
                                },
                                "Gamepad" => {
                                    let gamepad = attrs.nested.first().unwrap();
                                    let axis = attrs.nested.iter().skip(1).next().unwrap();
                                    let bevy_crate = match find_crate(|s| s == "bevy") {
                                        Ok(_) => quote! {bevy::input},
                                        Err(_) => quote ! {bevy_input}
                                    };
                                    event_attr.push(quote! {
                                        Pressed(bevy_actions::Button::Gamepad(#gamepad, #bevy_crate::gamepad::GamepadButtonType::#axis))
                                    });
                                },
                                _ => {}
                            }
                        },
                        "JustPressed" => {
                            match attrs.path.get_ident().unwrap().to_string().as_ref() {
                                "Keyboard" => {
                                    let code = attrs.nested.first().unwrap();
                                    event_attr.push(quote! {
                                        JustPressed(bevy_actions::Button::Keyboard(KeyCode::#code))
                                    });
                                },
                                "Mouse" => {
                                    let code = attrs.nested.first().unwrap();
                                    event_attr.push(quote! {
                                        JustPressed(bevy_actions::Button::Mouse(MouseButton::#code))
                                    });
                                },
                                "Gamepad" => {
                                    let gamepad = attrs.nested.first().unwrap();
                                    let axis = attrs.nested.iter().skip(1).next().unwrap();
                                    let bevy_crate = match find_crate(|s| s == "bevy") {
                                        Ok(_) => quote! {bevy::input},
                                        Err(_) => quote ! {bevy_input}
                                    };
                                    event_attr.push(quote! {
                                        JustPressed(bevy_actions::Button::Gamepad(#gamepad, #bevy_crate::gamepad::GamepadButtonType::#axis))
                                    });
                                },
                                _ => {}
                            }
                        },
                        "Axis" => {
                            match attrs.path.get_ident().unwrap().to_string().as_ref() {
                                "Mouse" => {
                                    let code = attrs.nested.first().unwrap();
                                    event_attr.push(quote! {
                                        Axis(bevy_actions::Axis::Mouse(MouseAxis::#code))
                                    });
                                },
                                "Gamepad" => {
                                    let gamepad = attrs.nested.first().unwrap();
                                    let axis = attrs.nested.iter().skip(1).next().unwrap();
                                    let bevy_crate = match find_crate(|s| s == "bevy") {
                                        Ok(_) => quote! {bevy::input},
                                        Err(_) => quote ! {bevy_input}
                                    };
                                    event_attr.push(quote! {
                                        Axis(bevy_actions::Axis::Gamepad(#gamepad, #bevy_crate::gamepad::GamepadAxisType::#axis))
                                    });
                                },
                                _ =>{}
                            }
                        },
                        _ => {}
                    }
                    variants.push(quote! {
                        #(map.insert(bevy_actions::Event::#event_attr, #enum_ident::#enum_item);)*
                    });
                }
            }
        }
    }
    if variants.is_empty() {
        let output = quote! { #hmap::default() };
        output.into()
    } else {
        let output = quote! {
            let mut map = #hmap::default();
            #(#variants)*
            map
        };
        output.into()
    }
}

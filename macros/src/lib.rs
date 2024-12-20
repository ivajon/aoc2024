use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{parse::Parse, parse_macro_input, Token};

struct Event {
    year: Year,
    day: Day,
    task: Task,
    cookies: String,
}

struct Year(usize);
struct Day(usize);
struct Task(usize);

struct LeaderBoard {
    code: Option<String>,
    cookie: String,
}

impl Parse for Year {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.peek(syn::Ident);
        if ident {
            let id: syn::Ident = input.parse()?;
            if id.to_string().as_str() != "year" {
                return Err(syn::Error::new_spanned(ident, "Expected year"));
            }
            let _: syn::Token![=] = input.parse()?;
        }

        let val: syn::LitInt = input.parse()?;
        Ok(Self(val.base10_parse()?))
    }
}
impl Parse for Day {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.peek(syn::Ident);
        if ident {
            let id: syn::Ident = input.parse()?;
            if id.to_string().as_str() != "day" {
                return Err(syn::Error::new_spanned(ident, "Expected year"));
            }
            let _: syn::Token![=] = input.parse()?;
        }

        let val: syn::LitInt = input.parse()?;
        Ok(Self(val.base10_parse()?))
    }
}
impl Parse for Task {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident = input.peek(syn::Ident);
        if ident {
            let id: syn::Ident = input.parse()?;
            if id.to_string().as_str() != "task" {
                return Err(syn::Error::new_spanned(ident, "Expected year"));
            }
            let _: syn::Token![=] = input.parse()?;
        }

        let val: syn::LitInt = input.parse()?;
        Ok(Self(val.base10_parse()?))
    }
}
impl Parse for Event {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let year = input.parse()?;
        let _: Token![,] = input.parse()?;
        let day = input.parse()?;
        let _: Token![,] = input.parse()?;
        let task = input.parse()?;
        let _: Token![,] = input.parse()?;
        let cookies: syn::LitStr = input.parse()?;

        let cookies = format!("{}", cookies.to_token_stream()).replace("\"", "");
        println!("Cookie {cookies}");
        let cookies = std::fs::read_to_string(cookies).expect("Could not read cookie file.");

        Ok(Self {
            year,
            day,
            task,
            cookies,
        })
    }
}
impl quote::ToTokens for Year {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let val = self.0;
        tokens.extend(quote::quote! {#val});
    }
}
impl quote::ToTokens for Day {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let val = self.0;
        tokens.extend(quote::quote! {#val});
    }
}
impl quote::ToTokens for Task {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let val = self.0;
        tokens.extend(quote::quote! {#val});
    }
}

#[proc_macro_attribute]
pub fn aoc(event: TokenStream, item: TokenStream) -> TokenStream {
    let event: Event = parse_macro_input!(event);
    println!("Parsed event input :)");

    let ic = item.clone();
    let r#fn: syn::ItemFn = parse_macro_input!(ic);
    let item = proc_macro2::TokenStream::from(item.clone());
    println!("Parsed function input:)");
    let (year, day, task, cookie) = (event.year.0, event.day.0, event.task.0, event.cookies);
    let name = r#fn.sig.ident;
    let get_url = format!("https://adventofcode.com/{year}/day/{day}/input");
    let set_url = format!("https://adventofcode.com/{year}/day/{day}/answer");
    let fn_name: proc_macro2::TokenStream = format!("aoc_{year}_{day}_{task}").parse().unwrap();
    let cache_name: String = format!("/tmp/aoc_{year}_{day}_{task}_input");

    let task_solved: String = format!("/tmp/aoc_{year}_{day}_{task}_solved");
    let cache_name: String = format!("/tmp/aoc_{year}_{day}_{task}_input");

    let cookie = cookie.replace("\n", "");
    let output = quote::quote! {
        #item
        async fn #fn_name() -> Result<(), Box<dyn std::error::Error>> {

            use std::io::Write;

            let cookies = reqwest::cookie::Jar::default();
            cookies.add_cookie_str(#cookie,&"https://adventofcode.com".parse()?);
            cookies.add_cookie_str(#cookie,&"https://advent.fly.dev".parse()?);
            let client = reqwest::ClientBuilder::new()
                .cookie_store(true)
                .cookie_provider(std::sync::Arc::new(cookies))
                .build()?;

            let data_request = client
                .request(Method::GET, #get_url)
                .header("Cookie", &format!("session={}",#cookie))
                .build()
               .unwrap();

            let cache_file = std::path::Path::new(#cache_name);



            let data = if !cache_file.exists() {
                println!("No cache exists, getting data from remote");
                let mut cache = std::fs::File::options().write(true).read(true).create(true).append(false).open(cache_file)?;
                let resp = client.execute(data_request).await?.text().await?;
                cache.write_all(&resp.clone().into_bytes())?;
                resp
            } else {
                println!("Cache exists, re-using old input.");
                std::fs::read_to_string(cache_file)?
            };

            let result = #name(data);
            println!("Result : {result}");
            let file_handle = std::path::Path::new(#task_solved);
            if !file_handle.exists() {
                let mut data = std::collections::HashMap::new();
                data.insert("level",#task.to_string());
                data.insert("annwer",result);
                let result_request = client
                    .request(Method::POST, #set_url)
                    //.body(format!("level={}&answer={}",#task,result))
                    .header("Accept", "text/plain")
                    //.header("Content-Type", "application/x-www-form-urlencoded")
                    .header("Cookie", &format!("session={}",#cookie)).json(&data)
                    .build()
                    .unwrap();
                let resp2 = client.execute(result_request).await?;
                match resp2.status().is_success() {
                    true => {
                        let text = resp2.text().await?;
                        if text.contains("That's the right answer") {
                            println!("Accepted :)");
                            eprintln!("Text:\n{text}");
                            std::fs::File::create(file_handle).unwrap();
                        }
                        else {
                            eprintln!("Answer not accepted :(");
                            eprintln!("Text:\n{text}");

                        }
                    },
                    false => {
                        let text = resp2.text().await?;
                        eprintln!("Not accepted : {text}");
                    }
                }
            }
            Ok(())
        }

    };
    println!("output :\n{}", output);
    output.into()
}

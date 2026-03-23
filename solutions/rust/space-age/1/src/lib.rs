// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

// use proc_macro::TokenStream;
// use quote::quote;
// use syn::{parse_macro_input, DeriveInput};

// use std::collections::HashMap;

#[derive(Debug)]
pub struct Duration{
    seconds: u64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration{seconds: s}
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

macro_rules! impl_planet {
    ($name:ident, $factor:expr) => {
        impl Planet for $name {
            fn years_during(d: &Duration) -> f64 {
                let earth_year_seconds: f64 = 31557600.0 * $factor;
                (d.seconds as f64) / earth_year_seconds
            }
        }
    };
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);

// #[proc_macro_derive(YEAR)]
// pub fn year_derive(input: TokenStream) -> TokenStream {
//     // 如果解析失败，这里会自动抛出编译错误
//     let input = parse_macro_input!(input as DeriveInput);

//     // 参数
//     let data = [
//         ("Mercury", 0.2408467),
//         ("Venus", 0.61519726),
//         ("Earth", 1.0),
//         ("Mars", 1.8808158),
//         ("Jupiter", 11.862615),
//         ("Saturn", 29.447498),
//         ("Uranus", 84.016846),
//         ("Neptune", 164.79132),
//     ];

//     let planet_map: HashMap<&str, f64> = data.into_iter().collect();

//     // 2. 提取结构体
//     let name = input.ident;
//     let name_str = name.to_string(); // 转为字符串用于查表

//     let factor = planet_map.get(&name_str[..]).unwrap();

//     // 3. 构建输出的代码
//     // #name 会被替换成上面提取的标识符
//     let expanded = quote! {
//         impl Planet for #name {
//             fn years_during(d: &Duration) -> f64 {
//                 let earth_year: f64 = 365.25 * 24. * 3600. * #factor;
//                 (d.seconds as f64) / earth_year
//             }
//         }
//     };

//     // 4. 将生成的代码转回 TokenStream 并返回
//     TokenStream::from(expanded)
// }

// #[derive(YEAR)] pub struct Mercury;
// #[derive(YEAR)] pub struct Venus;
// #[derive(YEAR)] pub struct Earth;
// #[derive(YEAR)] pub struct Mars;
// #[derive(YEAR)] pub struct Jupiter;
// #[derive(YEAR)] pub struct Saturn;
// #[derive(YEAR)] pub struct Uranus;
// #[derive(YEAR)] pub struct Neptune;

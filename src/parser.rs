use crate::gears::{CGear, EGear, Gear, NGear};

peg::parser! {
  pub grammar gearbox_parser() for str {
    rule _ = ([' ' | '\n' | '\t'] / ";" [c if c != '\n']* "\n")*

    rule del() = (_ "," _)

    rule num() -> u32
      =  n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    rule str() -> String
      =  "\"" s:("\\" s:[_] { vec![s] } / s:[c if !c.is_control() && c != '"']*) "\"" { s.iter().collect() }

    rule symbols_compact() -> Vec<String>
      =  "\"" s:("\\" s:[_] { s } / s:[c if !c.is_control() && c != '"'] { s })* "\"" { s.iter().map(|c|c.to_string()).collect() }

    rule symbols() -> Vec<String>
      =  "{" _ s:str()++del() _ "}" { s }

    rule parrarel() -> Vec<Box<dyn Gear>>
      =  "[" _ p:gear()**del() _ "]" { p }

    rule ngear() -> Box<NGear>
      = "g" _ n:num() _ p:parrarel()? _ c:gear()? { Box::new(NGear {
        n,
        child: c,
        parrarel: p.unwrap_or_default()
      }) }

    rule cgear() -> Box<CGear>
      = "c" _ n:num() _ s:(s:symbols()/s:symbols_compact()) _ l:("l" l:str() {l})? _ c:gear()? { Box::new(CGear {
        n,
        child: c,
        label: l,
        symbols: s
      }) }

    rule egear() -> Box<EGear>
      = "e" _ n:num() { Box::new(EGear {
        n,
      }) }

    rule gear() -> Box<dyn Gear>
      = g:ngear()/g:cgear()/g:egear() { g }

    pub rule gear_w() -> Box<dyn Gear>
      = _ g:gear() _ { g }
  }
}

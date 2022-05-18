use crate::gears::{CGear, EGear, Gear, NGear};

peg::parser! {
  pub grammar gearbox_parser() for str {
    /// Whitespace
    rule _w() = [' ' | '\n' | '\t']*

    /// Delimiter
    rule del() = (_w() "," _w())

    rule num() -> u32
      =  n:$(['0'..='9']+) {? n.parse().or(Err("u32")) }

    rule str() -> String
      =  "\"" s:("\\" s:[_] { vec![s] } / s:[c if !c.is_control() && c != '"']*) "\"" { s.iter().collect() }

    rule symbols_compact() -> Vec<String>
      =  "\""  s:("\\" s:[_] { vec![s] } / s:[c if !c.is_control() && c != '"']*) "\"" { s.iter().map(|c|c.to_string()).collect() }

    rule symbols() -> Vec<String>
      =  "{" _w() s:str()++del() _w() "}" { s }

    rule parrarel() -> Vec<Box<dyn Gear>>
      =  "[" _w() p:gear()**del() _w() "]" { p }

    rule ngear() -> Box<NGear>
      = "g" _w() n:num() _w() p:parrarel()? _w() c:gear()? { Box::new(NGear {
        n,
        child: c,
        parrarel: p.unwrap_or_default()
      }) }

    rule cgear() -> Box<CGear>
      = "c" _w() n:num() _w() s:(s:symbols()/s:symbols_compact()) _w() l:("l" l:str() {l})? _w() c:gear()? { Box::new(CGear {
        n,
        child: c,
        label: l,
        symbols: s
      }) }

    rule egear() -> Box<EGear>
      = "e" _w() n:num() { Box::new(EGear {
        n,
      }) }

    rule gear() -> Box<dyn Gear>
      = g:ngear()/g:cgear()/g:egear() { g }

    pub rule gear_w() -> Box<dyn Gear>
      = _w() g:gear() _w() { g }
  }
}

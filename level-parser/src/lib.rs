use std::{io, ops::Deref, collections::HashMap};
use svg::node::{element::tag, Value};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LevelParserError {
    #[error("SVG parser error")]
    SvgParseError(#[from] io::Error),
    #[error("Error parsing float")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Wrong arguments: {0}")]
    WrongArguments(String),
}

#[derive(Debug)]
pub enum Barrier {
    Rect(f32, f32, f32, f32, f32), // x, y, width, height, rotation
    Circle(f32, f32, f32),    // x, y, radius
}

#[derive(Debug)]
pub struct Level {
    pub barrier: Vec<Barrier>,
    pub spawns: HashMap<String, Vec<(f32, f32)>>,
    pub size: (f32, f32),
}

pub fn parse_transform(transform: &Value) -> f32 {
    let transform = transform.to_string();
    let regex = regex::Regex::new(r".*rotate\((?P<rotation>.*)\).*").unwrap();
    if let Some(caps) = regex.captures(&transform) {
        if let Ok(rotation) = &caps["rotation"].parse() {
            *rotation
        } else {
            0.0
        }
    } else {
        0.0
    }
}

pub fn parse_level_from_svg(file_content: &str) -> std::result::Result<Level, LevelParserError> {
    let mut parsing_colliders = false;
    let mut parsing_spawns = false;
    let mut g_depth = 0;
    let mut barrier = Vec::new();
    let mut spawns = HashMap::new();
    let mut last_text_position: (f32, f32) = (0.0, 0.0);
    let mut size = (0.0, 0.0);
    for event in svg::read(file_content)? {
        //if let svg::parser::Event::Tag(tagname, tag_type, _) = &event {
        //    println!("tagname: {}, tag_type: {:?}", tagname, tag_type);
        //}
        match event {
            svg::parser::Event::Tag("svg", tag::Type::Start, attributes) => {
                let width: f32 = attributes
                    .get("width")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No width coordinate in svg".into())
                    })?
                    .parse()?;
                let height: f32 = attributes
                    .get("height")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No height coordinate in svg".into())
                    })?
                    .parse()?;
                size = (width, height);
            }
            svg::parser::Event::Tag("g", tag::Type::Start, attributes) => {
                if attributes
                    .get("inkscape:groupmode")
                    .map(|attr| attr.deref())
                    == Some("layer")
                    && attributes.get("inkscape:label").map(|attr| attr.deref()) == Some("collider")
                {
                    parsing_colliders = true;
                    g_depth = 1;
                    println!("Found collider layer");
                }
                else if attributes
                    .get("inkscape:groupmode")
                    .map(|attr| attr.deref())
                    == Some("layer")
                    && attributes.get("inkscape:label").map(|attr| attr.deref()) == Some("spawns")
                {
                    parsing_spawns = true;
                    g_depth = 1;
                    println!("Found spawns layer");
                } else {
                    g_depth += 1;
                }
            }
            svg::parser::Event::Tag("g", tag::Type::End, _) => {
                g_depth -= 1;
                if g_depth == 0 {
                    parsing_colliders = false;
                    parsing_spawns = false;
                    println!("Layer parsing done");
                }
            }
            svg::parser::Event::Tag("rect", tag::Type::Empty, attributes) if parsing_colliders => {
                let x: f32 = attributes
                    .get("x")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No x coordinate in rect".into())
                    })?
                    .parse()?;
                let y: f32 = attributes
                    .get("y")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No y coordinate in rect".into())
                    })?
                    .parse()?;
                let width: f32 = attributes
                    .get("width")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No width coordinate in rect".into())
                    })?
                    .parse()?;
                let height: f32 = attributes
                    .get("height")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No height coordinate in rect".into())
                    })?
                    .parse()?;
                let rotation = attributes
                    .get("transform")
                    .map(parse_transform)
                    .unwrap_or(0.0);
                barrier.push(Barrier::Rect(x, y, width, height, rotation));
            }
            svg::parser::Event::Tag("circle", tag::Type::Empty, attributes)
                if parsing_colliders =>
            {
                let x: f32 = attributes
                    .get("cx")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No cx coordinate in circle".into())
                    })?
                    .parse()?;
                let y: f32 = attributes
                    .get("cy")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No cy coordinate in circle".into())
                    })?
                    .parse()?;
                let radius: f32 = attributes
                    .get("r")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No r coordinate in circle".into())
                    })?
                    .parse()?;
                barrier.push(Barrier::Circle(x, y, radius));
            }
            svg::parser::Event::Tag("text", tag::Type::Start, attributes)
                if parsing_spawns =>
            {
                let x: f32 = attributes
                    .get("x")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No x coordinate in text".into())
                    })?
                    .parse()?;
                let y: f32 = attributes
                    .get("y")
                    .ok_or_else(|| {
                        LevelParserError::WrongArguments("No y coordinate in text".into())
                    })?
                    .parse()?;
                last_text_position = (x, y);
            }
            svg::parser::Event::Text(text) if parsing_spawns => {
                let entries = spawns.entry(text.to_string()).or_insert_with(|| Vec::new());
                entries.push(last_text_position);
            }          
            _ => {

            }
        }
    }
    Ok(Level { barrier, spawns, size })
}

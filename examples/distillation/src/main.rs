use engcon::*;

#[derive(Debug, Clone, Default, Copy, PartialEq, Validatable)]
pub struct DistillationColumn {
    #[validate_value(x >= 3)]
    pub trays: i32,

    #[validate_value(x < trays, x >= 1)]
    pub feed_place: i32,

    #[validate_value(x > 0.0)]
    pub reflux_ratio: f32,

    #[validate_value(x > 0.0, x < 1.0)]
    pub distiliate_to_feed_ratio: f32,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dcs = [
        DistillationColumn::default(),
        DistillationColumn {
            trays: 20,
            feed_place: 25,
            ..Default::default()
        },
        DistillationColumn {
            trays: 20,
            feed_place: 10,
            distiliate_to_feed_ratio: 0.9,
            reflux_ratio: 1.5,
        },
        DistillationColumn {
            trays: 40,
            feed_place: 20,
            distiliate_to_feed_ratio: 1.,
            reflux_ratio: 0.75,
        },
    ];

    for dc in &dcs {
        println!("{dc:?}");
        match dc.validate() {
            Ok(_) => println!("is valid!"),
            Err(err) => println!("not valid: {}", err),
        }
    }
    Ok(())
}

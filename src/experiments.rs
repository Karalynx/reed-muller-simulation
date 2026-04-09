
use std::time::Instant;
use rand::{random, SeedableRng};

use charming::{
    component::{Axis, Grid, VisualMap}, 
    datatype::{CompositeValue, DataFrame},
    series::{Heatmap, Line},
    element::{
        AreaStyle, AxisType, Emphasis, 
        ItemStyle, Label, Orient,
        SplitArea, Tooltip
    }, 
    Chart, 
    ImageFormat,
    ImageRenderer,
    df
};
use rand_chacha::ChaCha12Rng;

use crate::{
    channel::{channel::{Channel, Send}, 
    channel_errors::ChannelErrors, 
    reed_muller::Hadamards, 
    split_vector::SplitVector}, 
    math::{matrix::GenMatrix, vector::BinaryVector}, 
    parameters::{Muller, Probability}
};

const MESSAGES: [&str; 5] = [
    "fAX].mDrMnGk}j#!B[r{,WBG}P&PyTf/QkcRCpi:PKY*#7vEB&",
    "+aMzjE5}$}L.bmhX[GBc!}:[3RN6W)e:[@KP]P)R(&DcthTkx6",
    "*-+gZzS(6etjzLx!8}=@m7xz6Y,MdE@rzH.yB.zf;W%Uq/L&kL",
    "T-_in?]iQR}5U)Bjv%U2P5qUQfad:J5An+m5U,AJN:4m.}y{U5",
    "U%GyH81,{M3Z.BcQRxc,i}pPfYvxx$cRff.CdNh[#XbpNUpF&(",
];

fn error_rate_experiment(muller_iters: Muller) -> Chart {
    let mut acc_message_length = 0;
    let messages = MESSAGES.map(|str| {
        let vec = BinaryVector::from_bits(str).unwrap();
        acc_message_length += vec.cols();
        vec
    });

    let gen_matrices = (1 .. muller_iters.get() + 1).map(|m| {
        GenMatrix::new(unsafe { Muller::new_unchecked(m) })
    }).collect::<Box<[GenMatrix]>>();

    let rng = ChaCha12Rng::from_seed(random());
    let mut error_rates = vec![[0.0f32; 9]; muller_iters.get() as usize + 1].into_boxed_slice();
    for m in 1 .. muller_iters.get() as usize + 1 {
        let muller = Muller::new(m as u8).unwrap();
        let hadamards = Hadamards::new(muller);

        for p in 1 .. 10 {
            let prob = Probability::new((p as f32) * 0.1).unwrap();
            let mut channel = Channel { p: prob, rng: rng.clone() };

            for vec in messages.iter() {
                let mut split_vec = SplitVector::new(&vec, muller);
                split_vec.encode(&gen_matrices[m - 1]);
                channel.send_multiple(&mut split_vec);
                split_vec.decode(&hadamards);

                let restored = split_vec.restore();
                let errors = ChannelErrors::from_vectors(vec, &restored);

                error_rates[m - 1][p - 1] += errors.get().len() as f32;
            }
            
            error_rates[m - 1][p - 1] /= acc_message_length as f32;
        }
    }

    let mut data: Vec<DataFrame> = Vec::<DataFrame>::new();
    for (muller, avgs) in error_rates.iter().enumerate() {
        for (prob, avg) in avgs.iter().enumerate() {
            data.push(df!(
                muller as i32, 
                prob as i32, 
                CompositeValue::from(format!("{:.2}", *avg))
            ));
        }
    }

    Chart::new()
        .tooltip(Tooltip::new().position("top"))
        .grid(Grid::new().height("50%").top("10%"))
        .x_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data((1 .. muller_iters.get() + 1).map(|m| {
                    m.to_string()
                }).collect())
                .split_area(SplitArea::new().show(true))
                .name("m"),
        )
        .y_axis(
            Axis::new()
                .type_(AxisType::Category)
                .data((1 .. 10).map(|p| {
                    format!("{:.1}", p as f32 * 0.1)
                }).collect())
                .split_area(SplitArea::new().show(true))
                .name("p"),
        )
        .visual_map(
            VisualMap::new()
                .min(0)
                .max(1)
                .calculable(true)
                .orient(Orient::Horizontal)
                .left("center")
                .bottom("15%"),
        )
        .series(
            Heatmap::new()
                .name("Punch Card")
                .label(Label::new().show(true))
                .emphasis(
                    Emphasis::new().item_style(
                        ItemStyle::new()
                            .shadow_blur(10)
                            .shadow_color("rgba(0, 0, 0, 0.5)"),
                    ),
                )
                .data(data),
        )
}

fn decoding_time_experiment(muller_iters: Muller, p: Probability) -> Chart {
    
    let rng = ChaCha12Rng::from_seed(random());
    let mut corrupted_vectors = (1 .. muller_iters.get() + 1).map(|m| {
        let muller = Muller::new(m).unwrap();
        let gen_matrix = GenMatrix::new(muller);

        let mut channel = Channel { p, rng: rng.clone() };
        MESSAGES.map(|str| {
            let mut split_vector = SplitVector::new(&BinaryVector::from_bits(str).unwrap(), muller);
            split_vector.encode(&gen_matrix);
            channel.send_multiple(&mut split_vector);
            split_vector
        }).into()
    }).collect::<Box<[Box<[SplitVector]>]>>();

    let durations = corrupted_vectors.iter_mut().enumerate().map(|(m, vectors)| {
        let hadamards = Hadamards::new(Muller::new(m as u8 + 1).unwrap());

        let now = Instant::now();
        for vector in vectors.iter_mut() {
            vector.decode(&hadamards);
        }
        
        now.elapsed().as_secs_f32()
    }).collect::<Vec<f32>>();

    Chart::new()
    .x_axis(
        Axis::new()
            .type_(AxisType::Category)
            .boundary_gap(false)
            .data((1 .. muller_iters.get() + 1).map(|i| {
                    i.to_string()
                }).collect()
            )
            .name("m"),
    )
    .y_axis(
        Axis::new()
        .type_(AxisType::Value)
        .name("Laikas (s)")
    )
    .series(
        Line::new()
            .area_style(AreaStyle::new())
            .data(durations),
    )
}

pub fn run_experiments(muller1: Muller, muller2: Muller) {
    let charts = [
        (error_rate_experiment(muller1), "error_rates"),
        (decoding_time_experiment(muller2, Probability::new(0.2).unwrap()), "decoding_time")
    ];

    let mut renderer = ImageRenderer::new(500, 500);
    for (chart, file_name) in charts.iter() {
        match renderer.save_format(ImageFormat::Png, &chart, format!("data/{file_name}.png")) {
            Ok(_) => {},
            Err(err) => eprintln!("{err}"),
        }
    }
}
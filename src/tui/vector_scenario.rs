
use crate::channel::channel_errors::ChannelErrors;
use super::shared::*;

/// Removes all whitespaces from `src`.
/// 
/// Returns a new [`String`] with removed whitespaces.
fn sanitise_str(src: &str) -> String {
    src.chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
}

fn send_vector(terminal: &mut Cursive, channel: &mut Channel, gen_matrix: &GenMatrix) {
    
    let text = terminal.call_on_name("initial_vector", |view: &mut VectorTextAreaV2| {
        sanitise_str(view.get_content())
    }).unwrap();

    if text.len() != gen_matrix.rows() as usize {
        error_popup(terminal, 
            "Įvesties klaida", 
            format!("Vektoriaus ilgis turi būti lygus {}", gen_matrix.rows())
        );
        return;
    }

    let vector = match BinaryVector::from_binary_bytes(text.as_str()) {
        Some(x) => x,
        None => { // Should never happen
            error_popup(
                terminal, 
                "Įvesties klaida", 
                "Vektoriuje turi būti tik 0 ir 1!"
            );
            return;
        },
    };

    // Dimensions were already checked before
    let mut encoded_vector = rm_encode(&vector, &gen_matrix);
    let encoded_str =  encoded_vector.to_string();

    let errors = channel.send_single(&mut encoded_vector);
    let corrupted_str = encoded_vector.to_string();

    terminal.call_on_name("encoded_vector", |view: &mut TextView| {
        view.set_content(&encoded_str);
    }).unwrap();

    terminal.call_on_name("corrupted_vector_data", |view: &mut VectorTextAreaV2| {
        view.set_content(&corrupted_str);
    }).unwrap();

    terminal.call_on_name("corrupted_vector_info", |view: &mut TextView| {
        view.set_content(errors.to_string());
    }).unwrap();
}

fn decode_vector(terminal: &mut Cursive, hadamards: &Hadamards) {

    let encoded_str_len = terminal.call_on_name("encoded_vector", |view: &mut TextView| {
       view.get_content().source().len()
    }).unwrap();

    let corrupted_str = terminal.call_on_name("corrupted_vector_data", |view: &mut VectorTextAreaV2| {
        sanitise_str(view.get_content())
    }).unwrap();

    if encoded_str_len != corrupted_str.len() {
        error_popup(terminal, 
            "Įvesties klaida", 
            format!("Iškraipyto vektoriaus ilgis turi būti lygus {encoded_str_len}")
        );
        return;
    }

    if encoded_str_len == 0 {
        error_popup(terminal, 
            "Loginė klaida", 
            format!("Vektorius nebuvo išsiųstas kanalu!")
        );
        return;        
    }

    let corrupted_vector = match BinaryVector::from_binary_bytes(corrupted_str) {
        Some(x) => x,
        None => { // Should never happen
            error_popup(terminal, 
                "Loginė klaida", 
                "Vektoriuje turi būti tik 0 ir 1!"
            );
            return;
        },
    };

    let (decoded_vector, weights) = rm_decode(&corrupted_vector, &hadamards);
    let decoded_str = decoded_vector.to_string();

    terminal.call_on_name("decoded_vector_data", |view: &mut TextView| {
        view.set_content(decoded_str);
    });

    terminal.call_on_name("decoded_vector_info", |view: &mut TextView| {
        view.set_content(format!("Svoriai: {:?}", weights.inner()));
    });
}

pub fn vector_scenario_screen(terminal: &mut Cursive, gen_matrix: Arc<Mutex<GenMatrix>>, hadamards: Arc<Mutex<Hadamards>>, channel: Arc<Mutex<Channel>>) {
    
    let initial_vector_view = Dialog::around(ListView::new()
        .child(&format!("Vektorius iš {} elem.", gen_matrix.lock().unwrap().rows()), 
            VectorTextAreaV2::new()
                .with_name("initial_vector")
                .max_height(5)
        )
    ).button("Siųsti", move |term: &mut Cursive| {
        send_vector(term, &mut channel.lock().unwrap(), &gen_matrix.lock().unwrap());
    });

    let sending_vector_view = Dialog::around(ListView::new()
        .child("Užkoduotas vektorius", TextView::empty()
            .with_name("encoded_vector")
            .scrollable()
            .min_height(1)
            .max_height(5)
        )

        .delimiter()

        .child("Iškraipytas vektorius", VectorTextAreaV2::new()
            .on_edit(|term, text, _| {
                
                let encoded_str = term.call_on_name("encoded_vector", |view: &mut TextView| {
                    view.get_content()
                }).unwrap();

                term.call_on_name("corrupted_vector_info", |view: &mut TextView| {
                    view.set_content(
                        // Recalculate error indexes
                        ChannelErrors::from_bytes(encoded_str.source(), text).to_string()
                    );
                });

            })
            .with_name("corrupted_vector_data")
            .max_height(5)
        )

        .child("", TextView::new("0 klaidų: []")
            .with_name("corrupted_vector_info")
            .scrollable()
            .max_height(3)
        )
    ).button("Dekoduoti", move |term| {
        decode_vector(term, &hadamards.lock().unwrap());
    });

    let decoded_vector_view = Dialog::around(ListView::new()
        .child("Dekoduotas vektorius", TextView::empty()
            .with_name("decoded_vector_data")
            .scrollable()
            .min_height(1)
            .max_height(5)
        )
        .child("", TextView::new("Svoriai: []")
            .with_name("decoded_vector_info")
            .scrollable()
            .max_height(3)
        )
    );


    let vector_scenario_view = LinearLayout::vertical()
        .child(initial_vector_view)
        .child(sending_vector_view)
        .child(decoded_vector_view)
        .full_screen()
        .scrollable();

    terminal.add_layer(Dialog::around(vector_scenario_view)
        .title("Vektoriaus siuntimas kanalu")
        .button("Grįžti", |term| {
            term.pop_layer();
        })
    );
}

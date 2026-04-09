
use crate::channel::split_vector::SplitVector;
use super::shared::*;

pub fn text_scenario_screen(terminal: &mut Cursive, gen_matrix: Arc<Mutex<GenMatrix>>, hadamards: Arc<Mutex<Hadamards>>, channel: Arc<Mutex<Channel>>) {
    let channel2 = Arc::new(Mutex::new(channel.lock().unwrap().clone()));

    let initial_text_view = Dialog::around(ListView::new()
        .child("Tekstas", TextAreaV2::new()
            .with_name("initial_text")
            .max_height(5)
        )
    ).button("Siųsti", move |term: &mut Cursive| {
        let gm = gen_matrix.lock().unwrap();

        let bits = term.call_on_name("initial_text", |view: &mut TextAreaV2| {
            BinaryVector::from_bits(view.get_content())
        }).unwrap();

        let mut raw_split_vector = match bits {
            Some(x) => SplitVector::new(&x, gm.muller()),
            None => {
                error_popup(term, 
                    "Įvesties klaida", 
                    format!("Teksto laukas tuščias")
                );
                return;
            },
        };

        let mut encoded_split_vector = raw_split_vector.clone();
        encoded_split_vector.encode(&gm);

        channel.lock().unwrap().send_multiple(&mut raw_split_vector);
        
        channel2.lock().unwrap().send_multiple(&mut encoded_split_vector);
        encoded_split_vector.decode(&hadamards.lock().unwrap());

        let raw_bytes = raw_split_vector.to_bytes();
        let decoded_bytes = encoded_split_vector.to_bytes();
        
        term.call_on_name("restored_raw_text", |view: &mut TextView| {
            view.set_content(String::from_utf8_lossy(&raw_bytes));
        });

        term.call_on_name("restored_decoded_text", |view: &mut TextView| {
            view.set_content(String::from_utf8_lossy(&decoded_bytes));
        });
    });

    let decoded_text_view = Dialog::around(ListView::new()
        .child("Atgamintas neužkoduotas tekstas", TextView::empty()
            .with_name("restored_raw_text")
            .scrollable()
            .min_height(1)
            .max_height(5))

        .delimiter()

        .child("Atgamintas užkoduotas tekstas", TextView::empty()
            .with_name("restored_decoded_text")
            .scrollable()
            .min_height(1)
            .max_height(5))
        );

    let text_scenario_view = LinearLayout::vertical()
        .child(initial_text_view)
        .child(decoded_text_view)
        .full_screen()
        .scrollable();
    
    terminal.add_layer(Dialog::around(text_scenario_view)
        .title("Teksto siuntimas kanalu")
        .button("Grįžti", |term| {
            term.pop_layer();
        })
    );
}
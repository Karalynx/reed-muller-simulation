
use super::{
    image_scenario::image_scenario_screen,
    text_scenario::text_scenario_screen, 
    vector_scenario::vector_scenario_screen,
    shared::*
};

fn create_settings_view(default_params: &Parameters, params_ref: Arc<Mutex<Parameters>>) -> ListView {
    let (params_ref2, params_ref3) = (
        params_ref.clone(), 
        params_ref.clone()
    );
    
    let param_m_view = UnsignedEditView::new()
        .content(default_params.m.to_string())
        .on_submit(move |term, text| {
            let mut params = params_ref.lock().unwrap();
            params.m = match text.parse::<Muller>() {
                Ok(x) => x,
                Err(err) => {
                    error_popup(
                        term, 
                        "Įvesties klaida", 
                        format!("Klaidingai įvedėte parametrą m: {err}")
                    );
                    return;
                },
            }
        })
        .with_name("param_m");
    
    let param_p_view = FloatEditView::new()
        .content(default_params.p.to_string())
        .on_submit(move |term, text| {
            let mut params = params_ref2.lock().unwrap();
            params.p = match text.parse::<Probability>() {
                Ok(x) => x,
                Err(err) => {
                    error_popup(
                        term, 
                        "Įvesties klaida", 
                        format!("Klaidingai įvedėte klaidos tikimybę: {err}")
                    );
                    return;
                },
            }
        })
        .with_name("param_p");

    let param_scenario_view = SelectView::<Scenario>::new()
        .item("Siunčiamas vektorius", Scenario::SendingVector)
        .item("Siunčiamas tekstas", Scenario::SendingText)
        .item("Siunčiamas paveikslėlis", Scenario::SendingImage)
        .selected(default_params.scenario as usize)
        .popup()
        .on_submit(move |_, scenario| {
            let mut params = params_ref3.lock().unwrap();
            params.scenario = *scenario;
        })
        .with_name("param_scenario");

    return ListView::new()
        .child("RM(1, m)", param_m_view)
        .child("Klaidos tikimybė", param_p_view)
        .child("Scenarijus", param_scenario_view)
}

pub fn setup_screen(terminal: &mut Cursive) {
    let default_params = Parameters::default();
    let params_ref = Arc::new(
        Mutex::new(default_params)
    );
    let gen_matrix_ref = Arc::new(
        Mutex::new(GenMatrix::new(default_params.m))
    );
    let hadamards_ref = Arc::new(
        Mutex::new(Hadamards::new(default_params.m))
    );
    
    let settings_view = create_settings_view(&default_params, params_ref.clone());
    terminal.add_layer(ScrollView::new(Dialog::around(settings_view)
        .title("Parametrų nustatymas")
        .button("Tęsti", move |term| {
            let mut new_params = Parameters::new();
            
            let m_res = term.call_on_name("param_m", |view: &mut UnsignedEditView| {
                view.get_content().parse::<Muller>()
            }).unwrap();
            
            new_params.m = match m_res {
                Ok(x) => x,
                Err(err) => {
                    error_popup(
                        term, 
                        "Įvesties klaida", 
                        format!("Klaidingai įvedėte parametrą m: {err}"));
                    return;
                },
            };

            let p_res = term.call_on_name("param_p", |view: &mut FloatEditView| {
                view.get_content().parse::<Probability>()
            }).unwrap();

            new_params.p = match p_res{
                Ok(x) => x,
                Err(err) => {
                    error_popup(
                        term, 
                        "Įvesties klaida", 
                        format!("Klaidingai įvedėte klaidos tikimybę: {err}"));
                    return;
                },
            };

            let scenario = term.call_on_name("param_scenario", |view: &mut SelectView<Scenario>| {
                view.selection()
            }).unwrap();

            if let Some(x) = scenario {
                new_params.scenario = *x;

                let mut params = params_ref.lock().unwrap();
                
                // Ensure new matrices are created only when `m` has changed
                if new_params.m != params.m {
                    let mut matrix = gen_matrix_ref.lock().unwrap();
                    *matrix = GenMatrix::new(new_params.m);

                    let mut hadamards = hadamards_ref.lock().unwrap();
                    *hadamards = Hadamards::new(new_params.m);
                }

                *params = new_params;
                let func = match new_params.scenario {
                    Scenario::SendingVector => vector_scenario_screen,
                    Scenario::SendingText => text_scenario_screen,
                    Scenario::SendingImage => image_scenario_screen
                };
                func(term, 
                    gen_matrix_ref.clone(), 
                    hadamards_ref.clone(), 
                    Arc::new(Mutex::new(Channel::new(params.p)))
                );
            }
            // Should never happen
            else {
                error_popup(term, 
                    "Įvesties klaida", 
                    "Nepasirinkote scenarijaus!"
                );
                return;
            }
        })
    ));
}